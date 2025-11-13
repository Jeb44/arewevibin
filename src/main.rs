#[macro_use]
extern crate rocket;

pub mod event;
use chrono::prelude::*;
use event::Event;

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
