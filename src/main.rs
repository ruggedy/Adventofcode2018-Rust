use std::io::Result;
mod day_1;
mod day_2;


fn main() -> Result<()> {
    println!("============ DAY ONE ============");
    let day_one_frequency = day_1::calc_frequency()?;
    println!("Final frequency value {}", day_one_frequency);


    println!("============ DAY TWO ============");
    let day_one_multi_frequency = day_1::calc_multiple_frequency()?;
    println!("Final frequency value {}", day_one_multi_frequency);
    Ok(())
}
