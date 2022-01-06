mod modules;   //こう宣言するとmodules.rsもしくはmodules/mod.rsを探しに行く
use std::io::{stdin,Read};

fn main(){
    //全体の状態遷移管理フラグ
    let mut global_flug=String::new();

    let show_posts_flug="01";
    let write_post_flug="02";
    let delete_post_flug="03";
    let publish_post_flug="04";
    let exit_flug="05";



    loop{

        //トップページ

        if global_flug == "0"{
            println!("\n1.show_posts");
            println!("2.new_post");
            println!("3.delete_post");
            println!("4.publish_post");
            println!("5.exit");

            stdin().read_line(&mut global_flug).unwrap();
            global_flug=(global_flug[..(global_flug.len()-1)]).to_string();

        }else if global_flug==show_posts_flug{
            modules::show_posts::show_posts();
            global_flug=String::from("0");

        }else if global_flug==write_post_flug{
            modules::write_post::write_post();
            global_flug=String::from("0");

        }else if global_flug==delete_post_flug{

            let mut delete_id=String::from("");
            let mut delete_flug=false;

            modules::show_posts::show_posts();

            println!("Please enter the id of the post you want to delete. ");
            stdin().read_line(&mut delete_id).unwrap();
            delete_id=(delete_id[..(delete_id.len()-1)]).to_string();

            let delete_id_result=delete_id.parse::<i32>();

            if delete_id_result.is_err(){
                println!("Please enter a number");
            }else{
                let delete_id_int=delete_id_result.unwrap();
                modules::delete_post::delete_post(&delete_id_int);
            }


            //modules::delete_post::delete_post(&delete_id_result);
            global_flug=String::from("0");

        }else if global_flug==publish_post_flug{
            modules::show_private_posts::show_private_posts();
            let mut publish_id=String::new();
            stdin().read_line(&mut publish_id);
            publish_id=(publish_id[..(publish_id.len()-1)]).to_string();

            let publish_id_result=publish_id.parse::<i32>();

            if publish_id_result.is_err(){
                println!("Please enter a number");
            }else{
                let publish_id_int=publish_id_result.unwrap();
                modules::publish_post::publish_post(&publish_id_int);
            }
            global_flug=String::from("0");

        }else if global_flug==exit_flug{
            break;
        }else{
            //println!("{}",global_flug);
            global_flug=String::from("0");
        }
    }



    let b=&b[..(b.len()-1)];    標準入力から値を受け取った場合、最後に打つenterも考慮しなければならない。文字列の最後の文字を含まないようにする文。

