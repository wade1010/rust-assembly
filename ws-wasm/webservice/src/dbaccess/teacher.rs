use crate::errors::MyError;
use crate::models::teacher::*;
use sqlx::postgres::PgPool;

pub async fn get_all_teachers_db(pool: &PgPool) -> Result<Vec<Teacher>, MyError> {
    let rows = sqlx::query!("select * from teacher")
        .fetch_all(pool)
        .await?;
    let teachers: Vec<Teacher> = rows
        .iter()
        .map(|t| Teacher {
            id: t.id,
            name: t.name.clone().unwrap_or_default(),
            picture_url: t.picture_url.clone().unwrap_or_default(),
            profile: t.profile.clone().unwrap_or_default(),
        })
        .collect();
    match teachers.len() {
        0 => Err(MyError::NotFound("No teacher found".into())),
        _ => Ok(teachers),
    }
}

pub async fn get_teacher_details_db(pool: &PgPool, teacher_id: i32) -> Result<Teacher, MyError> {
    let row = sqlx::query!("select * from teacher where id = $1", teacher_id)
        .fetch_one(pool)
        .await
        .map(|t| Teacher {
            id: t.id,
            name: t.name.unwrap_or_default(),
            picture_url: t.picture_url.unwrap_or_default(),
            profile: t.profile.unwrap_or_default(),
        })
        .map_err(|_| MyError::NotFound("Teacher id not found".into()))?;
    Ok(row)
}
pub async fn post_new_teacher_db(
    pool: &PgPool,
    new_teacher: CreateTeacher,
) -> Result<Teacher, MyError> {
    let row = sqlx::query!(
        "insert into teacher (name,picture_url,profile) values($1, $2, $3)
        returning id,name,picture_url,profile",
        new_teacher.name,
        new_teacher.picture_url,
        new_teacher.profile
    )
    .fetch_one(pool)
    .await?;
    Ok(Teacher {
        id: row.id,
        name: row.name.unwrap_or_default(),
        picture_url: row.picture_url.unwrap_or_default(),
        profile: row.profile.unwrap_or_default(),
    })
}
pub async fn update_teacher_details_db(
    pool: &PgPool,
    teacher_id: i32,
    update_teacher: UpdateTeacher,
) -> Result<Teacher, MyError> {
    let row = sqlx::query!("select * from teacher where id = $1", teacher_id)
        .fetch_one(pool)
        .await
        .map_err(|_| MyError::NotFound("Teacher id not found".into()))?;
    let temp = Teacher {
        id: row.id,
        name: if let Some(name) = update_teacher.name {
            name
        } else {
            row.name.unwrap_or_default()
        },
        picture_url: if let Some(picture_url) = update_teacher.picture_url {
            picture_url
        } else {
            row.picture_url.unwrap_or_default()
        },
        profile: if let Some(profile) = update_teacher.profile {
            profile
        } else {
            row.profile.unwrap_or_default()
        },
    };
    let update_row = sqlx::query!(
        "update teacher set name = $1,picture_url = $2,profile = $3 where id = $4
        returning id,name,picture_url,profile",
        temp.name,
        temp.picture_url,
        temp.profile,
        teacher_id
    )
    .fetch_one(pool)
    .await
    .map(|t| Teacher {
        id: t.id,
        name: t.name.unwrap_or_default(),
        picture_url: t.picture_url.unwrap_or_default(),
        profile: t.profile.unwrap_or_default(),
    })
    .map_err(|_| MyError::NotFound("Teacher id not found".into()))?;
    Ok(update_row)
}

pub async fn delete_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<String, MyError> {
    let row = sqlx::query(&format!("delte from teacher where id = {}", teacher_id))
        .execute(pool)
        .await
        .map_err(|_| MyError::NotFound("Unable to delete teacher".into()))?;
    Ok(format!("Deleted {:?}", row))
}
