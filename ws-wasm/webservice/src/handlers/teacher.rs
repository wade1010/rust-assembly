use crate::dbaccess::teacher::*;
use crate::errors::MyError;
use crate::models::teacher::*;
use crate::state::AppState;

use actix_web::{web, HttpResponse};
pub async fn get_all_teachers(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    get_all_teachers_db(&app_state.db)
        .await
        .map(|teachers| HttpResponse::Ok().json(teachers))
}

pub async fn get_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    get_teacher_details_db(&app_state.db, teacher_id)
        .await
        .map(|t| HttpResponse::Ok().json(t))
}

pub async fn post_new_teacher(
    new_teacher: web::Json<CreateTeacher>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_teacher_db(&app_state.db, new_teacher.into())
        .await
        .map(|t| HttpResponse::Ok().json(t))
}

pub async fn update_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
    update_teacher: web::Json<UpdateTeacher>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    update_teacher_details_db(&app_state.db, teacher_id, update_teacher.into())
        .await
        .map(|t| HttpResponse::Ok().json(t))
}

pub async fn delete_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    delete_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|msg| HttpResponse::Ok().json(msg))
}
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn test_post_new_teacher() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL cannot be empty");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let new_teacher: web::Json<CreateTeacher> = web::Json(CreateTeacher {
            name: "teacherA".to_string(),
            picture_url: "picture_url".to_string(),
            profile: "profile".to_string(),
        });
        let resp = post_new_teacher(new_teacher, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn test_get_all_teachers_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL cannot be empty");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let resp = get_all_teachers(app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[actix_rt::test]
    async fn test_get_tutor_detail_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL cannot be empty");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params = web::Path::from(1);
        let resp = get_teacher_details(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
