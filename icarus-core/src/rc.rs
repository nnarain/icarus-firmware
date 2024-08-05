//
// rc.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 13 2024
//

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RcError {
    #[error("Not enough bytes to decode")]
    NotEnoughBytes
}

/// Data received from the RC controller
#[derive(Debug, Default)]
pub struct RcInput {
    pub chnl0: u16,
    pub chnl1: u16,
    pub chnl2: u16,
    pub chnl3: u16,
}

impl RcInput {
    pub fn new(chnl0: u16, chnl1: u16, chnl2: u16, chnl3: u16) -> Self {
        RcInput { chnl0, chnl1, chnl2, chnl3 }
    }

    pub fn throttle(&self) -> (f32, f32, f32, f32) {
        (self.chnl0 as f32, self.chnl1 as f32, self.chnl2 as f32, self.chnl3 as f32)
    }
}

impl TryFrom<&[u8]> for RcInput {
    type Error = RcError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() >= 8 {
            let chnl0 = (bytes[0] as u16) | ((bytes[1] as u16) << 8);
            let chnl1 = (bytes[2] as u16) | ((bytes[3] as u16) << 8);
            let chnl2 = (bytes[4] as u16) | ((bytes[5] as u16) << 8);
            let chnl3 = (bytes[6] as u16) | ((bytes[7] as u16) << 8);

            Ok(RcInput::new(chnl0, chnl1, chnl2, chnl3))
        }
        else {
            Err(RcError::NotEnoughBytes)
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum DecoderState {
    Header,
    RemainingBytes(u8),
    Footer,
}

/// Decodes the incoming
/// TODO(nnarain): I feel this could be some sort of async iterator implementation
pub struct RcInputDecoder {
    state: DecoderState,
    channel_bytes: [u8; 8],
}

impl Default for RcInputDecoder {
    fn default() -> Self {
        RcInputDecoder { state: DecoderState::Header, channel_bytes: [0u8; 8] }
    }
}

impl RcInputDecoder {
    /// Decoder the byte stream using a simple state machine
    pub fn update(&mut self, byte: u8) -> Option<RcInput> {
        let (next_state, packet) = match self.state {
            DecoderState::Header => {
                if byte == 0x0F {
                    (DecoderState::RemainingBytes(8), None)
                }
                else {
                    (DecoderState::Header, None)
                }
            }
            DecoderState::RemainingBytes(remaining) => {
                self.channel_bytes[(8 - remaining) as usize] = byte;
                if remaining > 1 {
                    (DecoderState::RemainingBytes(remaining - 1), None)
                }
                else {
                    (DecoderState::Footer, None)
                }
            },
            DecoderState::Footer => {
                if byte == 0x00 {
                    (DecoderState::Header, RcInput::try_from(&self.channel_bytes[..]).ok())
                } else {
                    (DecoderState::Header, None)
                }
            },
        };

        self.state = next_state;
        packet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rc_input_bytes() {
        let bytes: [u8; 8] = [
            1, 0,
            2, 0,
            3, 0,
            4, 0
        ];

        let rc_input = RcInput::try_from(&bytes[..]).unwrap();
        assert_eq!(rc_input.chnl0, 1);
        assert_eq!(rc_input.chnl1, 2);
        assert_eq!(rc_input.chnl2, 3);
        assert_eq!(rc_input.chnl3, 4);
    }

    #[test]
    fn decode_one_packet() {
        let bytes: [u8; 10] = [
            0x0F,
            1, 0,
            2, 0,
            3, 0,
            4, 0,
            0x00
        ];

        let mut decoder = RcInputDecoder::default();

        let mut rc_input: Option<RcInput> = None;

        for b in bytes {
            rc_input = decoder.update(b);
        }

        let rc_input = rc_input.unwrap();
        assert_eq!(rc_input.chnl0, 1);
        assert_eq!(rc_input.chnl1, 2);
        assert_eq!(rc_input.chnl2, 3);
        assert_eq!(rc_input.chnl3, 4);
    }
}
