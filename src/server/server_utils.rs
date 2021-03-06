use std::net::TcpStream;
use std::io::{Read};
use crate::server::types;
use crate::server::codes;

/// This function is used to return the RequestCode used to indicate the user's desired action.
/// Arguments:
///     stream: &mut TcpStream => This is the stream item that is used to communicate with the user.
/// Points of Failure:
///     InvalidRequestType  =>  This code is used to idicate that the user has requested an operation that either doesn't exist of isn't supported.
///     CannotWriteToStream =>  This indicates a msc. error involving the stream.
pub fn get_user_request_type(stream: &mut TcpStream) -> Result<codes::RequestCodes, codes::ResponseCodes> {
    let mut buff = [0 as u8; 1];

    match stream.read_exact(&mut buff) {
        Ok(_) => {
            match buff[0] {
                0 => { Ok(codes::RequestCodes::Get) },
                1 => { Ok(codes::RequestCodes::Add) },
                _ => { Err(codes::ResponseCodes::InvalidRequestType) }
            }
        },
        Err(_) => {
            Err(codes::ResponseCodes::CannotWriteToStream)
        }
    }
}

/// This function retrieves the username from the connected user.
/// Arguments:
///     stream: &mut TcpStream => This is the stream item that is used to communicate with the user.
/// Points of Failure:
///     CannotReadUsername => This error occurs anytime the stream cannot be read or the given bytes cannot cannot be decoded to a valid utf8 string.
pub fn get_username(stream: &mut TcpStream) -> Result<types::Username, codes::ResponseCodes> {
    let mut buff = [0 as u8; 256];

    match stream.read(&mut buff) {
        Ok(size) => {
            match String::from_utf8(buff[0..size].to_vec()) {
                Ok(username) => {
                    Ok(username)
                },
                Err(_) => {
                    Err(codes::ResponseCodes::CannotReadUsername)
                }
            }
        },
        Err(_) => {
            Err(codes::ResponseCodes::CannotReadUsername)
        }
    }
}

/// This function is used to get the PublicKey bytes from the user.
/// Arguments:
///     stream: &mut TcpStream => This is the stream item that is used to communicate with the user.
/// Points of Failure:
///     CannotReadPublicKey => This error occurs anytime the stream cannot be read to the buffer.
pub fn get_public_key(stream: &mut TcpStream) -> Result<types::PublicKey, codes::ResponseCodes> {
    let mut buff = [0 as u8; 32];

    match stream.read_exact(&mut buff) {
        Ok(_) => { Ok(buff) },
        Err(_) => { Err(codes::ResponseCodes::CannotReadPublicKey) }
    }
}