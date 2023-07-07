use tokio::io::{
    AsyncRead,
    AsyncReadExt,
    AsyncWrite, 
    AsyncWriteExt
};
use std::io;
// Set the default buffer capacity to 8k
pub const BUFFER: usize = 8 * 1024;


pub async fn read_stream<I>(stream: &mut I, buff: &mut [u8]) -> io::Result<Vec<u8>>
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
