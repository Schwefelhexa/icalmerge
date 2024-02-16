use std::io::Cursor;

use ics::{properties::Name, ICalendar};
use rocket::{get, http::ContentType, launch, response::Responder, routes, Response};

#[get("/")]
fn index() -> Cal<'static> {
    let mut cal = ICalendar::new("2.0", format!("-//Alexander Baron//iCal merge v{}//EN", env!("CARGO_PKG_VERSION")));
    cal.push(Name::new("Merged Calendar"));


    Cal(cal)
}

#[launch]
fn rocket() -> _ {
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
