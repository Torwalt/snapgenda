use snapgenda::CalendarSnapshot;

pub struct Matrix {
    rows: Vec<Row>,
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
                    value: "".to_string(),
                },
                Cell {
                    value: "Mon".to_string(),
                },
                Cell {
                    value: "Tue".to_string(),
                },
                Cell {
                    value: "Wed".to_string(),
                },
                Cell {
                    value: "Thu".to_string(),
                },
                Cell {
                    value: "Fri".to_string(),
                },
                Cell {
                    value: "Sat".to_string(),
                },
                Cell {
                    value: "Sun".to_string(),
                },
            ],
        }
    }

    pub fn new() -> Row {
        // Every next row has to start with a cell denoting the time of day, e.g. 00:00, 01:00, etc.
        let cells: Vec<Cell> = Vec::new();

        Row { cells }
    }
}

#[derive(Debug, PartialEq)]
pub struct Slot {
    from: u8,
    to: u8,
}

impl Slot {
    pub fn new(from: u8) -> Slot {
        let to = from + 1;
        Slot { from, to }
    }

    pub fn new_slots() -> Vec<Slot> {
        let mut slots: Vec<Slot> = Vec::new();

        for i in 0..=22 {
            slots.push(Slot::new(i));
        }

        return slots;
    }
}

pub struct Cell {
    value: String,
}

pub fn render(cs: &CalendarSnapshot) -> Matrix {
    let mut rows: Vec<Row> = Vec::new();
    let header_row = Row::new_header();

    rows.push(header_row);

    Matrix { rows }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_slots() {
        let slots = Slot::new_slots();
        let exp: Vec<Slot> = vec![
            Slot { from: 0, to: 1 },
            Slot { from: 1, to: 2 },
            Slot { from: 2, to: 3 },
            Slot { from: 3, to: 4 },
            Slot { from: 4, to: 5 },
            Slot { from: 5, to: 6 },
            Slot { from: 6, to: 7 },
            Slot { from: 7, to: 8 },
            Slot { from: 8, to: 9 },
            Slot { from: 9, to: 10 },
            Slot { from: 10, to: 11 },
            Slot { from: 11, to: 12 },
            Slot { from: 12, to: 13 },
            Slot { from: 13, to: 14 },
            Slot { from: 14, to: 15 },
            Slot { from: 15, to: 16 },
            Slot { from: 16, to: 17 },
            Slot { from: 17, to: 18 },
            Slot { from: 18, to: 19 },
            Slot { from: 19, to: 20 },
            Slot { from: 20, to: 21 },
            Slot { from: 21, to: 22 },
            Slot { from: 22, to: 23 },
        ];

        assert_eq!(slots, exp);
    }
}
