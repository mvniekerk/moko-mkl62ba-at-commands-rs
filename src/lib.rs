#![no_std]

pub mod client;
pub mod digester;
pub mod general;
pub mod lora;
pub mod urc;

use atat_derive::AtatResp;

#[derive(Debug, Clone, AtatResp, PartialEq)]
pub struct NoResponse;
