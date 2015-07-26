//! Messages between server and client

use rustc_serialize::json;
use rustc_serialize::json::{DecodeResult, EncoderError};

#[derive(RustcDecodable, RustcEncodable, PartialEq, Debug)]
pub enum ServerToClient {
    ConnectResponse(usize), // User ID
    CreateGameResponse(usize), // Game Id
}

#[derive(RustcDecodable, RustcEncodable, PartialEq, Debug)]
pub enum ClientToServer {
    ConnectRequest,
    CreateGameRequest(usize), // User Id
}

pub trait Message {
    fn stringify(&self) -> Result<String, EncoderError>;
    fn parse(&String) -> DecodeResult<Self>;
}

impl Message for ServerToClient {
    fn stringify(&self) -> Result<String, EncoderError> {
        json::encode(self)
    }
    fn parse(message: &String) -> DecodeResult<Self> {
        json::decode(message)
    }
}

impl Message for ClientToServer {
    fn stringify(&self) -> Result<String, EncoderError> {
        json::encode(self)
    }
    fn parse(message: &String) -> DecodeResult<Self> {
        json::decode(message)
    }
}

#[test]
fn test_connect_response() {
    let original = ServerToClient::ConnectResponse(3);
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
    let original = ClientToServer::CreateGameRequest(3);
    let encoded = original.stringify().unwrap();
    let parsed: ClientToServer = Message::parse(&encoded).unwrap();
    assert_eq!(parsed, original);
}

#[test]
fn test_create_game_response() {
    let original = ServerToClient::CreateGameResponse(3);
    let encoded = original.stringify().unwrap();
    let parsed: ServerToClient = Message::parse(&encoded).unwrap();
    assert_eq!(parsed, original);
}
