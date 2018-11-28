#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate zip;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

// TODO make overridable
const DEFAULT_DOWNLOAD_HOST: &'static str = "https://storage.googleapis.com";
// TODO fetch from runtime env, fallback to config
const REVISION: &'static str = "609904";

#[cfg(target_os = "linux")]
const FILE_NAME: &'static str = "chrome-linux";
#[cfg(target_os = "macos")]
const FILE_NAME: &'static str = "chrome-mac";
#[cfg(target_os = "windows")]
const FILE_NAME: &'static str = "chrome-win";

#[cfg(target_os = "linux")]
lazy_static! {
    static ref DOWNLOAD_URL: String = format!(
        "{}/chromium-browser-snapshots/Linux_x64/{}/{}.zip",
        DEFAULT_DOWNLOAD_HOST, REVISION, FILE_NAME,
    );
}

#[cfg(target_os = "macos")]
lazy_static! {
    static ref DOWNLOAD_URL: String = format!(
        "{}/chromium-browser-snapshots/Mac/{}/{}ip",
        DEFAULT_DOWNLOAD_HOST, REVISION, FILE_NAME,
    );
}

#[cfg(target_os = "windows")]
lazy_static! {
    // TODO Windows archive name changed at r591479.
    static ref DOWNLOAD_URL: String = format!(
        "{}/chromium-browser-snapshots/Win/{}/{}.zip",
        DEFAULT_DOWNLOAD_HOST,
        REVISION,
        FILE_NAME,
    );
    //static DOWNLOAD_URL: &'static str = "%s/chromium-browser-snapshots/Win_x64/%d/%s.zip");
}

fn extract_archive(file: &mut File, outpath: &Path) {
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outfile_name = outpath.join(file.sanitized_name());

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!(
                "File {} extracted to \"{}\"",
                file.name(),
                outfile_name.display()
            );
            fs::create_dir_all(&outfile_name).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                file.name(),
                outfile_name.display(),
                file.size()
            );
            if let Some(p) = outfile_name.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outfile_name).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                println!(
                    "Setting permissions {} on \"{}\"",
                    mode,
                    outfile_name.display()
                );
                fs::set_permissions(&outfile_name, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
}

fn download(dest_path: &Path) {
    let zip_path = &dest_path.join("chrome.zip");
    let extract_dir = &dest_path.join(FILE_NAME);
    let target_dir = &dest_path.join("chrome");

    if !target_dir.exists() {
        println!("Downloading chrome archive");

        let mut file = File::create(&zip_path).unwrap();
        let mut result =
            reqwest::get(DOWNLOAD_URL.as_str()).expect("Failed to downoad chrome binary");

        let mut buf: Vec<u8> = vec![];
        result
            .copy_to(&mut buf)
            .expect("Failed to write downloaded binary");
        file.write_all(&buf)
            .expect("Failed to write downloaded binary");
        println!("Done! Archive is now at {}", &zip_path.display());
    }

    if !target_dir.exists() {
        println!("Extracting chrome archive");
        let mut file = File::open(&zip_path).expect("Failed to open archive for reading");
        extract_archive(&mut file, &dest_path);

        println!("Deleting archive at {} to clean up", &zip_path.display());
        fs::remove_file(&zip_path).expect("Failed to clean up archive");

        println!("Renaming result to {}", &target_dir.display());
        fs::rename(&extract_dir, &target_dir).expect("Failed to rename extracted chrome directory");
    }

    println!("Chrome downloaded to {}", &extract_dir.display());
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir);
    download(&dest_path);
}
