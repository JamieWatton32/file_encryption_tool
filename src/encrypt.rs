use std::collections::HashMap;
use cocoon::{Cocoon, Error};
use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct EncryptionData {
    inner: HashMap<String, String>,
}

impl EncryptionData{
    pub fn new() -> EncryptionData{
        return Self{
            inner: todo!(),
        }
    }
}