//
// telemetry.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 16 2024
//

#[derive(Debug, Clone, Copy)]
pub struct Telemetry {
    pub connected: bool,
    pub throttle: u16,
    pub t1: u16,
}
