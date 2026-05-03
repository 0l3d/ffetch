use std::process::{ Command, Stdio};
use std::fs;
use std::path::Path;
use crate::ffetch;

fn download_file(URL: &str, out: &str) -> bool {
    let stat = Command::new("curl")
        .args(["-L", URL, "-o", out])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .expect("Curl command failed, PLEASE CHECK CURL is INSTALLED");
    if !stat.success() {
        return false;
    }
    true
}

pub fn check_first_run(filename: &str) -> bool {
    let conf_path = Path::new(filename);
    if !conf_path.exists() {
        if let Some(parent) = conf_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).expect("Parent directory create error.");
            }
        }

       return true;
    }
    false
}

pub fn check_platform_and_download_ascii() {
    let platform = ffetch::devfunc_get_os_id().expect("Failed to get distro name.");
    let user = ffetch::get_username();
    let home = format!("/home/{}/.config/ffetch/ascii.txt", user); 
    let raw_url = "https://git.sr.ht/~oled/ffetch/blob/master/ascii";
    let stat = download_file(format!("{}/{}", raw_url, platform).as_str(), home.as_str());
    if !stat {
        println!("We cant found an ascii for your distro.");
    } 
}
