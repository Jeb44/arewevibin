#[macro_use]
extern crate rocket;
use std::fmt::{self};

use chrono::prelude::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[get("/")]
fn index() -> String {
    let data = NaiveDate::from_ymd_opt(2025, 11, 16);
    if let Some(date) = data {
        println!("Date: {}", date);
    }

    let local: DateTime<Local> = Local::now();
    // only visible inside the CLI
    println!("/************************/");
    println!("{:?}", local.day());
    println!("/************************/");
    let mut calendar: Vec<Event> = vec![];
    {
        calendar.push(Event::new(
            NaiveDate::from_ymd_opt(2025, 11, 5).unwrap(),
            NaiveTime::from_hms_opt(9, 0, 0),
            None,
            None,
            "Team retrospective",
            Some("Conference Room A"),
            Some("Discuss last sprint, blockers, and next steps."),
        ));
        calendar.push(Event::new(
            NaiveDate::from_ymd_opt(2025, 11, 12).unwrap(),
            None,
            None,
            None,
            "Doctor appointment",
            Some("City Clinic"),
            None,
        ));
        calendar.push(Event::new(
            NaiveDate::from_ymd_opt(2025, 12, 1).unwrap(),
            None,
            None,
            None,
            "Buy birthday gift",
            None,
            None,
        ));
        calendar.push(Event::new(
        NaiveDate::from_ymd_opt(2025, 12, 1).unwrap(),
        NaiveTime::from_hms_opt(14, 0, 0),
        NaiveDate::from_ymd_opt(2025, 12, 1),
        NaiveTime::from_hms_opt(15, 0, 0),
        "Wawaw",
        Some("Home uwu"),
        Some("Super elorabitieave party yep yep\nAs suprise guest Katty Parry will do nothing!!\n\nBUH"),
    ));
    }

    let mut text = "".to_owned();
    for event in &calendar {
        text.push_str(format!("{event} \n").as_str());
        text += "\n\n";
    }
    text += local.to_string().as_str();
    text
}

/// Calendar event with optional fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Event {
    pub start_date: NaiveDate,
    pub start_time: Option<NaiveTime>,
    pub end_date: Option<NaiveDate>,
    pub end_time: Option<NaiveTime>,
    pub title: String,
    pub place: Option<String>,
    pub description: Option<String>,
}

impl Event {
    pub fn new<T: Into<String>>(
        start_date: NaiveDate,
        start_time: Option<NaiveTime>,
        end_date: Option<NaiveDate>,
        end_time: Option<NaiveTime>,
        title: T,
        place: Option<T>,
        description: Option<T>,
    ) -> Self {
        Self {
            start_date,
            start_time,
            end_date,
            end_time,
            title: title.into(),
            place: place.map(Into::into),
            description: description.map(Into::into),
        }
    }
}

/* Pretty‑print an event --------------------------------------------------- */
impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "🐱 {}", self.title)?;
        let data = &self.start_date;
        write!(
            f,
            "⏲️ {}-{:02}-{:02}",
            data.year(),
            data.month(),
            data.day()
        )?;

        if let Some(time) = &self.start_time {
            write!(f, " {:02}:{:02}", time.hour(), time.minute())?;
        }

        if self.end_date.is_some() || self.end_time.is_some() {
            write!(f, " - ")?;

            if let Some(date) = &self.end_date {
                write!(f, "{}-{:02}-{:02}", date.year(), date.month(), date.day(),)?;
            }

            if let Some(time) = &self.end_time {
                write!(f, " {:02}:{:02}", time.hour(), time.minute())?;
            }
        }
        writeln!(f)?;

        if let Some(place) = &self.place {
            writeln!(f, "🏡 {}", place)?;
        }

        if let Some(desc) = &self.description {
            let desc = desc.trim().replace("\n", "\n| ");
            writeln!(f, "| {}", desc)?;
        }
        Ok(())
    }
}
