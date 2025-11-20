use std::collections::HashMap;

use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct EncryptionData {
    inner: HashMap<String, String>,
}

