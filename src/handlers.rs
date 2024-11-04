use axum::extract::Multipart;
use axum::routing::get;
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use chrono::NaiveDate;
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::blogpost::{self};
use crate::models::{BlogPost, NewBlogPost};
use std::fs;

pub fn blog_post_routers(pool: Pool) -> Router{
    Router::new()
        .route("/blogpost", post(create_blog_post))
        .route("/blogposts", get(read_all_blog_posts))
        .with_state(pool)
}

async fn create_blog_post(
    State(pool) : State<deadpool_diesel::postgres::Pool>,
    mut multipart: Multipart
) -> Result<Json<BlogPost>, (StatusCode,String)> {
    let mut new_blog_post = NewBlogPost{
        text: String::new(),
        publication_date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
        post_image_path: None,
        username: String::new(),
        avatar_path: None,
    };

    while let Some(field) = multipart.next_field().await.map_err(internal_error)? {
        match field.name() {
            Some("text") => {
                new_blog_post.text = field.text().await.map_err(internal_error)?
            },
            Some("publication_date") => {
                let date_str = field.text().await.map_err(internal_error)?;
                new_blog_post.publication_date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
                    .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))?;
            }
            Some("username") => {
                new_blog_post.username = field.text().await.map_err(internal_error)?;
            },
            Some("post_image") =>{
                let file_name = format!("{}.png", Uuid::new_v4());
                let file_path = format!("/media/{}", &file_name);

                let data = field.bytes().await.map_err(internal_error)?;
                fs::write(&file_path, &data).map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

                new_blog_post.post_image_path = Some(file_path);
            },
            Some("avatar_path") =>{
                let url = field.text().await.map_err(internal_error)?;
                
                let response = reqwest::get(url)
                .await
                .map_err(internal_error)?;
                
                let data = response.bytes().await.map_err(internal_error)?;
                
                match imghdr::from_bytes(&data){
                    Some(imghdr::Type::Png) => {
                        let file_name = format!("avatar_{}.png", Uuid::new_v4());
                        let file_path = format!("/media/{}", &file_name);

                        fs::write(&file_path, &data).map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

                        new_blog_post.avatar_path = Some(file_path);
                    },
                    _ => return Err((StatusCode::BAD_REQUEST, "Not an PNG".to_string())),
                }

            }
            _ => {}
            
        }
    }

    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn|{
            diesel::insert_into(blogpost::table)
            .values(new_blog_post)
            .returning(BlogPost::as_returning())
            .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(res))
}

async fn read_all_blog_posts(
    State(pool) : State<deadpool_diesel::postgres::Pool>
) -> Result<Json<Vec<BlogPost>>, (StatusCode, String)>{
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn| blogpost::table.select(BlogPost::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;

    Ok(Json(res))
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}