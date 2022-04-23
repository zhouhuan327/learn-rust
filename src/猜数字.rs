use rand::Rng;
use std::cmp::Ordering;
use std::io; // trait 类似于接口
fn main() {
    println!("猜数游戏!!");

    let secret_number = rand::thread_rng().gen_range(1, 11);
    println!("神秘数字是: {}", secret_number);

    loop {
        println!("猜测一个数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");

        // rust 允许同名变量,隐藏前面的变量
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("你猜测的数是:{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"), // arm
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("猜中了");
                break;
            }
        }
    }
}
