pub mod schema;         //schema.rsを読み込む
pub mod models;         //models.rsを読み込む   modの内容を使用する時はuseで宣言しないと使えない 例:13行目

#[macro_use]      //macroのインポート
extern crate diesel;        //models.rsの#[derive(Inserttable)]等のアトリビュートをつける為に必要
extern crate dotenv;        //.envファイルを読み込む為に必要

use diesel::prelude::*;         //21行目のestablish等に必要
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;

use self::models::{Post,NewPost};

pub fn establish_connection()->MysqlConnection{
    dotenv().ok();  //.envファイルを読み込む

    let database_url=env::var("DATABASE_URL")   //.envの内容を読み込む
        .expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}",database_url))   //database_urlでPgConnection::establishオブジェクトを作る
}

pub fn create_post<'a>(conn:&MysqlConnection,title:&'a str,body:&'a str){
    use schema::posts;

    let new_post=NewPost{
        title:title,
        body:body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");
}