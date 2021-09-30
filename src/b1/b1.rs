use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
    // i32 u64 i64 默认推断成i32
    let secret_number = rand::thread_rng().gen_range(1, 101); 
    println!("神秘数字:{}", secret_number);
    println!("猜测一个数字");
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
    
        // 显式声明类型
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("猜测的数是: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("to big"),
            Ordering::Equal => {
                println!("you win");
                break;
            },
        }
    }
}
