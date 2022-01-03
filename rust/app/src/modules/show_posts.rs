extern crate app;
extern crate diesel;

use self::app::*;
use self::models::*;
use self::diesel::prelude::*;

pub fn show_posts(){
    use app::schema::posts::dsl::*;   //postsやpublishedが使えるようになる

    let connection=establish_connection();  //mysqlに接続するためのオブジェクトを作る
    let results=posts.filter(published.eq(true))
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts",results.len());
    for post in results{
        println!("id-{}:title-{}",post.id,post.title);
        println!("------------");
        println!("{}",post.body);
    }
}