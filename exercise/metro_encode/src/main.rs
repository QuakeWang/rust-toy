use encoding::all::GBK;
use encoding::{DecoderTrap, Encoding};

fn main() {
    let binary_codes = [
        "1011101010111100",
        "1101011011011101",
        "1011010111011000",
        "1100110011111010",
    ];

    let result: String = binary_codes
        .iter()
        .map(|x| {
            let hex_string = format!("{:x}", u32::from_str_radix(x, 2).unwrap());
            let bytes = hex::decode(hex_string).unwrap();
            GBK.decode(&bytes, DecoderTrap::Strict).unwrap()
        })
        .collect();

    println!("{}", result);
}
