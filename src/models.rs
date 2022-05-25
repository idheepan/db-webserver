use rocket::serde::{Deserialize, Serialize};

use diesel::{Insertable, Queryable};

use crate::schema::data_en;
use chrono::{DateTime, Utc};
#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SensorReading {
    pub id: i32,
    pub ts: DateTime<Utc>,
    pub sensor: i32,
    pub temperature: f32,
    pub rhumidity: f32,
    pub enthalpy: f32,
}

// FIXME: The program already has all the data for this entry in other data structures
// Making unnecessary copies here. Would be better to insert the references. I don't know
// how to implement that with lifetimes.
#[derive(Deserialize, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "data_en"]
pub struct NewSensorReadingEntry {
    pub ts: DateTime<Utc>,
    pub sensor: i32,
    pub temperature: f32,
    pub rhumidity: f32,
    pub enthalpy: f32,
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct NewSensorReading {
    pub ts: DateTime<Utc>,
    pub sensor: i32,
    pub temperature: f32,
    pub rhumidity: f32,
}
