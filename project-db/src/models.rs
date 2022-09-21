use super::schema::*;

use diesel::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};


#[derive(Identifiable, Queryable, PartialEq, Debug)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub directory: String,
    pub start: NaiveDate,
    pub end: Option<NaiveDate>,
}

#[derive(Insertable)]
#[table_name = "projects"]
pub struct NewProject<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub directory: &'a str,
    pub start: NaiveDate,
    pub end: Option<NaiveDate>

}

#[derive(Identifiable, Queryable, Insertable, PartialEq, Debug)]
#[table_name = "aircrafts"]
pub struct Aircraft {
    pub id: i32,
    pub name: String,
    pub registration: Option<String>
}

#[derive(Identifiable, Queryable, Insertable, PartialEq, Debug)]
#[table_name = "pilots"]
pub struct Pilot {
    pub id: i32,
    pub name: String,
    pub registration: Option<String>
}

#[derive(Associations, Identifiable, Queryable, Insertable, PartialEq, Debug)]
#[belongs_to(Project)]
#[table_name = "locations"]
pub struct Location {
    pub id: i32,
    pub name: String,
    pub kml: String,
    pub project_id: i32
}


#[derive(Associations, Identifiable, Queryable, Insertable, PartialEq, Debug)]
#[belongs_to(Location)]
#[belongs_to(Pilot)]
#[belongs_to(Aircraft)]
#[table_name = "flights"]
pub struct Flight {
    pub id: i32,
    pub name: String,
    pub datasets: i32,
    pub location_id: i32,
    pub aircraft_id: i32,
    pub pilot_id: i32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
    pub speed_ms: i32,
    pub height_m: i32,
}

#[derive(Associations, Identifiable, Queryable, Insertable, PartialEq, Debug)]
#[belongs_to(Flight)]
#[table_name = "datasets"]
pub struct Dataset {
    pub id: i32,
    pub flight_number: i32,
    pub flight_id: i32,
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

// #[derive(Associations, Identifiable, Queryable, Insertable, PartialEq, Debug)]
// #[belongs_to(Flight)]
// #[table_name = "weathers"]
// pub struct Weather {
//     pub id: i32,
//     pub flight_id: i32,
//     pub temperature: f32,
//     pub humidity: f32,
//     pub wind: f32,
//     pub description: String,
//     pub source: String,
//     pub time: NaiveDateTime,
// }
