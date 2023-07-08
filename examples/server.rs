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

extern crate rewave;

use async_trait::async_trait;
use rewave::data::*;
use rewave::wave::*;
use std::net::SocketAddr;
use tokio;

struct RepelServer {}

#[async_trait]
impl Rewave for RepelServer {
    async fn request(
        &self,
        request: Request,
    ) -> Result<Response, ResponseError> {
        println!("New request : {:?}", request);
        Ok(Response {
            body: Some(Vec::new()),
            header: ResponseHeader {
                status: Status::Ok,
                message: Some("hello world".to_string()),
                error: false,
            },
        })
    }
    async fn on_connect(
        &self,
        username: Option<&str>,
        addr: SocketAddr,
    ) {
        println!(
            "New connection: {}:{}",
            addr.ip(),
            addr.port()
        )
    }
}

#[tokio::main]
async fn main() {
    let repel = RepelServer {};
    let rwh = RewaveHandler::new(repel);
    let server = RewaveServer::new(
        rwh,
        RewaveServerConfig {
            auth: true,
            addr: Some("127.0.0.1:2079"),
        },
    );
    server.serve().await;
}
