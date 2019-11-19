mod day_1;
mod day_2;
mod day_3;

fn main() {
    println!("============ DAY ONE PART 1 ============");
    println!("Final frequency value {:?}", day_1::calc_frequency());

    println!("============ DAY ONE PART 2 ============");
    println!(
        "First double frequency value {}",
        day_1::calc_multiple_frequency().unwrap()
    );

    println!("============ DAY TWO PART 1 ============");
    println!("checksum value {}", day_2::calc_checksum().unwrap());

    println!("============ DAY TWO PART 2 ============");
    println!("checksum value {}", day_2::find_boxes().unwrap());

    println!("============ DAY THREE  ============");
    day_3::calc_overlapping_materials();
}
