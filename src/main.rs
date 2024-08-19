use std::error::Error;

use snapgenda::{CalendarSnapshot, WeekRequest};

mod render;

fn main() -> Result<(), Box<dyn Error>> {
    let wr = WeekRequest::new(1, 2024)?;
    let clndr = CalendarSnapshot::new(wr);
    let m = render::render_calendar(&clndr);

    for row in m.rows {
        println!("{}", row.render());
    }

    Ok(())
}
