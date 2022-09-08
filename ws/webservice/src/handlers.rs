use crate::error::MyError;

use super::db_access::*;
use super::state::AppState;
use actix_web::{web, HttpResponse};
use std::convert::TryFrom;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

use super::models::Course;
// use chrono::Utc;

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_course_db(&app_state.db, new_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = i32::try_from(params.0).unwrap();
    get_courses_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = i32::try_from(params.0).unwrap();
    let course_id = i32::try_from(params.1).unwrap();
    get_course_detail_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
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
    async fn post_course_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let course = web::Json(Course {
            teacher_id: 1,
            // id: None,//之前存内存里面的
            id: Some(5), //数据库是serial类型，所以设置为None,有问题
            name: "Test course".into(),
            time: None,
        });
        let resp = new_course(course, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_all_courses_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let teacher_id: web::Path<(i32,)> = web::Path::from((1,));
        let resp = get_courses_for_teacher(app_state, teacher_id)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_course_detail() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let resp = get_course_detail(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
// pub async fn new_course(
//     new_course: web::Json<Course>,
//     app_state: web::Data<AppState>,
// ) -> HttpResponse {
//     println!("Received new course");
//     let course_count = app_state.courses.lock().unwrap()
//         .clone().into_iter().filter(|course| course.teacher_id == new_course.teacher_id)
//         .collect::<Vec<Course>>().len();
//     let new_course = Course {
//         teacher_id: new_course.teacher_id,
//         id: Some(course_count + 1),
//         name: new_course.name.clone(),
//         time: Some(Utc::now().naive_utc()),
//     };
//     app_state.courses.lock().unwrap().push(new_course);
//     HttpResponse::Ok().json("Course added")
// }

// pub async fn get_courses_for_teacher(app_state: web::Data<AppState>, params: web::Path<(i32,)>) -> HttpResponse {
//     let teacher_id = params.0;
//     let filtered_courses = app_state.courses.lock().unwrap()
//         .clone().into_iter().filter(|course| course.teacher_id == teacher_id)
//         .collect::<Vec<Course>>();
//     if filtered_courses.len() > 0 {
//         HttpResponse::Ok().json(filtered_courses)
//     } else {
//         HttpResponse::Ok().json("No courses found for teacher".to_string())
//     }
// }

// pub async fn get_course_detail(
//     app_state: web::Data<AppState>,
//     params: web::Path<(i32, i32)>,
// ) -> HttpResponse {
//     let (teacher_id, course_id) = params.0;
//     let selected_course = app_state.courses.lock().unwrap().clone()
//         .into_iter().find(|x| x.teacher_id == teacher_id && x.id == Some(course_id))
//         .ok_or("Course not found");
//     match selected_course {
//         Ok(course) => HttpResponse::Ok().json(course),
//         Err(err) => HttpResponse::Ok().json(err)
//     }
//     // if let Ok(course) = selected_course {
//     //     HttpResponse::Ok().json(course)
//     // } else {
//     //     HttpResponse::Ok().json("Course not found".to_string())
//     // }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::http::StatusCode;
//     use std::sync::Mutex;

//     #[actix_rt::test]
//     async fn post_course_test() {
//         let course = web::Json(Course {
//             teacher_id: 1,
//             name: "Test course".into(),
//             id: None,
//             time: None,
//         });
//         let app_state: web::Data<AppState> = web::Data::new(AppState {
//             health_check_response: "".to_string(),
//             visit_count: Mutex::new(0),
//             courses: Mutex::new(vec![]),
//         });
//         let resp = new_course(course, app_state).await;
//         assert_eq!(resp.status(), StatusCode::OK);
//     }

//     #[actix_rt::test]
//     async fn test_get_all_courses_success() {
//         let app_state: web::Data<AppState> = web::Data::new(AppState {
//             health_check_response: "".to_string(),
//             visit_count: Mutex::new(0),
//             courses: Mutex::new(vec![]),
//         });
//         let teacher_id: web::Path<(i32,)> = web::Path::from((1,));
//         let resp = get_courses_for_teacher(app_state, teacher_id).await;
//         assert_eq!(resp.status(), StatusCode::OK);
//     }

//     #[actix_rt::test]
//     async fn test_get_course_detail() {
//         let app_state: web::Data<AppState> = web::Data::new(AppState {
//             health_check_response: "".to_string(),
//             visit_count: Mutex::new(0),
//             courses: Mutex::new(vec![]),
//         });
//         let params = web::Path::from((1, 1));
//         let resp = get_course_detail(app_state, params).await;
//         assert_eq!(resp.status(), StatusCode::OK);
//     }
// }
