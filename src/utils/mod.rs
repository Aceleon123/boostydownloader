use colored::Colorize;
use std::{fs::File, path::Path};
use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

pub async fn download(url: String, path: String) -> Result<()> {
    let file = format!("{}/{}.png", path, url.split_once("image/").unwrap().1.split_once("?").unwrap().0);
    if Path::new(&file).exists() {
        println!("Skipping {} because it's already downloaded", file.bright_blue());
        return Ok(());
    }
    println!("Downloading from {} to {}", url.bright_blue(), file.green()); 
    let resp = reqwest::get(url).await.expect("request failed");
    let mut out = File::create(file).expect("failed to create file");
    std::io::copy(&mut resp.bytes().await.unwrap().as_ref(), &mut out).expect("failed to copy content");

    Ok(())
}
