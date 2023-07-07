extern crate rewave;
use std::{
    net::{TcpStream, Shutdown}, 
    io::{Write, Read, self}
};
use rewave::{data::*, buffer::BUFFER};


fn _read_stream(
    mut stream: TcpStream, 
    buff: &mut [u8]
) -> io::Result<Vec<u8>> {
    let mut data = Vec::new();
    
    while let Ok(size) = stream.read(buff) {
        if size == 0 {
            stream.shutdown(Shutdown::Both).ok();
            stream.flush().ok();
            break;
        }

        data.extend_from_slice(&buff[..size]);

        if size < BUFFER {
            break;
        }
    }

    Ok(data)
}

fn main() {
    let mut buff = vec![0; 1024];
    match TcpStream::connect("127.0.0.1:2000") {
        Ok(mut _stream) => {
            let body = Request {
                body: None,
                header: RequestHeader { auth: None, _type: RequestType::Auth }
            };
            let data = serialize::<Request>(&body).unwrap();
            let _ = _stream.write(&data);
            
            if let Ok(raw) = _read_stream(_stream, &mut buff) {
                let res = deserialize::<Response>(&raw)
                    .unwrap();
                println!("{:?}", res);
            }
             

        },
        Err(e) => {
            println!("Failed to connect : {}", e);
        }
    }
}
