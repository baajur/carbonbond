use std::io::stdin;

use carbon_bond::db;
use carbon_bond::send_invite_email;
use db::schema::users::dsl::*;
use diesel::prelude::*;
use db::models::*;

fn main() -> std::io::Result<()> {
    println!("碳鍵 - 資料庫管理介面");
    let db_conn = db::connect_db();
    let mut opt = 1;
    let p1 = "[0] 結束程式";
    let p2 = "[1] 新增使用者";
    let p3 = "[2] 檢視使用者名單";
    let p4 = "[3] 寄出邀請信";
    let p5 = "[4] 清空資料庫";
    while opt != 0 {
        println!("{}", p1);
        println!("{}", p2);
        println!("{}", p3);
        println!("{}", p4);
        println!("{}", p5);
        let mut buff = String::new();
        stdin().read_line(&mut buff)?;
        if let Ok(_opt) = buff.replace("\n", "").parse::<u8>() {
            opt = _opt;
            if opt == 0 {
                break;
            } else if opt == 1 {
                loop {
                    println!("> {}", p2);
                    println!("> 請輸入 名字 密碼，或輸入空白行回到選單");
                    buff.clear();
                    stdin().read_line(&mut buff)?;
                    let words: Vec<&str> = buff.split_whitespace().collect();
                    if words.len() == 0 {
                        break;
                    } if words.len() != 2 {
                        println!("輸入格式錯誤");
                    } else {
                        db::create_user(&db_conn, words[0], words[1]);
                        println!("成功新增使用者：{}", words[0]);
                    }
                }
            } else if opt == 2 {
                let results = users
                    //.filter(id.eq("石墨"))
                    .load::<User>(&db_conn)
                    .expect("取使用者失敗");
                for user in results {
                    print!("{} ", user.id);
                }
                println!("\n");
            } else if opt == 3 {
                loop {
                    println!("> {}", p4);
                    println!("> 請輸入欲邀請人的信箱，或輸入空白行回到選單");
                    buff.clear();
                    stdin().read_line(&mut buff)?;
                    let words: Vec<&str> = buff.split_whitespace().collect();
                    if words.len() == 0 {
                        break;
                    } if words.len() != 1 {
                        println!("輸入格式錯誤");
                    } else {
                        send_invite_email(None, words[0]);
                    }
                }
            } else if opt == 4 {
                panic!("尚未實作");
            } else {
                println!("請輸入範圍內的正整數");
            }
        } else {
            println!("請輸入範圍內的正整數");
        }
    }
    Ok(())
}