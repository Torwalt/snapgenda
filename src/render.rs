use chrono::{NaiveTime, TimeDelta, Timelike};
use snapgenda::{CalendarSnapshot, Day, Slot};

const WEEK_DAYS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

pub struct Matrix {
    rows: Vec<Row>,
    cols: Vec<Column>,
}

impl Matrix {
    fn new(cs: &CalendarSnapshot) -> Matrix {
        let cols = Column::from_days(&cs.week.days);
        let rows = Row::new_rows(&cols);

        Matrix { rows, cols }
    }

    fn rows_for_slot(&mut self, s: &Slot) -> &mut [Row] {
        let start = s.from.time().hour();
        let end = s.to.time().hour();

        &mut self.rows[start as usize..end as usize]
    }

    pub fn render(&self) -> String {
        let mut out = String::new();

        // let header_string = format!("{}\n", &self.header_row.render()).to_string();
        // out.push_str(&header_string);
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

        for i in 0..=23 {
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
    is_start: bool,
    is_end: bool,
    values: Vec<String>,
}

impl Cell {
    fn render(&self) -> String {
        let mut out = String::new();
        for v in &self.values {
            let s = format!("{:^9}", v);
            out.push_str(&s.to_string())
        }
        return out;
    }

    fn new_header_cell(week_day: &str) -> Cell {
        return Cell {
            is_start: false,
            is_end: false,
            values: vec![week_day.to_string()],
        };
    }

    fn new_cells(s: &snapgenda::Slot) -> Vec<Cell> {
        let mut out: Vec<Cell> = Vec::new();

        let mut cursor = s.from;
        while cursor <= s.to {
            let c = Cell {
                is_start: false,
                is_end: false,
                values: vec![s.availability.to_string()],
            };
            out.push(c);

            cursor = cursor + TimeDelta::hours(1);
            if cursor >= s.to {
                break;
            }
        }

        return out;
    }
}

pub fn render_calendar(cs: &CalendarSnapshot) -> Matrix {
    let mut m = Matrix::new(cs);

    for day in &cs.week.days {
        for slot in &day.slots {
            let cells = Cell::new_cells(slot);
            let slot_rows = m.rows_for_slot(slot);
            for (cell, row) in cells.iter().zip(slot_rows.iter_mut()) {
                row.cells.push(cell.clone())
            }
        }
    }

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
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveDateTime};
    use snapgenda::Availability;

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
                is_start: false,
                is_end: false,
                values: vec![Availability::Free.to_string()],
            },
            // 11-12
            Cell {
                is_start: false,
                is_end: false,
                values: vec![Availability::Free.to_string()],
            },
            // 12-13
            Cell {
                is_start: false,
                is_end: false,
                values: vec![Availability::Free.to_string()],
            },
            // 13-14
            Cell {
                is_start: false,
                is_end: false,
                values: vec![Availability::Free.to_string()],
            },
            // 14-15
            Cell {
                is_start: false,
                is_end: false,
                values: vec![Availability::Free.to_string()],
            },
            // 15-16
            Cell {
                is_start: false,
                is_end: false,
                values: vec![Availability::Free.to_string()],
            },
        ];
        let cells = Cell::new_cells(&s);
        assert_eq!(exp_cells.len(), cells.len());
        assert_eq!(exp_cells, cells)
    }
}
