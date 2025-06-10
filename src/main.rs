pub mod armstrong_numbers;

fn main() {
    let numbers = [9, 10, 153, 154, 370, 9474];

    for &number in numbers.iter() {
        println!(
            "{} is an Armstrong number? {}",
            number,
            armstrong_numbers::is_armstrong_number(number)
        );
    }
}
