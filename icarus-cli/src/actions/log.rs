//
// log.rs
//
// @author Natesh Narain <nnaraindev@gmail.com>
// @date Dec 14 2021
//
use icarus_wire::IcarusState;

use clap::Parser;

use tokio::sync::mpsc::Receiver;

use serde::Serialize;

use std::{
    path::PathBuf,
    time::Instant, str::FromStr,
};

#[derive(Parser, Debug)]
pub struct Args {
    /// CSV output directory
    #[clap(value_parser, default_value = "output")]
    output_dir: String,
}

#[derive(Serialize, Debug)]
struct SensorRow {
    ts: f32,
    ax: f32,
    ay: f32,
    az: f32,
    gx: f32,
    gy: f32,
    gz: f32,
}

#[derive(Serialize, Debug)]
struct AttitudeRow {
    ts: f32,
    pitch: f32,
    roll: f32,
    yaw: f32,
}

pub async fn run(args: Args, mut recv: Receiver<IcarusState>) -> anyhow::Result<()> {
    let out_dir = PathBuf::from_str(&args.output_dir)?;

    let sensors_path = out_dir.join("sensors.csv");
    let attitude_path = out_dir.join("attitude.csv");

    // TODO: async csv writer
    let mut sensors_writer = csv::Writer::from_path(sensors_path)?;
    let mut attitude_writer = csv::Writer::from_path(attitude_path)?;

    let start = Instant::now();

    loop {
        match recv.recv().await {
            Some(state) => {
                let now = Instant::now();

                match state {
                    IcarusState::Sensors(sensors) => {
                        let imu_row = SensorRow {
                            ts: now.duration_since(start).as_secs_f32(),
                            ax: sensors.accel.x,
                            ay: sensors.accel.y,
                            az: sensors.accel.z,
                            gx: sensors.gyro.x,
                            gy: sensors.gyro.y,
                            gz: sensors.gyro.z,
                        };
                        sensors_writer.serialize(imu_row)?;
                    },
                    IcarusState::EstimatedState(state) => {
                        let attitude_row = AttitudeRow {
                            ts: now.duration_since(start).as_secs_f32(),
                            pitch: state.attitude.pitch,
                            roll: state.attitude.roll,
                            yaw: state.attitude.yaw,
                        };
                        attitude_writer.serialize(attitude_row)?;
                    },
                    _ => {}
                }
            },
            None => break,
        }
    }

    Ok(())
}
