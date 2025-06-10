pub mod leap_year;

fn main() {
    let year = 2024;
    println!("Is {} a leap year? {}", year, leap_year::is_leap_year(year));
}
