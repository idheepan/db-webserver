use chrono::DateTime;

use rocket::{
    http::Method,
    response::status::{Created, NoContent, NotFound},
    serde::json::Json,
};
use rocket_cors::{AllowedHeaders, AllowedOrigins};

use diesel::prelude::*;

use db_webserver::{
    models::{NewSensorReading, NewSensorReadingEntry, SensorReading},
    schema::data_en,
    ApiError, PgConnection,
};

use psychrometry::psychrolib::{self, PsychrometryFunctions, UnitIndependent};
use rocket::State;
const AMB_PRESSURE: f64 = 101325_f64;
// TODO: Retrieve a list of sensor readings between given times for one specific sensor.
#[rocket::launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:8001"]);
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::build()
        // States
        .manage(psychrolib::SI {})
        // Fairings
        .attach(PgConnection::fairing())
        // Routes
        .mount(
            "/sensor-data",
            rocket::routes![
                retrieve_some,
                retrieve_one,
                create,
                // update,
                // destroy
            ],
        )
        .attach(cors)
}

// FIXME: Time parse cleanup or return useful info instead of panicking and burning.
#[rocket::get("/retreive?<startts>&<endts>")]
async fn retrieve_some(
    connection: PgConnection,
    startts: String,
    endts: String,
) -> Result<Json<Vec<SensorReading>>, NoContent> {
    let start_time = DateTime::parse_from_rfc3339(startts.as_str()).unwrap(); //TODO: Handle panic.
    let end_time = DateTime::parse_from_rfc3339(endts.as_str()).unwrap();

    connection
        .run(move |c| {
            data_en::table
                .filter(data_en::ts.between(start_time, end_time))
                .load(c)
        })
        .await
        .map(Json)
        .map_err(|_e| NoContent)
}

#[rocket::get("/<id>")]
async fn retrieve_one(
    connection: PgConnection,
    id: i32,
) -> Result<Json<SensorReading>, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| data_en::table.filter(data_en::id.eq(id)).first(c))
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}
//
// TODO: When adding a new sensor reading calculate its enthalpy and add it to the column
#[rocket::post("/", data = "<sensor_reading>")]
async fn create(
    state: &State<psychrolib::SI>,
    connection: PgConnection,
    sensor_reading: Json<NewSensorReading>,
) -> Result<Created<Json<SensorReading>>, Json<ApiError>> {
    let incoming = sensor_reading.into_inner();
    let temperature = incoming.temperature;
    let rhumidity = incoming.rhumidity;
    let hum_ratio = state
        .get_hum_ratio_from_rel_hum(temperature as f64, rhumidity as f64, AMB_PRESSURE)
        .unwrap();
    let enthalpy = state
        .get_moist_air_enthalpy(temperature as f64, hum_ratio)
        .unwrap() as f32;

    let new_reading = NewSensorReadingEntry {
        ts: incoming.ts,
        sensor: incoming.sensor,
        temperature,
        rhumidity,
        enthalpy,
    };
    connection
        .run(move |c| {
            diesel::insert_into(data_en::table)
                .values(&new_reading)
                .get_result(c)
        })
        .await
        .map(|a| Created::new("/").body(Json(a)))
        .map_err(|e| {
            Json(ApiError {
                details: e.to_string(),
            })
        })
}
