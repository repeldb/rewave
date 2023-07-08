/*
 * Copyright (c) 2023-present repelDB
 *
 * See the license file for more info
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use serde::{
    de::DeserializeOwned,
    Deserialize,
    Serialize,
};
use serde_json;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum OpCode {
    Insert,
    Get,
    Update,
    Delete,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RepelData {
    pub data: Vec<u8>,
    pub op: OpCode,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestBody {
    pub op: OpCode,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RequestType {
    Auth,
    Ping,
    Query,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RequestHeader {
    pub auth: Option<String>,
    #[serde(rename = "type")]
    pub _type: RequestType,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Status {
    Ok,
    InvalidQuery,
    NotFound,
    Unauthorized,
    AlreadyExists,
    InvalidBody,
    SyntaxError,
    InternalError,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseHeader {
    pub status: Status,
    pub message: Option<String>,
    pub error: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseError {
    pub message: String,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Request {
    pub body: Option<RequestBody>,
    pub header: RequestHeader,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Response {
    pub body: Option<Vec<u8>>,
    pub header: ResponseHeader,
}

pub fn process_req(buf: &[u8]) -> Result<Request, Response> {
    if let Some(req) = deserialize::<Request>(buf) {
        Ok(req)
    } else {
        Err(Response {
            body: None,
            header: ResponseHeader {
                status: Status::InvalidBody,
                message: Some(String::from("Invalid request format")),
                error: true,
            },
        })
    }
}

pub fn deserialize<V>(data: &[u8]) -> Option<V>
where
    V: DeserializeOwned,
{
    match serde_json::from_str(std::str::from_utf8(data).unwrap()) {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}

pub fn serialize<V>(data: &V) -> Result<Vec<u8>, String>
where
    V: Serialize,
{
    match serde_json::to_string(data) {
        Ok(data) => Ok(data.into_bytes()),
        Err(err) => Err(err.to_string()),
    }
}
