//
// error.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 11 2021
//

/// Point to Point Protocol Errors
#[derive(Debug)]
pub enum Error {
    /// Error encoding byte stream
    EncodeError,
    /// Error decoding byte stream
    DecodeError,
    /// Error queuing decoded value
    QueueError,
}
