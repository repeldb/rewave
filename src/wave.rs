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

use crate::buffer::{
    read_stream,
    BUFFER,
};
use crate::data::{
    deserialize,
    process_req,
    serialize,
    Request,
    RequestType,
    Response,
    ResponseError,
    ResponseHeader,
    Status,
};
use async_trait::async_trait;
use std::future::Future;
use std::{
    net::SocketAddr,
    sync::Arc,
};
use tokio::io::{
    AsyncRead,
    AsyncReadExt,
    AsyncWrite,
    AsyncWriteExt,
};
use tokio::net::{
    TcpListener,
    ToSocketAddrs,
};

struct Inner<T>(Arc<T>);

#[async_trait]
pub trait Rewave: Send + Sync + 'static {
    async fn request(
        &self,
        request: Request,
    ) -> Result<Response, ResponseError>;
    async fn on_connect(
        &self,
        username: Option<&str>,
        addr: SocketAddr,
    );
}

pub struct RewaveHandler<T: Rewave> {
    _inner: Inner<T>,
}

impl<T: Rewave> Clone for RewaveHandler<T> {
    fn clone(&self) -> Self {
        Self {
            _inner: self._inner.clone(),
        }
    }
}

impl<T: Rewave> Clone for Inner<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Debug, Clone)]
pub struct RewaveServerConfig {
    pub auth: bool,
    pub addr: Option<&'static str>,
}

pub struct RewaveServer<T: Rewave> {
    rwh: RewaveHandler<T>,
    config: RewaveServerConfig,
}

impl<T: Rewave> RewaveHandler<T> {
    pub fn new(inner: T) -> Self {
        let i = Inner(Arc::new(inner));
        Self { _inner: i }
    }
}

impl<T: Rewave> RewaveServer<T> {
    pub fn new(
        rwh: RewaveHandler<T>,
        config: RewaveServerConfig,
    ) -> Self {
        Self { rwh, config }
    }

    /// Run and serve Rewave Server
    pub async fn serve(&self) {
        let addr = self.config.addr.or(Some("0.0.0.0:2079")).unwrap();

        let listener = TcpListener::bind(addr).await.unwrap();

        loop {
            let (stream, addr) = listener.accept().await.unwrap();

            let rwh = self.rwh.clone();
            let config = self.config.clone();

            let mut _buff = vec![0; 1024];
            tokio::spawn(async move {
                let username = "test";
                rwh._inner
                    .0
                    .on_connect(Some(username), addr)
                    .await;

                handle_request(stream, move |req| {
                    let rwh = rwh.clone();
                    async move { rwh._inner.0.request(req).await }
                })
                .await;
            });
        }
    }
}

async fn handle_request<F, Fut, I>(
    mut stream: I,
    cb: F,
) where
    F: Fn(Request) -> Fut,
    Fut: Future<Output = Result<Response, ResponseError>>,
    I: AsyncRead + AsyncWrite + Unpin,
{
    let mut buffer = vec![0; BUFFER];

    loop {
        let data = match read_stream(&mut stream, &mut buffer).await {
            Ok(data) => data,
            Err(_err) => {
                break;
            }
        };

        if data.is_empty() {
            break;
        }

        match process_req(&data) {
            Ok(req) => {
                println!("{:?}", req);
                let req = match req.header._type {
                    RequestType::Ping => {
                        let res = Response {
                            body: Some(Vec::new()),
                            header: ResponseHeader {
                                status: Status::Ok,
                                message: None,
                                error: false,
                            },
                        };

                        let res_data = serialize::<Response>(&res).unwrap();

                        stream.write_all(&res_data).await.ok();

                        continue;
                    }
                    _ => req,
                };

                let res = match cb(req).await {
                    Ok(res) => res,
                    Err(err) => Response {
                        body: None,
                        header: ResponseHeader {
                            status: err.status,
                            message: Some(err.message),
                            error: true,
                        },
                    },
                };

                let res_data = serialize::<Response>(&res).unwrap();
                stream.write_all(&res_data).await.ok();
            }
            Err(err) => {
                let res = serialize::<Response>(&err).unwrap();

                stream.write_all(&res).await.ok();
            }
        }
    }
}
