use crate::errors::AppError;
use crate::routes::convert;
use crate::{models, Pool};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use futures::Future;

#[derive(Debug, Serialize, Deserialize)]
struct PostInput {
    title: String,
    body: String,
}

// Adds a new post linked to a specific user.
//
// Takes the user ID, the new post data (in JSON format), and the database connection pool.
// It then attempts to find the user in the database, and if successful, creates a new post
// with the provided title and body, linked to the found user.
//
// This function returns a Future which upon resolution will contain an `HttpResponse` in case
// of success or an `AppError` in case of failure.
fn add_post(
    user_id: web::Path<i32>,
    post: web::Json<PostInput>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn: &SqliteConnection = &pool.get().unwrap();
        let key = models::UserKey::ID(user_id.into_inner());
        models::find_user(conn, key).and_then(|user| {
            let post = post.into_inner();
            let title = post.title;
            let body = post.body;
            models::create_post(conn, &user, title.as_str(), body.as_str())
        })
    })
    .then(convert)
}

fn publish_post(
    post_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn: &SqliteConnection = &pool.get().unwrap();
        models::publish_post(conn, post_id.into_inner())
    })
    .then(convert)
}

fn user_posts(
    user_id: web::Path<i32>,
    pool: web::Data<Pool>,
) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn: &SqliteConnection = &pool.get().unwrap();
        models::user_posts(conn, user_id.into_inner())
    })
    .then(convert)
}

fn all_posts(pool: web::Data<Pool>) -> impl Future<Item = HttpResponse, Error = AppError> {
    web::block(move || {
        let conn: &SqliteConnection = &pool.get().unwrap();
        models::all_posts(conn)
    })
    .then(convert)
}


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users/{id}/posts")
            .route(web::post().to_async(add_post))
            .route(web::post().to_async(user_posts))
    )
    .service(web::resource("/posts").route(web::get().to_async(all_posts)))
    .service(web::resource("/posts/{id}/publish").route(web::post().to_async(publish_post)));
}
