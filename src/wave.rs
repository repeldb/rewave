use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt, AsyncReadExt};
use std::future::Future;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::{TcpListener, ToSocketAddrs};
use crate::data::{
    Request, 
    Response, 
    ResponseError, 
    ResponseHeader,
    RequestType,
    Status,
    deserialize, 
    serialize,
    process_req, 

};
use crate::buffer::{BUFFER, read_stream};


struct Inner<T>(Arc<T>);

#[async_trait]
pub trait Rewave: Send + Sync + 'static {
    async fn request(&self, request: Request) -> Result<Response, ResponseError>;
    async fn on_connect(&self, username: Option<&str>, addr: SocketAddr);
}

pub struct RewaveHandler<T: Rewave> {
    _inner: Inner<T>
}

impl <T: Rewave>Clone for RewaveHandler<T> {
    fn clone(&self) -> Self {
        Self {
            _inner: self._inner.clone()
        }
    }
}

impl <T: Rewave>Clone for Inner<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

pub struct RewaveServer<T: Rewave> {
    rwh: RewaveHandler<T>,
    auth: bool,
}


impl <T: Rewave>RewaveHandler<T> {
    pub fn new(inner: T) -> Self {
        let i = Inner(Arc::new(inner));
        Self { _inner: i }
    }
}

impl <T: Rewave> RewaveServer <T>{
    pub fn new(rwh: RewaveHandler<T>, auth: bool) -> Self {
        Self {
            rwh,
            auth
        }
    }

    /// Run and serve Rewave Server
    pub async fn serve<V: ToSocketAddrs>(&self, addr: V) {
        let listener = TcpListener::bind(addr)
            .await
            .unwrap();

    
        loop {
            let (stream, addr) = listener.accept()
                .await
                .unwrap();
              
            let rwh = self.rwh.clone();
            let mut buff = vec![0; 1024];
            tokio::spawn(async move {
            
                let username = "test";
                rwh._inner.0.on_connect(Some(username), addr)
                    .await;
                
                handle_request(stream, move | req | {
                    let rwh = rwh.clone();
                    async move { rwh._inner.0.request(req).await }
                })
                .await;
            });
        }
    }

} 

async fn handle_request<F, Fut, I>(mut stream: I, cb: F) 
    where
        F: Fn(Request) -> Fut,
        Fut: Future<Output = Result<Response, ResponseError>>,
        I: AsyncRead + AsyncWrite + Unpin,
    {
        let mut buffer = vec![0; BUFFER];
  
        loop {
            let data = match read_stream(&mut stream, &mut buffer)
                .await {
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
                                    status:  Status::Ok, 
                                    message: None, 
                                    error: false 
                                },
                            };

                            let res_data = serialize::<Response>(&res)
                                .unwrap();

                            stream.write_all(&res_data)
                                .await
                                .ok();

                            continue;
                        },
                        _ => req
                    };

                    let res = match cb(req).await {
                        Ok(res) => res,
                        Err(err) => Response {
                            body: None,
                            header: ResponseHeader { 
                                status: err.status, 
                                message: Some(err.message), 
                                error: true 
                            }
                        }
                    };

                    let res_data = serialize::<Response>(&res)
                        .unwrap();
                    stream.write_all(&res_data).await.ok();

                },
                Err(err) => {
                    let res = serialize::<Response>(&err)
                        .unwrap();

                    stream.write_all(&res).await.ok();
                }
            }
        }
}



