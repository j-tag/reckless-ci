use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::process::Command;
use std::thread;

fn handle_client(mut stream: UnixStream) -> std::io::Result<()> {
    let mut message = String::new();
    stream.read_to_string(&mut message)?;
    println!("{message}");
    run_command()?;
    stream.write_all(b"hello client")?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let socket_path = "/tmp/reckless.sock";

    if std::fs::metadata(socket_path).is_ok() {
        std::fs::remove_file(socket_path)?;
    }

    let listener = UnixListener::bind(socket_path)?;

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                /* connection succeeded */
                thread::spawn(|| handle_client(stream));
            }
            Err(_err) => {
                /* connection failed */
                break;
            }
        }
    }
    Ok(())
}

fn run_command() -> std::io::Result<()> {
    let _output = Command::new("sh")
        .arg("-c")
        .arg("cargo build")
        .current_dir("/path/to/repo")
        .output()?;

    Ok(())
    //println!("{}", output.stdout.as_slice());
}
