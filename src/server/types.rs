use std::collections::HashMap;

pub type Username = String;
pub type PublicKey = [u8; 32];
pub type KeyMap = HashMap<Username, PublicKey>;