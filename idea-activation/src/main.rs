use reqwest::{self};
use tokio::fs::{self, File};
use std::fs::{self as s_fs, remove_dir_all, remove_file};
use tokio::io::{self, AsyncWriteExt};
use std::process::Command;
use std::string::String;
use encoding_rs::{GBK};

async fn request_file() -> io::Result<()> {
    let mut resp = reqwest::get("http://idea.medeming.com/jets/images/jihuoma.zip").await.unwrap();
    let mut out = File::create("jihuoma.zip").await?;
    let mut bytes = resp.bytes().await.unwrap();
    out.write(&mut bytes.to_vec()).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // let client = reqwest::Client::new();
    request_file().await.unwrap();

    let mut output = Command::new("cmd")
        .args(&["/C", "7z", "x", "jihuoma.zip", "-ojihuoma/"])
        .output()
        .expect("failed to execute process");
    let d = GBK.decode(&mut output.stdout).0;
    println!("{}", d.to_string());

    let dirs = s_fs::read_dir("./jihuoma").unwrap();
    for dir in dirs {
        let f = dir.unwrap().file_name();
        let filename = f.to_string_lossy();
        match filename.find("later") {
            Some(_) => {
                Command::new("cmd")
                    .args(&["/C", "notepad", &("jihuoma/".to_string() + &*filename.to_string())])
                    .output()
                    .expect("failed to execute process");
            },
            _ => ()
        }
    }

    remove_file("jihuoma.zip").unwrap();
    remove_dir_all("./jihuoma").unwrap();

    // io::copy(&mut bytes.to_vec(), &mut out).await?;
    /*for mut b in &bytes.to_vec() {
        println!("{}", b);
    }*/
    // out.write(&mut bytes).await?;
    // io::copy(&mut resp, &mut out).unwrap();
    Ok(())
}
