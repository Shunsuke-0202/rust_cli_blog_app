extern crate app;
extern crate diesel;

use self::diesel::prelude::*;
use self::app::*;
use self::models::Post;
use std::env::args;

pub fn publish_post<'a>(publish_id:&'a i32){
    use app::schema::posts::dsl::{posts,published,id};

    let connection=establish_connection();

    let post=diesel::update(posts.filter(id.eq(publish_id)))                     // SQLのWHERE
        .set(published.eq(true))                                //.setでpublishedにtrueに変える
        .execute(&connection)                                   //.executeで実行
        .expect("Error");
}