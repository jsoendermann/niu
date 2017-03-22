#![feature(lookup_host)]

extern crate byteorder;

use std::io::{Read, Write, Error, Cursor};
use std::net::{TcpListener, TcpStream, lookup_host};
use byteorder::{BigEndian, ReadBytesExt};


fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
  println!("New connection!");

  //   +----+----------+----------+
  //   |VER | NMETHODS | METHODS  |
  //   +----+----------+----------+
  //   | 1  |    1     | 1 to 255 |
  //   +----+----------+----------+
  let mut handshake_buffer = [0xFFu8; 1 + 1 + 255];
  stream.read(&mut handshake_buffer)?;

  let version = handshake_buffer[0];
  // let number_of_methods = handshake_buffer[1];
  // let methods = &handshake_buffer[2..257];

  if version != 5 {
    // Error
  }

  //   +----+--------+
  //   |VER | METHOD |
  //   +----+--------+
  //   | 1  |   1    |
  //   +----+--------+
  let method_selection: [u8; 2] = [
    0x5, // Socks 5
    0, // No authentication required
  ];
  stream.write(&method_selection)?;


  //   +----+-----+-------+------+----------+----------+
  //   |VER | CMD |  RSV  | ATYP | DST.ADDR | DST.PORT |
  //   +----+-----+-------+------+----------+----------+
  //   | 1  |  1  | X'00' |  1   | Variable |    2     |
  //   +----+-----+-------+------+----------+----------+
  let mut request_head_buffer = [0xFFu8; 4];
  stream.read_exact(&mut request_head_buffer)?;

  // if request_head_buffer[3] == 3 {
    let mut domain_name_length = [0xFFu8; 1];
    stream.read_exact(&mut domain_name_length)?;
    // println!("{:?}", domain_name_length[0]);
    // let mut domain_name: Vec<u8> = Vec::with_capacity(domain_name_length[0] as usize);
    let mut domain_name_bytes = vec![0xFFu8; domain_name_length[0] as usize];
    stream.read_exact(&mut domain_name_bytes)?;
    let domain_name = String::from_utf8_lossy(&domain_name_bytes);
    println!("{:?}", domain_name);
  // }

  let mut port_buffer = [0u8; 2];
  stream.read_exact(&mut port_buffer)?;

  let port_number = Cursor::new(port_buffer).read_u16::<BigEndian>().expect("");
  println!("{:?}:{:?}", domain_name, port_number);


  //outbound.set_read_timeout(Some(Duration::from_secs(5))).unwrap();

  return Ok(())
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8099").unwrap();

    println!("Server listening at port PORT");

    for stream in listener.incoming() {
      match stream {
          Ok(stream) => {
              match handle_client(stream) {
                Ok(_) => {}
                Err(_) => {}
              };
          }
          Err(_) => { /* connection failed */ }
      }
  }
}
