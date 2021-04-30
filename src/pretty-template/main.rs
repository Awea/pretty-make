// use error_chain::error_chain;
use std::fs::File;
use std::io::copy;
// use tempfile::Builder;

// 🚧 Is it off any use?
// https://docs.rs/error-chain/0.12.4/error_chain/
// error_chain! {
//      foreign_links {
//          Io(std::io::Error);
//          HttpRequest(reqwest::Error);
//      }
// }

pub fn update_or_create_using_template(file: &str, url: &str) {
    match download(file, url) {
        Err(e) => println!("{:?}", e),
        _ => (),
    }
}

// Inspiration: https://rust-lang-nursery.github.io/rust-cookbook/web/clients/download.html
fn download(file: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = url;
    let response = reqwest::blocking::get(target)?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        // let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{:?}'", file);
        File::create(file)?
    };
    let content = response.text()?;
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}
