use chrono::{Days, Local, NaiveDateTime};

type WeekNumber = u8;

#[derive(Debug)]
pub struct CalendarSnapshot {
    week: Week,
}

impl CalendarSnapshot {
    pub fn new() -> CalendarSnapshot {
        CalendarSnapshot {
            week: Week::new(21),
        }
    }
}

#[derive(Debug)]
pub struct Week {
    days: [Day; 7],
}

impl Week {
    pub fn new(calendar_week: u8) -> Week {
        // Grab the monday of the passed week.
        let mon = week_start(calendar_week);
        Week {
            days: [
                Day::new(mon),
                Day::new(mon.checked_add_days(Days::new(1)).unwrap()),
                Day::new(mon.checked_add_days(Days::new(2)).unwrap()),
                Day::new(mon.checked_add_days(Days::new(3)).unwrap()),
                Day::new(mon.checked_add_days(Days::new(4)).unwrap()),
                Day::new(mon.checked_add_days(Days::new(5)).unwrap()),
                Day::new(mon.checked_add_days(Days::new(6)).unwrap()),
            ],
        }
    }
}

pub fn week_start(w: WeekNumber) -> NaiveDateTime {
    // TODO: impl
    Local::now().naive_local()
}

#[derive(Debug)]
pub struct Day {
    slots: Vec<Slot>,
}

impl Day {
    pub fn new(d: NaiveDateTime) -> Day {
        // Unwrap is safe here.
        let start = d.date().and_hms_opt(0, 0, 0).unwrap();
        let end = d.date().and_hms_opt(23, 59, 59).unwrap();

        Day {
            slots: vec![Slot::new(start, end)],
        }
    }
}

#[derive(Debug)]
pub struct Slot {
    from: NaiveDateTime,
    to: NaiveDateTime,
    availability: Availability,
}

impl Slot {
    pub fn new(from: NaiveDateTime, to: NaiveDateTime) -> Slot {
        Slot {
            availability: Availability::Free,
            from,
            to,
        }
    }
}

#[derive(Debug)]
pub enum Availability {
    Busy,
    Free,
}
