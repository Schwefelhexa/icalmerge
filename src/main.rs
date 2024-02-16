use std::{env, io::Cursor};

use ical::IcalParser;
use ics::{components::Property, properties::Name, ICalendar};
use itertools::Itertools;
use rocket::{get, http::ContentType, launch, response::Responder, routes, Response};

#[get("/")]
async fn index() -> Cal<'static> {
    let mut output_calendar = ICalendar::new(
        "2.0",
        format!(
            "-//Alexander Baron//iCal merge v{}//EN",
            env!("CARGO_PKG_VERSION")
        ),
    );
    output_calendar.push(Name::new("Merged Calendar"));

    let sources_raw = env::var("ICAL_SOURCES").unwrap_or_default();
    let sources = sources_raw.split(',').collect_vec();

    for source in sources {
        let ical = reqwest::get(source).await.unwrap().text().await.unwrap();
        let ical = IcalParser::new(ical.as_bytes());

        for calendar in ical.flatten() {
            for event in calendar.events {
                let uid = event
                    .properties
                    .iter()
                    .find(|p| p.name.to_uppercase() == "UID")
                    .and_then(|p| p.value.clone())
                    .unwrap();
                let dtstamp = event
                    .properties
                    .iter()
                    .find(|p| p.name.to_uppercase() == "DTSTAMP")
                    .and_then(|p| p.value.clone())
                    .unwrap();
                let mut output_event = ics::Event::new(uid, dtstamp);

                for prop in event.properties {
                    output_event.push(Property::new(
                        prop.name.clone(),
                        prop.value.clone().unwrap_or("".to_string()),
                    ));
                }

                output_calendar.add_event(output_event);
            }
        }
    }

    Cal(output_calendar)
}

#[launch]
fn rocket() -> _ {
    let _ = dotenvy::dotenv(); // Ignore faileure to load .env file
    rocket::build().mount("/", routes![index])
}

struct Cal<'a>(ICalendar<'a>);
#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for Cal<'_> {
    fn respond_to(self, _: &'r rocket::Request) -> rocket::response::Result<'static> {
        let cal = self.0.to_string();

        Response::build()
            .header(ContentType::Calendar)
            .sized_body(cal.len(), Cursor::new(cal))
            .ok()
    }
}
