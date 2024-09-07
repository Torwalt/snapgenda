use std::error::Error;

use snapgenda::{AddSlot, CalendarSnapshot, WeekRequest};

mod google;
mod render;

fn main() -> Result<(), Box<dyn Error>> {
    let wr = WeekRequest::new(1, 2024)?;
    let mut clndr = CalendarSnapshot::new(wr);
    let day_slot = &clndr.week.days[2].slots[0];

    let add_slot = AddSlot {
        week_day: snapgenda::WeekDay::Wednesday,
        slot: snapgenda::Slot {
            from: day_slot.from.date().and_hms_opt(10, 0, 0).unwrap(),
            to: day_slot.to.date().and_hms_opt(15, 0, 0).unwrap(),
            availability: snapgenda::Availability::Busy,
        },
    };
    clndr.add_slot(add_slot);

    let m = render::render_calendar(&clndr);

    print!("{}", m.render());

    Ok(())
}
