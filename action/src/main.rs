use std::{env, process::exit};

fn main() {
    let mut args = env::args();
    let name = args.next().unwrap_or_else(|| {
        eprintln!("Missing arg 1, the name");
        exit(1);
    });
    println!("Hello {name}");
    let time = std::time::Instant::now();
    println!("::set-output name=time::{time:?}")
}

