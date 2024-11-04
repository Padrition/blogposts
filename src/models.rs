use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::blogpost;

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = blogpost)]
pub struct BlogPost{
    pub id: i32,
    pub text: String,
    pub publication_date: NaiveDate,
    pub post_image_path: Option<String>,
    pub username: String,
    pub avatar_path: Option<String>,

}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = blogpost)]
pub struct NewBlogPost{
    pub text: String,
    pub publication_date: NaiveDate,
    pub post_image_path: Option<String>,
    pub username: String,
    pub avatar_path: Option<String>,
}