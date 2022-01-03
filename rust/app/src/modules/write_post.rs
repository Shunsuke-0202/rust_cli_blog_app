extern crate app;
extern crate diesel;

use self::app::*;
use std::io::{stdin,Read};

pub fn write_post(){
    let connection=establish_connection();

    println!("What would you like your title to be");
    let mut title=String::new();
    stdin().read_line(&mut title).unwrap();
    let title=&title[..(title.len()-1)];
    println!("\nOk! Let's write {} (Press {} when finished)\n",title,EOF);
    let mut body=String::new();
    stdin().read_to_string(&mut body).unwrap();

    create_post(&connection,title,&body);
    //println!("\nSaved draft {} with id {}",title,post.id);
}

#[cfg(not(windows))]
const EOF:&'static str ="CTRL+D";

#[cfg(windows)]
const EOF:&'static str="CTRL+Z";