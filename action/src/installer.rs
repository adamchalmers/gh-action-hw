use crate::{
    config::{Config, OsPlat},
    AResult,
};
use anyhow::anyhow;
use regex::Regex;
use reqwest::{header::HeaderMap, Client};
use semver::{Version, VersionReq};
use serde::Deserialize;
use std::{io::Write, path::Path, str::FromStr};

const RELEASE_URL: &str = "https://api.github.com/repos/protocolbuffers/protobuf/releases";
const DOWNLOAD_URL: &str = "https://github.com/protocolbuffers/protobuf/releases/download";

pub async fn get_protoc(cfg: Config, write_to: &Path) -> AResult<Version> {
    // Make a HTTP client.
    let mut headers = HeaderMap::new();
    headers.append(
        reqwest::header::USER_AGENT,
        "adamchalmers-setup-protoc".parse()?,
    );
    if !cfg.repo_token.is_empty() {
        headers.append(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", cfg.repo_token).parse()?,
        );
    }
    let client = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()?;

    let version = compute_version(cfg.requested_version, cfg.include_pre_releases, &client).await?;
    println!("Getting version {version}");
    let file_name = get_file_name(&version, cfg.os_plat, &cfg.os_arch);
    let protoc_resp = client
        .get(format!("{DOWNLOAD_URL}/{version}/{file_name}"))
        .send()
        .await?;
    let mut file = std::fs::File::create(write_to)?;
    let bytes = protoc_resp.bytes().await?;
    file.write_all(&bytes)?;
    Ok(version)
}

pub async fn compute_version(
    requested_version: VersionReq,
    include_pre_releases: bool,
    client: &Client,
) -> AResult<semver::Version> {
    let all_versions = fetch_versions(include_pre_releases, client).await?;
    let selected_version = all_versions
        .iter()
        .filter(|v| requested_version.matches(v))
        .max();
    match selected_version {
        None => Err(anyhow!("could not find a version which matched {requested_version}. Available versions were {all_versions:?}")),
        Some(v) => Ok(v.to_owned()),
    }
}

/// Retrieve a list of versions scraping tags from the Github API
pub async fn fetch_versions(
    include_pre_releases: bool,
    client: &Client,
) -> AResult<Vec<semver::Version>> {
    let mut tags: Vec<ProtocRelease> = Default::default();
    for page_num in 1.. {
        let resp = client
            .get(format!("{RELEASE_URL}?page={page_num}"))
            .send()
            .await?;
        let releases: Releases = resp.json().await.unwrap_or_default();
        let next_page = releases.result;
        if next_page.is_empty() {
            break;
        } else {
            tags.extend(next_page);
        }
    }

    let tag_regex = Regex::new(r#"v\d+\.[\w\.]+"#)?;
    let versions = tags
        .into_iter()
        .filter(|tag| tag_regex.is_match(&tag.tag_name))
        .filter(|tag| include_pre_releases || !tag.prerelease)
        .map(|tag| tag.tag_name.replace('v', ""))
        .filter_map(|v| match semver::Version::from_str(&v) {
            Ok(v) => Some(v),
            Err(e) => {
                eprintln!("ignoring {v} because {e}");
                None
            }
        })
        .collect();
    Ok(versions)
}

#[derive(Deserialize, Default)]
struct Releases {
    result: Vec<ProtocRelease>,
}

#[derive(Deserialize)]
struct ProtocRelease {
    tag_name: String,
    prerelease: bool,
}

fn get_file_name(v: &Version, os_plat: OsPlat, os_arch: &str) -> String {
    match os_plat {
        OsPlat::Windows => {
            format!("protoc-{v}-win{os_arch}.zip")
        }
        OsPlat::Darwin => {
            format!("protoc-{v}-osx-{os_arch}.zip")
        }
        OsPlat::Linux => format!("protoc-{v}-linux-{os_arch}.zip"),
    }
}
