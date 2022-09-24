use std::io::Write;
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
    let (url, bytes) = match installer::get_protoc(cfg).await {
        Ok(x) => x,
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        }
    };
    // Finish downloading the file, and write it to the filesystem.
    let mut file = match std::fs::File::create(write_to) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        }
    };
    if let Err(err) = file.write_all(&bytes) {
        eprintln!("{err}");
        exit(1);
    }

    println!("::set-output name=url::{url}");
    println!("::set-output name=path::{}", write_to.display());

    println!("Success");
}
