pub fn is_armstrong_number(num: u32) -> bool {
    let num_str: String = num.to_string();
    let num_digits: u32 = num_str.len() as u32;

    let mut sum: u32 = 0u32;
    for ch in num_str.chars() {
        let digit: u32 = ch.to_digit(10).unwrap();

        sum += digit.pow(num_digits);
    }

    sum == num
}