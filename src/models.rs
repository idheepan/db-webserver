use rocket::serde::{Deserialize, Serialize};

use diesel::{Insertable, Queryable};

use crate::schema::data;
use chrono::{DateTime, Utc};
#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct SensorReading {
    pub id: i32,
    pub ts: DateTime<Utc>,
    pub sensor: i32,
    pub temperature: f32,
    pub rhumidity: f32,
}

#[derive(Deserialize, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "data"]
pub struct NewSensorReading {
    pub ts: DateTime<Utc>,
    pub sensor: i32,
    pub temperature: f32,
    pub rhumidity: f32,
}
// #[derive(Deserialize, AsChangeset, Debug)]
// #[serde(crate = "rocket::serde")]
// #[table_name = "sensor_readings_table"]
// pub struct UpdatedArtist {
//     pub name: Option<String>,
//     pub description: Option<String>,
// }
