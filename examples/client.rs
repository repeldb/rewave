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

use rewave::{
    buffer::BUFFER,
    data::*,
};
use std::{
    io::{
        self,
        Read,
        Write,
    },
    net::{
        Shutdown,
        TcpStream,
    },
};

fn _read_stream(
    mut stream: TcpStream,
    buff: &mut [u8],
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
                header: RequestHeader {
                    auth: None,
                    _type: RequestType::Auth,
                },
            };
            let data = serialize::<Request>(&body).unwrap();
            let _ = _stream.write(&data);

            if let Ok(raw) = _read_stream(_stream, &mut buff) {
                let res = deserialize::<Response>(&raw).unwrap();
                println!("{:?}", res);
            }
        }
        Err(e) => {
            println!("Failed to connect : {}", e);
        }
    }
}
