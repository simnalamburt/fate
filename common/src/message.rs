//! Messages between server and client

use rustc_serialize::json;
use rustc_serialize::json::{DecodeResult, EncoderError};

#[derive(RustcDecodable, RustcEncodable, PartialEq, Debug)]
pub enum ServerToClient {
    ConnectResponse { user_id: usize },
    CreateGameResponse { game_id: usize },
}

#[derive(RustcDecodable, RustcEncodable, PartialEq, Debug)]
pub enum ClientToServer {
    ConnectRequest,
    CreateGameRequest { user_id: usize },
}

pub trait Message: Sized {
    fn stringify(&self) -> Result<String, EncoderError>;
    fn parse(_: &str) -> DecodeResult<Self>;
}

impl Message for ServerToClient {
    fn stringify(&self) -> Result<String, EncoderError> {
        json::encode(self)
    }
    fn parse(message: &str) -> DecodeResult<Self> {
        json::decode(message)
    }
}

impl Message for ClientToServer {
    fn stringify(&self) -> Result<String, EncoderError> {
        json::encode(self)
    }
    fn parse(message: &str) -> DecodeResult<Self> {
        json::decode(message)
    }
}

#[test]
fn test_connect_response() {
    let original = ServerToClient::ConnectResponse { user_id: 3 };
    let encoded = original.stringify().unwrap();
    let parsed: ServerToClient = Message::parse(&encoded).unwrap();
    assert_eq!(parsed, original);
}

#[test]
fn test_connect_request() {
    let original = ClientToServer::ConnectRequest;
    let encoded = original.stringify().unwrap();
    let parsed: ClientToServer = Message::parse(&encoded).unwrap();
    assert_eq!(parsed, original);
}

#[test]
fn test_create_game_request() {
    let original = ClientToServer::CreateGameRequest { user_id: 3 };
    let encoded = original.stringify().unwrap();
    let parsed: ClientToServer = Message::parse(&encoded).unwrap();
    assert_eq!(parsed, original);
}

#[test]
fn test_create_game_response() {
    let original = ServerToClient::CreateGameResponse { game_id: 3 };
    let encoded = original.stringify().unwrap();
    let parsed: ServerToClient = Message::parse(&encoded).unwrap();
    assert_eq!(parsed, original);
}
