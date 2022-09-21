#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use chrono::NaiveDate;

pub mod schema;
pub mod models;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db() {
        let connection = SqliteConnection::establish("test.db").expect("Failed to open conn");

        let project = models::NewProject {
            name: "Easten Wheat",
            description: "A mapping project of 3 distinct fields at ARDEC",
            directory: "a/place/that/exists",
            start: NaiveDate::from_ymd(2020, 1, 1),
            end: Some(NaiveDate::from_ymd(2020, 1, 25))
        };

        diesel::insert_into(schema::projects::table)
            .values(&project)
            .execute(&connection)
            .expect("FAIL");

    }
}
