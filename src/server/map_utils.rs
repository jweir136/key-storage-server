use crate::server::types;
use crate::server::codes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::result::Result;

/// This function is used to extract the PublicKey bytes from the server data storage.
/// Arguments:
///     username: String => This is the username of the user which to query.
///     map: &Arc<Mutex<HashMap<String, [u8; 32]>>> => This is the HashMap which stores all the data. This function expects a mutex held within an atomic reference (which is standard for parallel computing).
/// Returns:
///     This function returns either the PublicKey or a ResponseCodes enum which is used to denote the error which occured.
/// Points of Failure:
///     UsernameNotFound => This failure occurs when the function tries to query a user which is not stored.
///     CannotLockMutex  => This failture occurs when the mutex cannot be locked or any reason.
pub fn get_from_map(username: types::Username, map: &Arc<Mutex<HashMap<types::Username, types::PublicKey>>>) -> Result<types::PublicKey, codes::ResponseCodes> {
    match map.lock() {
        Ok(guard) => {
            match guard.get(&username) {
                Some(key) => {
                    Ok(*key)
                },
                None => {
                    Err(codes::ResponseCodes::UsernameNotFound)
                }
            }
        },
        Err(_) => {
            Err(codes::ResponseCodes::CannotLockMutex)
        }
    }
}

#[test]
pub fn get_from_map_test() {
    let mut map = HashMap::<types::Username, types::PublicKey>::new();
    map.insert("Jake".to_string(), [1 as u8; 32]);
    map.insert("Ryan".to_string(), [0 as u8; 32]);

    let arcmap = Arc::new(Mutex::new(map));

    match get_from_map("Jake".to_string(), &arcmap) {
        Ok(key) => {
            assert_eq!(key, [1 as u8; 32]);
        },
        _ => {
            panic!();
        }
    };

    match get_from_map("Ryan".to_string(), &arcmap) {
        Ok(key) => {
            assert_eq!(key, [0 as u8; 32]);
        },
        _ => {
            panic!();
        }
    };
}