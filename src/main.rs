#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{http::Status, response::status::Custom};

// Struktur input JSON
#[derive(Deserialize)]
struct InputData {
    total_mass: f64,        // Massa total campuran (gram)
    percentage_nitro: f64,  // Persentase Shell Nitro+ (0-1, misal 0.8)
    percentage_m5: f64,     // Persentase M5 (0-1, misal 0.2)
}

// Struktur output JSON
#[derive(Serialize)]
struct VolumeResponse {
    volume_nitro: f64, // Volume Shell Nitro+ dalam ml
    volume_m5: f64,    // Volume M5 dalam ml
    total_volume: f64, // Total volume bahan bakar
    lube_volume: f64,  // Volume lube dalam ml
}

// Struktur untuk respons error JSON
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

// Endpoint POST untuk menghitung volume dan lube
#[post("/calculate_volume", format = "json", data = "<input>")]
fn calculate_volume(input: Json<InputData>) -> Result<Json<VolumeResponse>, Custom<Json<ErrorResponse>>> {
    // Validasi total persentase harus 100%
    if (input.percentage_nitro + input.percentage_m5 - 1.0).abs() > f64::EPSILON {
        let error_response = ErrorResponse {
            error: "Persentase Shell Nitro+ dan M5 harus berjumlah 100%.".to_string(),
        };
        return Err(Custom(Status::BadRequest, Json(error_response)));
    }

    // Densitas bahan bakar
    let density_nitro = 0.74; // g/ml
    let density_m5 = 0.80;    // g/ml

    // Hitung massa masing-masing bahan bakar
    let mass_nitro = input.total_mass * input.percentage_nitro;
    let mass_m5 = input.total_mass * input.percentage_m5;

    // Hitung volume masing-masing bahan bakar
    let volume_nitro = mass_nitro / density_nitro;
    let volume_m5 = mass_m5 / density_m5;

    // Total volume bahan bakar
    let total_volume = volume_nitro + volume_m5;

    // Hitung volume lube (5 ml per 1000 ml)
    let lube_volume = total_volume * (5.0 / 1000.0);

    // Kembalikan hasil dalam format JSON
    Ok(Json(VolumeResponse {
        volume_nitro,
        volume_m5,
        total_volume,
        lube_volume,
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![calculate_volume])
}
