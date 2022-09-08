use std::convert::TryInto;

use crate::errors::MyError;
use crate::state::AppState;
use crate::{dbaccess::course::*, models::course::*};
use actix_web::{web, HttpResponse};

pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_course_db(&app_state.db, new_course.try_into()?)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    get_courses_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    get_course_detail_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}
pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    delete_course_db(&app_state.db, teacher_id, course_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}
pub async fn update_course_details(
    app_state: web::Data<AppState>,
    update_course: web::Json<UpdateCourse>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = params.into_inner();
    update_course_details_db(&app_state.db, teacher_id, course_id, update_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
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
        let course = web::Json(CreateCourse {
            teacher_id: 1,
            // id: None,//之前存内存里面的
            name: "Test course".into(),
            description: "This is a course".into(),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("English".into()),
            level: Some("Beginner".into()),
        });
        let resp = post_new_course(course, app_state).await.unwrap();
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
        let teacher_id: web::Path<i32> = web::Path::from(1);
        let resp = get_courses_for_teacher(app_state, teacher_id)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_one_course_success() {
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

    #[actix_rt::test]
    async fn test_get_one_course_failure() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 100));
        let resp = get_course_detail(app_state, params).await;
        match resp {
            Ok(_) => println!("Something wrong...."),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }

    #[actix_rt::test]
    async fn update_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let update_course = UpdateCourse {
            name: Some("Course Name changed".into()),
            description: Some("Course Description changed".into()),
            format: None,
            level: Some("Course Level".into()),
            price: None,
            duration: None,
            language: Some("Course Language".into()),
            structure: None,
        };
        let params: web::Path<(i32, i32)> = web::Path::from((1, 2));
        let update_param = web::Json(update_course);
        let resp = update_course_details(app_state, update_param, params)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[ignore]
    #[actix_rt::test]
    async fn delete_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 3));
        let resp = delete_course(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[ignore]
    #[actix_rt::test]
    async fn delete_course_failure() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 101));
        let resp = delete_course(app_state, params).await;
        match resp {
            Ok(_) => println!("Something wrong..."),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
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
