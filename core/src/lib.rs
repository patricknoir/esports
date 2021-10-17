#[macro_use]
pub extern  crate failure;

pub extern crate actix;
pub extern crate actix_web;
pub extern crate jsonwebtoken;

pub mod data;
pub mod prelude;
pub mod error;
pub mod api;
pub mod jwt;