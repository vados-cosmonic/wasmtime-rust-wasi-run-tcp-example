use std::io::Write;
use std::net::TcpStream;

fn main() {
    let _ = TcpStream::connect("127.0.0.1:8080")
        .unwrap()
        .write("what up".as_bytes());
}
