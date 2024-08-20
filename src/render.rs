use chrono::{NaiveDateTime, NaiveTime, TimeDelta};
use snapgenda::{CalendarSnapshot, Slot};

pub struct Matrix {
    pub rows: Vec<Row>,
}

pub struct Row {
    cells: Vec<Cell>,
}

impl Row {
    pub fn new_header() -> Row {
        Row {
            cells: vec![
                // First Cell is empty.
                Cell {
                    values: vec!["-----------------".to_string()],
                    is_start: true,
                    is_end: false,
                },
                Cell {
                    is_start: true,
                    is_end: false,
                    values: vec!["Mon".to_string()],
                },
                Cell {
                    is_start: true,
                    is_end: false,
                    values: vec!["Tue".to_string()],
                },
                Cell {
                    is_start: true,
                    is_end: false,
                    values: vec!["Wed".to_string()],
                },
                Cell {
                    is_start: true,
                    is_end: false,
                    values: vec!["Thu".to_string()],
                },
                Cell {
                    is_start: true,
                    is_end: false,
                    values: vec!["Fri".to_string()],
                },
                Cell {
                    is_start: true,
                    is_end: false,
                    values: vec!["Sat".to_string()],
                },
                Cell {
                    is_start: true,
                    is_end: false,
                    values: vec!["Sun".to_string()],
                },
            ],
        }
    }

    pub fn new(time_slot: TimeSlot) -> Row {
        let mut cells: Vec<Cell> = Vec::new();

        cells.push(Cell {
            is_start: true,
            is_end: false,
            values: vec![time_slot.render()],
        });

        Row { cells }
    }

    pub fn new_rows() -> Vec<Row> {
        let mut rows: Vec<Row> = Vec::new();

        for i in 0..=23 {
            let from: u32 = i.try_into().unwrap();
            let ts = TimeSlot::new(from);
            let row = Row::new(ts);
            rows.push(row);
        }

        return rows;
    }

    pub fn render(&self) -> String {
        let mut out = String::new();

        for s in &self.cells {
            out.push_str(&s.render())
        }

        return out;
    }
}

pub struct Cell {
    is_start: bool,
    is_end: bool,
    values: Vec<String>,
}

impl Cell {
    pub fn render(&self) -> String {
        let mut out = String::new();
        for v in &self.values {
            let s = format!("| {:^9} |", v);
            out.push_str(&s.to_string())
        }
        return out;
    }

    pub fn from_slots(slots: Vec<Slot>) -> Vec<Cell> {
        let mut out: Vec<Cell> = Vec::new();
        for slot in slots {
            let cells = Cell::new_cells(slot.from, slot.to);
            out.extend(cells);
        }

        return out;
    }

    pub fn new_cells(from: NaiveDateTime, to: NaiveDateTime) -> Vec<Cell> {
        let out: Vec<Cell> = Vec::new();
        let diff = from - to;
        if diff <= TimeDelta::hours(1) {}

        return out;
    }
}

pub fn render_calendar(cs: &CalendarSnapshot) -> Matrix {
    let mut rows: Vec<Row> = Vec::new();
    let header_row = Row::new_header();
    rows.push(header_row);

    // Init all Rows
    let mut slot_rows: Vec<Row> = Vec::new();
    slot_rows.extend(Row::new_rows());

    for day in &cs.week.days {
        for (i, row) in slot_rows.iter_mut().enumerate() {
            let cell = Cell {
                values: vec!["Free".to_string()],
                is_start: false,
                is_end: false,
            };
            row.cells.push(cell);
        }
    }

    rows.extend(slot_rows);

    Matrix { rows }
}

#[derive(Debug, PartialEq)]
pub struct TimeSlot {
    from: NaiveTime,
    to: NaiveTime,
}

impl TimeSlot {
    fn new(from: u32) -> TimeSlot {
        let from_time = NaiveTime::from_hms_opt(from, 0, 0).unwrap();
        let (to_time, _) = from_time.overflowing_add_signed(TimeDelta::hours(1));
        TimeSlot {
            from: from_time,
            to: to_time,
        }
    }

    fn render(&self) -> String {
        let s = format!(
            "{} - {}",
            self.from.format("%H:%M"),
            self.to.format("%H:%M")
        );
        format!("| {:^9} |", s)
    }
}
