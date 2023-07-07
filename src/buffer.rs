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

use std::io;
use tokio::io::{
    AsyncRead,
    AsyncReadExt,
    AsyncWrite,
    AsyncWriteExt,
};

// Set the default buffer capacity to 8k
pub const BUFFER: usize = 8 * 1024;

pub async fn read_stream<I>(
    stream: &mut I,
    buff: &mut [u8],
) -> io::Result<Vec<u8>>
where
    I: AsyncRead + AsyncWrite + Unpin,
{
    let mut data = Vec::new();
    while let Ok(n) = stream.read(buff).await {
        if n == 0 {
            stream.shutdown().await?;
            stream.flush().await?;
            break;
        }

        data.extend_from_slice(&buff[..n]);

        if n < BUFFER {
            break;
        }
    }
    Ok(data)
}
