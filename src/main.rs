#[macro_use]
extern crate log;

use simplog::simplog::SimpleLogger;

fn main() {
    SimpleLogger::init(Some("INFO"));

    let mut rt_int32: u32;
    let mut rt_int16: u16;
    let mut count: usize;

    // Do for u16 and f32 first
    let mut float32: f32;
    info!("Checking u16 -> f32 - from {} to {}", 0, std::u16::MAX);
    count = 0;
    for int16 in 0_u16..std::u16::MAX {
        float32 = int16 as f32;
        rt_int16 = float32 as u16;
        if int16 != rt_int16 {
//            error!("{} as u32 is {} as f64", int32, float64);
            count += 1;
        }
    }
    info!("total count that do not match between u16 and f32 is {}", count);

    // Do for u32 and f64 first
    let mut float64: f64;
    info!("Checking u32 -> f64 - from {} to {}", 0, std::u32::MAX);
    count = 0;
    for int32 in 0_u32..std::u32::MAX {
        float64 = int32 as f64;
        rt_int32 = float64 as u32;
        if int32 != rt_int32 {
//            error!("{} as u32 is {} as f64", int32, float64);
            count += 1;
        }
    }
    info!("total count that do not match between u32 and f32 is {}", count);

    // Do for u32 and f32
    let mut float32: f32;
    info!("Checking u32 -> f32 - from {} to {}", 0, std::u32::MAX);
    count = 0;
    for int32 in 0_u32..std::u32::MAX {
        float32 = int32 as f32;
        rt_int32 = float32 as u32;
        if int32 != rt_int32 {
//            error!("{} as u32 is {} as f32", int32, float32);
            count += 1;
        }
    }
    info!("total count that do not match between u32 and f32 is {}", count);
}
