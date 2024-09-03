use std::fs;

fn main() {
    let phone_prefixes: [&str; 6] = ["050", "052", "053", "054", "058", "059"];

    let mut numbers = String::new();
    for prefix in phone_prefixes {
        for i in 0..10000000 {
            numbers += prefix;
            numbers += &pad_zero(i);
            numbers += "\n";
        }
    }

    let _ = fs::write("PhoneNumbers.txt", numbers);
}

fn pad_zero(num: u32) -> String {
    let mut clone = num.to_string();
    while clone.len() < 7 {
        clone += "0";
    }
    clone
}
