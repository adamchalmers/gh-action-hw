use std::process::exit;
mod config;
mod installer;
mod unused;

type AResult<T> = anyhow::Result<T>;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let cfg = match config::Config::new_from_env() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        }
    };

    let write_to = std::path::Path::new("/output/protoc");
    match installer::get_protoc(cfg, write_to).await {
        Ok(version) => {
            println!("::set-output name=protoc-version::{version}");
            println!("::set-output name=path::{}", write_to.display());
        }
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        }
    }
    println!("Success");
}
