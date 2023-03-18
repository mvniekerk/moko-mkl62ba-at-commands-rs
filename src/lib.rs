#![no_std]

extern crate alloc;

pub mod client;
pub mod general;
pub mod lora;
pub mod urc;

use atat_derive::AtatResp;

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct NoResponse;