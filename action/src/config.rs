use crate::AResult;
use anyhow::anyhow;
use semver::VersionReq;
use std::{env, str::FromStr};

const WINDOWS: [&str; 2] = ["32", "64"];
const MACOS: [&str; 2] = ["aarch_64", "x86_64"];
const LINUX: [&str; 5] = ["aarch_64", "x86_64", "ppcle_64", "s390_64", "x86_32"];

#[derive(Clone, Debug)]
pub struct Config {
    pub requested_version: VersionReq,
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
        let requested_version = args
            .next()
            .ok_or_else(|| anyhow!("Missing arg 1, version"))?;
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
            requested_version: parse_version_req(requested_version)?,
            include_pre_releases,
            repo_token,
            os_arch,
            os_plat,
        })
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

fn parse_version_req(requested_version: String) -> AResult<VersionReq> {
    // Strip leading 'v'
    let requested_version = if requested_version.starts_with('v') {
        requested_version.chars().skip(1).collect()
    } else {
        requested_version
    };
    // Strip trailing 'x'
    let requested_version = if requested_version.ends_with('x') {
        requested_version
            .chars()
            .take(requested_version.len() - 1)
            .collect()
    } else {
        requested_version
    };

    let version_req = VersionReq::parse(&format!("^{requested_version}"))?;
    Ok(version_req)
}
