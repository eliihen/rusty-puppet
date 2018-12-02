#![feature(futures_api, async_await, await_macro)]
#[macro_use]
extern crate log;
extern crate futures;
extern crate rand;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate websocket;

pub mod browser;
pub mod connection;
pub mod handle;
pub mod launcher;
pub mod message;
pub mod page;
pub mod ws;
