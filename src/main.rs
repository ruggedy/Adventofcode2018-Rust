use std::io::Result;
mod day_1;
mod day_2;

fn main() -> Result<()> {
    println!("============ DAY ONE PART 1 ============");
    let day_one_frequency = day_1::calc_frequency()?;
    println!("Final frequency value {}", day_one_frequency);

    println!("============ DAY ONE PART 2 ============");
    let day_one_multi_frequency = day_1::calc_multiple_frequency()?;
    println!("First double frequency value {}", day_one_multi_frequency);

    println!("============ DAY TWO PART 1 ============");
    let day_two_checksum = day_2::calc_checksum()?;
    println!("checksum value {:?}", day_two_checksum);

    println!("============ DAY TWO PART 2 ============");
    let day_two_find_boxes = day_2::find_boxes()?;
    println!("checksum value {:?}", day_two_find_boxes);

    Ok(())
}
