use time::{PrimitiveDateTime, Date, Time};
mod gigasecond;

fn main() {
    let start = PrimitiveDateTime::new(
        Date::from_calendar_date(2024, time::Month::June, 9).unwrap(),
        Time::from_hms(20, 0, 0).unwrap(),
    );

    println!("{}", gigasecond::after(start));
}
