use std::net::TcpStream;
use std::io::{Read, Write};
use crate::server::types;
use crate::server::codes;

pub fn get_user_request_type(stream: &mut TcpStream) -> Result<codes::RequestCodes, codes::ResponseCodes> {
    let mut buff = [0 as u8; 1];

    match stream.read_exact(&mut buff) {
        Ok(_) => {
            match buff[0] {
                0 => { Ok(codes::RequestCodes::Get) },
                1 => { Ok(codes::RequestCodes::Add) },
                _ => { Err(codes::ResponseCodes::CannotReadUsername) }
            }
        },
        Err(_) => {
            Err(codes::ResponseCodes::CannotReadUsername)
        }
    }
}