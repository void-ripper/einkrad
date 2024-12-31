use borsh::{BorshDeserialize, BorshSerialize};

use crate::version::Version;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum Message {
    Greeting { version: Version },
}
