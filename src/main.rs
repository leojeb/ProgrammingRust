#[macro_use]
extern crate rocket;
// extern crate serde;
extern crate serde_json;

use rocket::serde::json::{Json, Value};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct TtyRequestBody {
    username: String,
    password: String,
    remote_id: i32,
}

#[derive(Serialize)]
struct TtyResponseBody {
    code: i32,
    data: TtyData,
    msg: String,
}

#[derive(Serialize)]
struct TtyData {
    target_device_ip: String,
    target_device_port: i32,
}


#[post("/remote/v1/start_tty_remote", format = "json", data = "<tty_request_body>")]
fn start_tty_remote(tty_request_body: Json<TtyRequestBody>) -> Json<TtyResponseBody> {
    let tty_data = TtyData {
        target_device_ip: "192.168.1.1".to_string(),
        target_device_port: 8080,
    };
    let tty_response_body = TtyResponseBody {
        code: 0,
        data: tty_data,
        msg: "start_tty_remote success".to_string(),
    };
    Json(tty_response_body)
}

#[post("/remote/v1/end_tty_remote", format = "json", data = "<tty_request_body>")]
fn end_tty_remote(tty_request_body: Json<TtyRequestBody>) -> Json<TtyResponseBody> {
    let tty_data = TtyData {
        target_device_ip: "192.168.1.1".to_string(),
        target_device_port: 8080,
    };
    let tty_response_body = TtyResponseBody {
        code: 0,
        data: tty_data,
        msg: "end_tty_remote success".to_string(),
    };
    Json(tty_response_body)
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![start_tty_remote, end_tty_remote])
        .launch()
        .await
        .unwrap();
}

