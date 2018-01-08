// Copyright Paul Hammant, 2018
// MIT license per the LICENSE.md in
// https://github.com/paul-hammant/EditThisPage_WebExtension/blob/master/LICENSE.txt

extern crate serde_json;
mod error;
use error::Error;

use std::path::PathBuf;
use std::process::Command;
use std::io::{Read, Write};
use std::mem::transmute;
use std::thread;
use std::time::Duration;

fn one_of(first: &[&str]) -> Option<PathBuf> {
    for path in first {
        let path = PathBuf::from(path);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

fn get_message<T: Read>(stdin: &mut T) -> Result<String, Error> {
    let mut bytes: [u8; 4] = [0; 4];
    stdin.read_exact(&mut bytes)?;

    let msg_len: u32 = unsafe { transmute(bytes) };

    let mut bytes = vec![0; msg_len as usize];
    stdin.read_exact(&mut bytes)?;


    let result = String::from_utf8(bytes)?;
    Ok(decode(&result))
}

fn send_message<T: Write>(stdout: &mut T, msg: &str) -> Result<(), Error> {
    let msg = encode(msg);

    let bytes: [u8; 4] = unsafe { transmute(msg.len() as u32) };
    stdout.write(&bytes)?;
    stdout.write(msg.as_bytes())?;
    stdout.flush()?;
    Ok(())
}

fn encode(text: &str) -> String {
    serde_json::to_string(text).unwrap()
}

fn decode(text: &str) -> String {
    serde_json::from_str(text).unwrap()
}

fn get_paths() -> Result<Vec<&'static str>, Error> {
    let files;
    if cfg![target_os = "windows"] {
        files = vec![
            "C:\\Program Files\\mozilla.org\\SeaMonkey\\seamonkey.exe",
            "D:\\Program Files (x86)\\SeaMonkey\\seamonkey.exe",
        ]
    } else if cfg![target_os = "macos"] {
        files = vec!["/Applications/SeaMonkey.app/Contents/MacOS/seamonkey"]
    } else if cfg![target_os = "linux"] {
        files = vec!["/usr/local/seamonkey/seamonkey"]
    } else {
        return Err(Error::Custom("Unknown OS".to_owned()));
    };

    Ok(files)
}

fn start() -> Result<(), Error> {
    let files = get_paths()?;
    let cmd = one_of(&files).ok_or(format!(
        "Error: None of these executable paths exist:
        {:?}, therefore no launching of an editor!",
        files
    ))?;

    let mut stdout = std::io::stdout();
    let mut stdin = std::io::stdin();

    loop {
        let reply;
        let msg = get_message(&mut stdin)?;

        if msg.starts_with("edit: ") {
            let pieces = msg.split(": ").collect::<Vec<&str>>();
            Command::new(&cmd).args(&["-edit", pieces[1]]).spawn()?;
            reply = "ok";

        } else if msg == "quit" {
            send_message(&mut stdout, "ok")?;

            thread::sleep(Duration::from_secs(5));
            break;
        } else {
            reply = "Command not understood"
        }

        send_message(&mut stdout, reply)?;
    }
    return Ok(());
}


fn main() {
    if let Err(err) = start() {
        let last_msg = std::error::Error::description(&err);
        send_message(&mut std::io::stdout(), last_msg).unwrap();
    }
}
