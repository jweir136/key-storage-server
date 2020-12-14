use crate::server::types;
use crate::server::codes;
use std::net::TcpStream;
use std::io::Write;

/// This method is used to parse input and create the target return type.
/// Arguments:
///     response: ResponseCodes => This is the server defined enum value that is used to denote the type of error (if any) occured.
///     out : Option<[u8; 32]>  => This is the option which determines if any public key bytes will be returned.
/// Points of Failure:
///     None
pub fn format_json(response: codes::ResponseCodes, out: Option<types::PublicKey>) -> String {
    let (code, string_output): (i32, String) = match response {
        codes::ResponseCodes::Valid => { 
            match out {
                Some(key) => { (200, format!("{:?}", key)) },
                None => { (200, "".to_string()) }
            }
        },
        codes::ResponseCodes::InvalidRequestType => { (500, "Invalid Request".to_string()) },
        codes::ResponseCodes::CannotReadUsername => { (501, "Cannot read inputed username".to_string()) },
        codes::ResponseCodes::CannotReadPublicKey => { (502, "Cannot read inputed key".to_string()) },
        codes::ResponseCodes::UsernameNotFound => { (503, "Given username already exists".to_string()) },
        codes::ResponseCodes::UsernameAlreadyExists => { (504, "Given username not found".to_string()) },
        codes::ResponseCodes::CannotLockMutex => { (505, "Mutex cannot be locked".to_string()) },
        codes::ResponseCodes::CannotWriteToStream => { (506, "Cannot return to user".to_string()) }
    };
    format!("{{ 'code':{}, 'response':{} }}", code, string_output)
}

pub fn output_to_user(stream: &mut TcpStream, bytes: &[u8]) -> Result<(), codes::ResponseCodes> {
    match stream.write_all(bytes) {
        Ok(_) => {
            Ok(())
        },
        Err(_) => {
            Err(codes::ResponseCodes::CannotWriteToStream)
        }
    }
}