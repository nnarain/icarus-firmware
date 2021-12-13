//
// rx.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 11 2021
//
use crate::{reader::RingBufferReader, Error};

use heapless::spsc::Producer;
use bincode::Decode;

use crate::SYNC;

/// State machine for receiving bytes
enum RxState {
    Sync,
    Length,
    Transfer,
}

/// Convert a stream of bytes into a stream of the specified time
pub struct ReceiveQueue<'a, Msg, const BUF_SIZE: usize, const STREAM_SIZE: usize> {
    stream: Producer<'a, Msg, STREAM_SIZE>,

    state: RxState,

    buf: [u8; BUF_SIZE],
    read_idx: usize,
    write_idx: usize,

    remaining_bytes: usize,
}

impl<'a, Msg: Decode, const BUF_SIZE: usize, const STREAM_SIZE: usize> ReceiveQueue<'a, Msg, BUF_SIZE, STREAM_SIZE> {
    /// Construct a new [ReceiveQueue] with the stream
    pub fn new(stream: Producer<'a, Msg, STREAM_SIZE>) -> Self {
        ReceiveQueue {
            stream,

            state: RxState::Sync,

            buf: [0u8; BUF_SIZE],
            read_idx: 0,
            write_idx: 0,

            remaining_bytes: 0,
        }
    }

    /// Update the queue with the given byte
    pub fn update(&mut self, byte: u8) -> Result<(), Error> {
        // Store the byte in the buffer and advance the write index
        self.buf[self.write_idx] = byte;

        #[cfg(test)]
        println!("{}", byte);

        self.state = match self.state {
            RxState::Sync => {
                if byte == SYNC {
                    RxState::Length
                }
                else {
                    RxState::Sync
                }
            },
            RxState::Length => {
                // Add 2 for the CRC
                self.remaining_bytes = (byte as usize) + 2;
                RxState::Transfer
            },
            RxState::Transfer => {
                self.remaining_bytes -= 1;

                if self.remaining_bytes == 0 {
                    // TODO: CRC

                    // Read index plus sync and length
                    // TODO: Remove sync and length from buffer?
                    let read_idx = self.read_idx + 2;

                    let reader = RingBufferReader::new(&self.buf, read_idx);

                    let cmd = bincode::decode_from_reader(
                                reader,
                                bincode::config::Configuration::standard()
                            )
                            .map_err(|_| Error::DecodeError)?;

                    self.stream.enqueue(cmd).map_err(|_| Error::QueueError)?;

                    self.read_idx = self.write_idx + 1;

                    RxState::Sync
                }
                else {
                    RxState::Transfer
                }
            }
        };

        self.write_idx = (self.write_idx + 1) % BUF_SIZE;

        Ok(())
    }

    /// Update the queue with the given slice
    pub fn update_from_slice(&mut self, bytes: &[u8]) -> Result<(), Error> {
        for byte in bytes {
            self.update(*byte)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bincode::Encode;
    use heapless::spsc::Queue;

    #[derive(Encode, Decode, Debug, PartialEq)]
    enum Bar {
        A, B, C, D, E, F
    }

    #[derive(Encode, Decode, Debug, PartialEq)]
    struct Foo {
        x: u8,
        b: Bar,
    }

    #[test]
    fn receive_queue() {
        let mut queue: Queue<Foo, 3> = Queue::new();
        let (p, mut c) = queue.split();

        let mut foo_queue: ReceiveQueue<'_, Foo, 50, 3> = ReceiveQueue::new(p);

        let in_cmd = Foo {x: 10, b: Bar::C};
        let mut in_cmd_buf = [0u8; 50];

        let config = bincode::config::Configuration::standard();
        let len = bincode::encode_into_slice(in_cmd, &mut in_cmd_buf, config).unwrap();

        foo_queue.update(SYNC).unwrap();
        foo_queue.update(len as u8).unwrap();
        foo_queue.update_from_slice(&in_cmd_buf[0..len]).unwrap();
        foo_queue.update(0xFF).unwrap();
        foo_queue.update(0xFF).unwrap();

        assert_eq!(c.len(), 1);
        let cmd = c.dequeue().unwrap();
        assert_eq!(cmd, Foo {x: 10, b: Bar::C});
    }
}
