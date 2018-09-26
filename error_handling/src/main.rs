fn main() -> Result<(), std::num::ParseIntError> {
    println!("{:?}", "4".parse::<u32>());
    match "g".parse::<u32>() {
        Ok(value) => println!("Parsing worked: {}", value),
        Err(e) => println!("Problem parsing: {:?}", e),
    }
    println!("Hex: {:?}", u32::from_str_radix("f", 16));
    println!("Value or try: {:?}", u32::from_str_radix("ö", 16)? );
    Ok(())
}
