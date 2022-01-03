//削除したい投稿のidを受け取り、削除
extern crate app;    //srcファイル配下が使用できるようになる
extern crate diesel;

use self::diesel::prelude::*;
use self::app::*;
use std::env::args;

pub fn delete_post<'a>(delete_id:&'a i32){
    use app::schema::posts::dsl::*;

    let connection=establish_connection();

    let delete=diesel::delete(posts.filter(id.eq(delete_id)))   //delete文 like文で対象となるtitleのデータを削除
        .execute(&connection)
        .expect("The id does not exist");
}
