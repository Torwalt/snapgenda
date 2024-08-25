use chrono::{NaiveTime, TimeDelta};
use snapgenda::{CalendarSnapshot, Day};

const WEEK_DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

pub struct Matrix {
    rows: Vec<Row>,
}

impl Matrix {
    fn new(cs: &CalendarSnapshot) -> Matrix {
        let mut cols: Vec<Column> = Vec::new();
        cols.push(Column::new_timeslot());
        cols.extend(Column::from_days(&cs.week.days));
        let mut rows: Vec<Row> = Vec::new();
        rows.extend(Row::new_rows(&cols));

        Matrix { rows }
    }

    pub fn render(&self) -> String {
        let mut out = String::new();

        for row in &self.rows {
            let row_string = format!("{}\n", row.render()).to_string();
            out.push_str(&row_string);
        }

        out
    }
}

struct Row {
    cells: Vec<Cell>,
}

impl Row {
    fn new_rows(cols: &Vec<Column>) -> Vec<Row> {
        let mut rows: Vec<Row> = Vec::new();
        let mut buffer_cells: Vec<Cell> = Vec::new();
        for _ in 1..=7 {
            let cell = Cell::new(&Cell::new_buffer());
            buffer_cells.push(cell);
        }
        rows.push(Row {
            cells: buffer_cells,
        });

        for i in 0..=26 {
            let mut row_cells: Vec<Cell> = Vec::new();
            for col in cols {
                let cell = &col.cells[i];
                row_cells.push(cell.clone());
            }
            let row = Row { cells: row_cells };
            rows.push(row);
        }

        rows
    }

    fn render(&self) -> String {
        let mut out = String::new();

        for s in &self.cells {
            out.push_str(&s.render())
        }

        out
    }
}

struct Column {
    cells: Vec<Cell>,
}

impl Column {
    fn new_timeslot() -> Column {
        let time_slots = TimeSlot::new_range();
        let mut cells: Vec<Cell> = Vec::new();

        let render_buffer = Cell::new_buffer();
        cells.push(Cell::new(&render_buffer));

        for slot in time_slots {
            let v = slot.render();
            let cell = Cell { values: vec![v] };
            cells.push(cell);
        }
        cells.push(Cell::new(&render_buffer));
        cells.push(Cell::new(&render_buffer));

        Column { cells }
    }

    fn from_day(day: &Day, week_day: &str) -> Column {
        let mut cells: Vec<Cell> = Vec::new();
        cells.push(Cell::new_header_cell(week_day));

        for slot in &day.slots {
            let slot_cells = Cell::new_cells(&slot);
            cells.extend(slot_cells);
        }

        Column { cells }
    }

    fn from_days(days: &[Day; 7]) -> Vec<Column> {
        let mut out: Vec<Column> = Vec::new();

        for (day, week_day) in days.iter().zip(WEEK_DAYS) {
            let colmn = Column::from_day(&day, week_day);
            out.push(colmn);
        }

        out
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Cell {
    values: Vec<String>,
}

impl Cell {
    fn new(value: &String) -> Cell {
        Cell {
            values: vec![value.to_string()],
        }
    }

    fn new_buffer() -> String {
        let ts = TimeSlot::new(1);
        let ts_str = ts.render();
        let mut replaced = String::new();
        ts_str.chars().for_each(|_| {
            replaced.push(' ');
        });

        replaced
    }

    fn render(&self) -> String {
        let mut out = String::new();
        for v in &self.values {
            let value = format!("{:^9}", v);
            out.push_str(&value.to_string())
        }
        return out;
    }

    fn new_header_cell(week_day: &str) -> Cell {
        return Cell {
            values: vec![week_day.to_string()],
        };
    }

    fn new_cells(s: &snapgenda::Slot) -> Vec<Cell> {
        let mut out: Vec<Cell> = Vec::new();
        out.push(Cell {
            values: vec!["|¯¯¯¯¯¯|".to_string()],
        });

        let mut cursor = s.from;
        while cursor <= s.to {
            let box_value = format!("| {} |", s.availability.to_string());

            let c = Cell {
                values: vec![box_value],
            };
            out.push(c);

            cursor = cursor + TimeDelta::hours(1);
            if cursor >= s.to {
                break;
            }
        }
        out.push(Cell {
            values: vec!["|______|".to_string()],
        });

        return out;
    }
}

pub fn render_calendar(cs: &CalendarSnapshot) -> Matrix {
    let m = Matrix::new(cs);

    m
}

#[derive(Debug, PartialEq)]
struct TimeSlot {
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
        format!("{:^9}", s)
    }

    fn new_range() -> Vec<TimeSlot> {
        let mut out: Vec<TimeSlot> = Vec::new();
        for i in 0..=23 {
            out.push(TimeSlot::new(i));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime};
    use snapgenda::{Availability, Slot};

    use super::*;

    #[test]
    fn test_cell_new_cells() {
        let date = NaiveDate::from_ymd_opt(2024, 8, 21).unwrap();
        let from = NaiveDateTime::new(date, NaiveTime::from_hms_opt(10, 0, 0).unwrap());
        let to = NaiveDateTime::new(date, NaiveTime::from_hms_opt(15, 30, 0).unwrap());
        let s = Slot::new(from, to);

        let exp_cells = vec![
            // 10-11
            Cell {
                values: vec![Availability::Free.to_string()],
            },
            // 11-12
            Cell {
                values: vec![Availability::Free.to_string()],
            },
            // 12-13
            Cell {
                values: vec![Availability::Free.to_string()],
            },
            // 13-14
            Cell {
                values: vec![Availability::Free.to_string()],
            },
            // 14-15
            Cell {
                values: vec![Availability::Free.to_string()],
            },
            // 15-16
            Cell {
                values: vec![Availability::Free.to_string()],
            },
        ];
        let cells = Cell::new_cells(&s);
        assert_eq!(exp_cells.len(), cells.len());
        assert_eq!(exp_cells, cells)
    }
}
