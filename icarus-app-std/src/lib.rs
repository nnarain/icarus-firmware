//
// lib.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Jul 10 2022
//

pub mod stat;
pub mod wifi;
pub mod console;

pub struct ImuCalibrationOffset {
    pub ax_offset: f32,
    pub ay_offset: f32,
    pub az_offset: f32,
    pub gx_offset: f32,
    pub gy_offset: f32,
    pub gz_offset: f32,
}
