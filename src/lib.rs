#![no_std]

pub mod client;
pub mod general;
pub mod lora;
pub mod urc;
pub mod digester;

use atat_derive::AtatResp;

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct NoResponse;
