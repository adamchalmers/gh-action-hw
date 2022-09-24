use anyhow::anyhow;
use reqwest::header::HeaderMap;
use std::{env, str::FromStr};

use crate::AResult;

const WINDOWS: [&str; 2] = ["32", "64"];
const MACOS: [&str; 2] = ["aarch_64", "x86_64"];
const LINUX: [&str; 5] = ["aarch_64", "x86_64", "ppcle_64", "s390_64", "x86_32"];
const MISSING_TAG: &str = "Missing arg 1, the tagged version e.g. v3.20.2 or v21.6";

#[derive(Clone, Debug)]
pub struct Config {
    pub tag: String,
    pub include_pre_releases: bool,
    pub repo_token: String,
    pub os_arch: String,
    pub os_plat: OsPlat,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum OsPlat {
    Windows,
    Darwin,
    Linux,
}

impl FromStr for OsPlat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "windows" => Ok(Self::Windows),
            "darwin" => Ok(Self::Darwin),
            "linux" => Ok(Self::Linux),
            other => Err(anyhow!(
                "unsupported os {other}, supported options are 'windows', 'darwin' or 'linux'"
            )),
        }
    }
}

impl Config {
    pub fn new_from_env() -> anyhow::Result<Self> {
        let mut args = env::args();
        let tag = args.next().ok_or_else(|| anyhow!(MISSING_TAG))?;
        if tag.is_empty() {
            return Err(anyhow!(MISSING_TAG))?;
        }

        let include_pre_releases = args
            .next()
            .ok_or_else(|| anyhow!("Missing arg 2 include-pre-releases"))?
            == "true";

        let repo_token = args.next().ok_or_else(|| {
            anyhow!("Missing arg 3, repo-token, if you don't want this, just use the empty string.")
        })?;

        let os_arch = args
            .next()
            .ok_or_else(|| anyhow!("Missing arg 4, os-arch"))?;

        let os_plat = args
            .next()
            .ok_or_else(|| anyhow!("Missing arg 5, os-plat"))?
            .parse()?;

        validate(&os_arch, os_plat);
        Ok(Self {
            tag,
            include_pre_releases,
            repo_token,
            os_arch,
            os_plat,
        })
    }

    pub fn new_client(&self) -> AResult<reqwest::Client> {
        // Make a HTTP client.
        let mut headers = HeaderMap::new();
        headers.append(
            reqwest::header::USER_AGENT,
            "adamchalmers/gh-action-hw".parse()?,
        );
        headers.append(
            reqwest::header::ACCEPT,
            "application/vnd.github+json".parse()?,
        );
        if !self.repo_token.is_empty() {
            headers.append(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {}", self.repo_token).parse()?,
            );
        }
        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?;
        Ok(client)
    }
}

fn validate(os_arch: &str, os_plat: OsPlat) {
    match os_plat {
        OsPlat::Windows => {
            if !WINDOWS.contains(&os_arch) {
                eprintln!(
                    "warning: {os_arch} is an unusual architecture, I suggest one of {WINDOWS:?}"
                );
            }
        }
        OsPlat::Darwin => {
            if !MACOS.contains(&os_arch) {
                eprintln!(
                    "warning: {os_arch} is an unusual architecture, I suggest one of {MACOS:?}"
                );
            }
        }
        OsPlat::Linux => {
            if !LINUX.contains(&os_arch) {
                eprintln!(
                    "warning: {os_arch} is an unusual architecture, I suggest one of {LINUX:?}"
                );
            }
        }
    }
}
