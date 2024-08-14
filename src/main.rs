use std::error::Error;

use snapgenda::{CalendarSnapshot, WeekRequest};

mod render;

fn main() -> Result<(), Box<dyn Error>> {
    let wr = WeekRequest::new(1, 2024)?;
    let clndr = CalendarSnapshot::new(wr);
    println!("{:?}", clndr);
    Ok(())
}
