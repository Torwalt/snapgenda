use clap::Parser;
use std::error::Error;

use chrono::NaiveDate;
use snapgenda::{AddSlot, CalendarSnapshot, WeekRequest};

mod args;
mod google;
mod render;

fn main() -> Result<(), Box<dyn Error>> {
    let cli_args = args::Args::parse();

    let wr = WeekRequest::new(1, 2024)?;
    let clndr = CalendarSnapshot::new(wr);
    render_example(clndr);

    match cli_args.provider {
        args::Provider::GoogleCalendar => process_google_provider(cli_args)?,
    };

    Ok(())
}

fn process_google_provider(cli_args: args::Args) -> Result<(), Box<dyn Error>> {
    let google_args = args::GoogleArgs::new(cli_args)?;

    let calendar_args = google::CalendarArgs {
        from: NaiveDate::from_ymd_opt(2024, 9, 8).unwrap(),
        to: NaiveDate::from_ymd_opt(2024, 9, 14).unwrap(),
        calendar_id: google_args.calendar_email,
        client_args: google::ClientArgs {
            id: google_args.google_id,
            secret: google_args.google_secret,
        },
    };

    let google_calendar = google::fetch_calendar(calendar_args)?;

    Ok(())
}

fn render_example(mut clndr: CalendarSnapshot) {
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

    let render_matrix = render::render_calendar(&clndr);
    print!("{}", render_matrix.render());
}
