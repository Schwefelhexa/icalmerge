use std::{env, io::Cursor};

use ics::{properties::Name, ICalendar};
use itertools::Itertools;
use log::info;
use rocket::{get, http::ContentType, launch, response::Responder, routes, Response};

#[get("/")]
fn index() -> Cal<'static> {
    let mut cal = ICalendar::new("2.0", format!("-//Alexander Baron//iCal merge v{}//EN", env!("CARGO_PKG_VERSION")));
    cal.push(Name::new("Merged Calendar"));

    let sources_raw = env::var("ICAL_SOURCES").unwrap_or_default();
    let sources = sources_raw.split(',').collect_vec();
    info!("Sources: {:?}", sources);

    Cal(cal)
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
