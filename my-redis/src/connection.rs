use tokio::net::TcpStream;
use mini_redis::{Frame, Result};
use bytes::BytesMut;
use std::io::Cursor;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};

struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
    cursor: unsize
}

impl Connection {
    pub fn new(steam: TcpStream) -> Connection {
        Connection {
            stream,
            // Allocate the buffer with 4kb of capacity.
            buffer: BytesMut::with_capacity(4096), // vec![0, 4096],// BytesMut::with_capacity(4096),
            cursor: 0,
        }
    }

    pub fn parse_frame(&mut self)
        -> Result<Option<Frame>>
    {
        // Create the `T: Buf` type.
        let mut buf = Cursor::new(&self.buffer[..]);

        //Check whether a full frame is available
        match Frame::check(&mut buf) {
            Ok(_) => {
                // Get the byte length of the frame
                let len = buf.position() as usize;

                // Reset the internal cursor for the
                // call to `parse`.
                buf.set_position(0);

                // Parse the frame
                let frame = Frame::parse(&mut buf)?;

                // Discard the frame from the buffer
                self.buffer.advance(len);

                // Return the frame to the caller.
                Ok(Some(frame))
            }
            // Not enough data has been buffered
            Err(Incomplete) => Ok(None),
            // An error was encountered
            Err(e) => Err(e.into()),
        }
    }

    pub async fn read_frame(&mut self)
        -> Result<Option<Frame>>
    {
        loop {
            // Attempt to parse a frame from the buffered data. If
            // enough data has been buffered, the frame is
            // returned.
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            // // Ensure the buffer has capacity
            // if self.buffer.len() == self.cursor {
            //     // Grow the buffer
            //     self.buffer.resize(self.cursor * 2, 0);
            // }
            //
            // // Read into the buffer, tracking the number
            // // of bytes read
            // let n = self.stream.read(
            //     &mut self.buffer[self.cursor..]).await?;
            //
            // if 0 == n {
            //     return if self.cursor == 0 {
            //         Ok(None)
            //     } else {
            //         Err("connection reset by peer".into())
            //     }
            // } else {
            //     // Update our cursor
            //     self.cursor += n;
            // }

            /// There is not enough buffered data to read a frame.
            /// Attempt to read more data from the socket.
            ///
            /// On success, the number of bytes is returned. `0`
            /// indicates "end of stream".
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                // The remote closed the connection. For this to be
                // a clean shutdown, there should be no data in the
                // read buffer. If there is, this means that the
                // peer closed the socket while sending a frame.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }

    async fn write_value(&mut self, frame: &Frame)
        -> io::Result<()>
    {
        match frame {
            Frame::Simple(val) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*val).await?;
            }
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            }
            Frame::Bulk(val) => {
                let len = val.len();

                self.stream.write_u8(b'$').await?;
                self.write_decimal(len as u64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Array(_val) => unimplemented!(),
        }

        self.stream.flush().await;

        Ok(())
    }

    pub async fn write_frame(&mut self, frame: &Frame)
        -> io::Result<()>
    {
        // Arrays are encoded by encoding each entry. All other frame types are
        // considered literals. For now, mini-redis is not able to encode
        // recursive frame structures. See below for more details.
        match frame {
            Frame::Array(val) => {
                // Encode the frame type prefix. For an array, it is `*`.
                self.stream.write_u8(b'*').await?;

                // Encode the length of the array.
                self.write_decimal(val.len() as u64).await?;

                // Iterate and encode each entry in the array.
                for entry in &**val {
                    self.write_value(entry).await?;
                }
            }
            // The frame type is a literal. Encode the value directly.
            _ => self.write_value(frame).await?,
        }

        // Ensure the encoded frame is written to the socket. The calls above
        // are to the buffered stream and writes. Calling `flush` writes the
        // remaining contents of the buffer to the socket.
        self.stream.flush().await
    }
}
