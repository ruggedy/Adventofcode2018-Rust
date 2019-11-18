mod day_1;
mod day_2;
mod day_3;

fn main() {
    println!("============ DAY ONE PART 1 ============");
    println!("Final frequency value {:?}", day_1::calc_frequency());

    println!("============ DAY ONE PART 2 ============");
    println!(
        "First double frequency value {:?}",
        day_1::calc_multiple_frequency()
    );

    println!("============ DAY TWO PART 1 ============");
    println!("checksum value {:?}", day_2::calc_checksum());

    println!("============ DAY TWO PART 2 ============");
    println!("checksum value {:?}", day_2::find_boxes());

    println!("============ DAY THREE PART 1 ============");
    println!(
        "intersection value {:?}",
        day_3::calc_overlapping_materials()
    );

    println!("============ DAY THREE PART 2 ============");
    println!("checksum value {:?}", day_3::find_valid_claim());
}
