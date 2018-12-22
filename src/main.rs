fn main() {
    let mut float32: f32;
    let mut rt_int32: u32;
    let mut count: usize = 0;

    // Do for u32 first
    println!("Checking u32 from {} to {}", 0, std::u32::MAX);
    for int32 in 0_u32..std::u32::MAX {
        float32 = int32 as f32;
        rt_int32 = float32 as u32;
        if int32 != rt_int32 {
            println!("{} in u32 is {} in f32", int32, float32);
            count += 1;
        }
    }
    println!("total count that do not match between u32 and f32 is {}", count);
}
