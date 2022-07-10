use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fmt::Write;

fn main() {
  let listener = TcpListener::bind("127.0.0.1:9999").unwrap();
  println!("listening started, ready to accept");
  for stream in listener.incoming() {
    let stream = stream.unwrap();

    handle_connection(stream)
  }
}

fn handle_connection(mut stream: TcpStream) {
  let mut buf = [0; 1024];
  
  stream.read(&mut buf).unwrap();

  println!("Request: {}", String::from_utf8_lossy(&buf));
  write_bytes(&buf);
}

fn write_bytes(buf: &[u8]) {
  let mut s = String::new();
  for &byte in buf {
    write!(&mut s, "{:X}", byte).expect("Unable to write");
  }
  println!("{}\n", s);
}