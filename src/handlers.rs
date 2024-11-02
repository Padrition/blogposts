use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use deadpool_diesel::postgres::Pool;
use diesel::prelude::*;

use crate::models::{BlogPost, NewBlogPost};

pub fn blog_post_routers(pool: Pool) -> Router{
    Router::new()
        .route("/blogpost", post(create_blog_post))
        .with_state(pool)
}

async fn create_blog_post(
    State(pool) : State<deadpool_diesel::postgres::Pool>,
    Json(new_blog_post): Json<NewBlogPost>,
) -> Result<Json<BlogPost>, (StatusCode,String)> {
    let conn = pool.get().await.map_err(internal_error)?;
    let res = conn
        .interact(|conn|{
            diesel::insert_into(crate::schema::blogpost::table)
            .values(new_blog_post)
            .returning(BlogPost::as_returning())
            .get_result(conn)
        })
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