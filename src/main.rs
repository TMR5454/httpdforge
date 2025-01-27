use std::env;
//use std::fmt::format;
use anyhow::Context;
use env_logger;
use log;
use std::io::{Read, Write};
use std::net::TcpListener;

fn http_server() -> Result<(), anyhow::Error> {
    let linstener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in linstener.incoming() {
        let mut stream = stream.context("faile to create tcp-stream")?;
        let mut buffer = [0; 1024];

        if let Ok(_usize) = stream.read(&mut buffer) {
            let s = String::from_utf8(buffer.to_vec()).context("parse Error")?;
            log::debug!("\n{}", s);
            let _n = stream.write(&buffer).context(format!("fail echo \"{}\"reply", s))?;
        }
    }
    Ok(())
}

fn main() {
    env::set_var("RUST_BACKTRACE", "full");
    env::set_var("RUST_LOG", "trace");
    env_logger::init();

    http_server();
}
