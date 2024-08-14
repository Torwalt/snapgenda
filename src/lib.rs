use chrono::{Datelike, Days, NaiveDate, NaiveDateTime};

#[derive(Debug, PartialEq)]
pub enum Error {
    WeekOutOfRange(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::WeekOutOfRange(s) => write!(f, "Week out of range {s}"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, PartialEq)]
pub struct WeekRequest {
    year: i32,
    week_number: u8,
}

impl WeekRequest {
    pub fn new(week_number: u8, year: i32) -> Result<WeekRequest, Error> {
        if week_number > 53 {
            return Err(Error::WeekOutOfRange(
                "week cannot be higher than 53".to_string(),
            ));
        }

        let wr = WeekRequest { year, week_number };

        let mon = week_start(&wr);

        // Week is considered part of the same year, when all days are part of the same year.
        // Atleast thats how google calendar does it. Shortcut: The sunday must be in the same
        // year.
        let sunday = mon.checked_add_days(Days::new(6)).unwrap();
        if sunday.year() > year {
            return Err(Error::WeekOutOfRange(format!(
                "year {} has less than {} weeks",
                year, week_number
            )));
        }

        Ok(wr)
    }
}

#[derive(Debug)]
pub struct CalendarSnapshot {
    pub week: Week,
}

impl CalendarSnapshot {
    pub fn new(wr: WeekRequest) -> CalendarSnapshot {
        CalendarSnapshot {
            week: Week::new(wr),
        }
    }
}

#[derive(Debug)]
pub struct Week {
    pub days: [Day; 7],
}

impl Week {
    pub fn new(wr: WeekRequest) -> Week {
        let mon = week_start(&wr);
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

fn week_start(wr: &WeekRequest) -> NaiveDate {
    let start_year = NaiveDate::from_ymd_opt(wr.year, 1, 1).unwrap();
    let week_multiplicator = wr.week_number - 1;
    let days_until_week = (week_multiplicator as u64) * 7;
    start_year
        .checked_add_days(Days::new(days_until_week))
        .unwrap()
}

#[derive(Debug)]
pub struct Day {
    pub slots: Vec<Slot>,
}

impl Day {
    pub fn new(d: NaiveDate) -> Day {
        // Unwrap is safe here.
        let start = d.and_hms_opt(0, 0, 0).unwrap();
        let end = d.and_hms_opt(23, 59, 59).unwrap();

        Day {
            slots: vec![Slot::new(start, end)],
        }
    }
}

#[derive(Debug)]
pub struct Slot {
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
    pub availability: Availability,
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

#[cfg(test)]
mod tests {
    use super::*;

    struct WeekStartTestCase {
        wr: WeekRequest,
        expected: NaiveDate,
    }

    #[test]
    fn test_week_start() {
        let test_cases = vec![
            WeekStartTestCase {
                wr: WeekRequest {
                    year: 2024,
                    week_number: 1,
                },
                expected: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            },
            WeekStartTestCase {
                wr: WeekRequest {
                    year: 2024,
                    week_number: 50,
                },
                expected: NaiveDate::from_ymd_opt(2024, 12, 9).unwrap(),
            },
        ];

        for test_case in test_cases {
            let act_day = week_start(&test_case.wr);
            assert_eq!(test_case.expected, act_day);
        }
    }

    struct WeekRequestTestCase {
        year: i32,
        week_number: u8,
        exp: Result<WeekRequest, Error>,
    }

    #[test]
    fn test_week_request() {
        let test_cases = vec![
            WeekRequestTestCase {
                year: 2024,
                week_number: 1,
                exp: Ok(WeekRequest {
                    year: 2024,
                    week_number: 1,
                }),
            },
            WeekRequestTestCase {
                year: 2024,
                week_number: 54,
                exp: Err(Error::WeekOutOfRange(
                    "week cannot be higher than 53".to_string(),
                )),
            },
            WeekRequestTestCase {
                year: 2024,
                week_number: 53,
                exp: Err(Error::WeekOutOfRange(
                    "year 2024 has less than 53 weeks".to_string(),
                )),
            },
            WeekRequestTestCase {
                year: 2024,
                week_number: 52,
                exp: Ok(WeekRequest {
                    year: 2024,
                    week_number: 52,
                }),
            },
        ];

        for test_case in test_cases {
            let wr_act = WeekRequest::new(test_case.week_number, test_case.year);
            assert_eq!(test_case.exp, wr_act);
        }
    }
}
