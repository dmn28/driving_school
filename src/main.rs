use std::sync::Arc;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;
use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use chrono::{Local, Timelike};
use tokio::sync::RwLock;
use tokio::time::sleep;
use crate::models::drivingschool::DrivingSchool;
use serde_json::json;
use crate::models::student::{Student, StudentInput};

mod models;
mod handler;

struct AppState {
    state: RwLock<DrivingSchool>,
    id_counter: u8,
}

#[tokio::main]
async fn main() {
    println!("Server started");

    let app_state = Arc::new(AppState { state: RwLock::new(DrivingSchool::new()), id_counter: 0 });
    let app_state_clone = app_state.clone();

    // let async_task = tokio::spawn(async move {
    //     loop {
    //         let now = Local::now().naive_local();
    //         let midnight = now.date().succ_opt().unwrap().and_hms_opt(0, 0, 0).unwrap();
    //         let duration_until_midnight = midnight - now;
    //
    //         // sleep(Duration::from_secs(duration_until_midnight.num_seconds().unsigned_abs())).await;
    //         sleep(Duration::from_secs(3)).await;
    //
    //         //TODO refactor after Arc added
    //         // driving_school.read().await.print();
    //         app_state.state.read().await.print();
    //     }
    // });

    // println!("Cleanup task scheduled");
    // async_task.await.expect("async task failed");

    let app = Router::new()
        .route("/students", get(list_students))
        .route("/students/:id", get(get_student_by_id))
        .route("/students", post(add_student))
        .with_state(app_state);


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();


    println!("End of program");
}

async fn get_student_by_id(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {}

async fn list_students(
    State(state): State<Arc<AppState>>
) -> impl IntoResponse {
    let students = &state.state.read().await.students;
    for student in students {
        println!("{}", student);
    }
    println!("===========================");
    println!("{}", &state.id_counter);
    (StatusCode::OK, Json(json!(students)))
    // (StatusCode::OK, Json(json!({"ok":"1"})))
}

async fn add_student(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<StudentInput>,
) -> impl IntoResponse {
    let student = Student::new(
        payload.name,
        payload.date_of_birth,
        payload.has_id,
        payload.passed_eye_test,
    );


    state.state.write().await.add_student(student.unwrap());
    // state.id_counter = state.id_counter +  1;

    for student in &state.state.read().await.students {
        println!("{}", student);
    }
    println!("====================================");

    (StatusCode::OK, "Student added")
}

#[cfg(test)]
mod tests {}
