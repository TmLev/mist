use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "usize")]
pub struct Request(pub usize);

#[derive(Message)]
#[rtype(result = "usize")]
pub struct Response(pub usize);

#[derive(Message)]
#[rtype(result = "usize")]
pub struct Dummy(pub usize);
