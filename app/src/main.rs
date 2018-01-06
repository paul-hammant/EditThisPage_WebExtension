# Copyright Paul Hammant, 2018
# MIT license per the LICENSE.md in https://github.com/paul-hammant/EditThisPage_WebExtension/blob/master/LICENSE.txt

extern crate serde_json;

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

enum Error {
    IO(std::io::Error),
    UTF8(std::string::FromUtf8Error),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Error::IO(ref err) => write!(f, "{:?}", err),
            &Error::UTF8(ref err) => write!(f, "{:?}", err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Error::UTF8(err)
    }
}

fn get_message<T: Read>(stdin: &mut T) -> Result<String, Error> {
    let mut bytes: [u8; 4] = [0; 4];
    stdin.read_exact(&mut bytes)?;

    let msg_len: u32 = unsafe { transmute(bytes) };

    let mut bytes = vec![0; msg_len as usize];
    stdin.read_exact(&mut bytes)?;


    let result = String::from_utf8(bytes)?;
    Ok(result)
}

fn send_message<T: Write>(stdout: &mut T, msg: &str) -> Result<(), Error> {
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

fn main() {
    let files = vec![
        "/Applications/SeaMonkey.app/Contents/MacOS/seamonkey",
        "/usr/local/seamonkey/seamonkey",
        "C:/Program Files/mozilla.org/SeaMonkey/seamonkey.exe",
        //"D:/Program Files (x86)/SeaMonkey/seamonkey.exe",
    ];

    let cmd = one_of(&files).expect(&format!("Any of {:?} does not exist!", files));
    let mut stdout = std::io::stdout();
    let mut stdin = std::io::stdin();

    loop {
        let reply;

        let msg = get_message(&mut stdin).expect("Failed to read income msg");
        let msg = decode(&msg);

        if msg.starts_with("edit: ") {
            let pieces = msg.split(": ").collect::<Vec<&str>>();
            Command::new(&cmd)
                .args(&["-edit", pieces[1]])
                .spawn()
                .expect("Failed to create child project");
            reply = "ok";

        } else if msg == "quit" {
            send_message(&mut stdout, &encode("ok")).expect("Failed to write reply");
            thread::sleep(Duration::from_secs(5));
            return;

        } else {
            reply = "Command not understood"
        }

        send_message(&mut stdout, &encode(reply)).expect("Failed to write reply");
    }
}
