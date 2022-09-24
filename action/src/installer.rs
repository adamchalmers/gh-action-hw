use crate::{
    config::{Config, OsPlat},
    AResult,
};
use reqwest::Url;

const DOWNLOAD_URL: &str = "https://github.com/protocolbuffers/protobuf/releases/download";

/// Download protoc.
pub async fn get_protoc(cfg: Config) -> AResult<(Url, bytes::Bytes)> {
    let url = get_url_for_tag(&cfg.tag, cfg.os_plat, &cfg.os_arch)?;
    let client = cfg.new_client()?;
    let protoc_resp = client.get(url.clone()).send().await?;
    let bytes = protoc_resp.bytes().await?;
    Ok((url, bytes))
}

/// Find the GitHub URL for downloding the given tag on the given OS arch/platform.
fn get_url_for_tag(tag: &str, os_plat: OsPlat, os_arch: &str) -> AResult<Url> {
    let v: String = if tag.starts_with('v') {
        tag.chars().skip(1).collect()
    } else {
        tag.to_owned()
    };
    let file_name = match os_plat {
        OsPlat::Windows => {
            format!("protoc-{v}-win{os_arch}.zip")
        }
        OsPlat::Darwin => {
            format!("protoc-{v}-osx-{os_arch}.zip")
        }
        OsPlat::Linux => format!("protoc-{v}-linux-{os_arch}.zip"),
    };
    let url = Url::parse(&format!("{DOWNLOAD_URL}/{tag}/{file_name}"))?;
    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download() {
        let cfg = Config {
            tag: "v3.20.2".to_owned(),
            include_pre_releases: false,
            repo_token: Default::default(),
            os_arch: "x86_64".to_owned(),
            os_plat: OsPlat::Linux,
        };
        let (url, bytes) = get_protoc(cfg).await.unwrap();
        assert_eq!(url.as_str(), "https://github.com/protocolbuffers/protobuf/releases/download/v3.20.2/protoc-3.20.2-linux-x86_64.zip");
        assert_eq!(bytes.len(), 1_715_083);
    }
}
