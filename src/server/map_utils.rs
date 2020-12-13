use crate::server::types;
use crate::server::codes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::result::Result;

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