#[macro_use]
extern crate rocket;
use std::fmt;

#[get("/")]
fn index() -> String {
    let mut calendar: Vec<Event> = Vec::new();
    calendar.push(Event::new(
        Date::new(2025, 11, 5),
        Some(Time::new(9, 0)),
        "Team retrospective",
        Some("Conference Room A"),
        Some("Discuss last sprint, blockers, and next steps."),
    ));
    calendar.push(Event::new(
        Date::new(2025, 11, 12),
        Some(Time::new(16, 0)),
        "Doctor appointment",
        Some("City Clinic"),
        None,
    ));
    calendar.push(Event::with_title(
        Date::new(2025, 12, 1),
        None,
        "Buy birthday gift",
    ));

    let mut text: String = String::from("Hello, world!\n");
    for event in &calendar {
        text += format!("{event:?} \n").as_str();
    }
    text
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

/// Simple year‑month‑day representation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    pub year: i32,
    pub month: u8, // 1‑12
    pub day: u8,   // 1‑31 (no validation here)
}

impl Date {
    /// Create a new date.
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time {
    pub hour: u8,   // 0‑23
    pub minute: u8, // 0‑59
}

impl Time {
    /// Create a new time.
    pub fn new(hour: u8, minute: u8) -> Self {
        Self { hour, minute }
    }
}

/// Calendar event with optional fields.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Event {
    pub date: Date,
    pub time: Option<Time>,
    pub title: String,
    pub place: Option<String>,
    pub description: Option<String>,
}

impl Event {
    /// Build a new event. `place` and `description` can be omitted (`None`).
    pub fn new<T: Into<String>>(
        date: Date,
        time: Option<Time>,
        title: T,
        place: Option<T>,
        description: Option<T>,
    ) -> Self {
        Self {
            date,
            time,
            title: title.into(),
            place: place.map(Into::into),
            description: description.map(Into::into),
        }
    }

    /// Convenience shortcut when you only have a title.
    pub fn with_title(date: Date, time: Option<Time>, title: impl Into<String>) -> Self {
        Self::new(date, time, title, None, None)
    }
}

/* Pretty‑print an event --------------------------------------------------- */
impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Date part
        write!(
            f,
            "{}-{:02}-{:02}: {}",
            self.date.year, self.date.month, self.date.day, self.title
        )?;

        // Optional time
        if let Some(time) = &self.time {
            write!(f, " @ {:?}", time)?;
        }

        // Optional place
        if let Some(place) = &self.place {
            write!(f, " @ {}", place)?;
        }

        // Optional description (on a new line for readability)
        if let Some(desc) = &self.description {
            write!(f, "\n    {}", desc)?;
        }
        Ok(())
    }
}
