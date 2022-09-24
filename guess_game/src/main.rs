extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("insert number");
                continue;
            }

        };

        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => {
                println!("Too big!");
                break;
            },
            Ordering::Equal   => println!("You win!"),
        }
    }
   
}

/*

rust에서 if역할을 하는 것이 match 이며 이후의 값은 Ordering으로 처리가 가능하다.

trim같은 경우에는 해당 값의 앞과 뒤의 빈 공백을 제거해 주는 특징이 있다.

왜냐하면 우리가 io를 통해서 5라는 숫자를 입력하고 엔터를 누르면 반환되는 값은 5\n이다.

그러기 떄문에 \n값을 지워주기 위해서 trim을 활용한다

parse는 문자열을 숫자형으로 파싱하는 역할을 한다.

parse는 에러가 발생하기 쉽습니다.

만약 A&&같은 문자열이 있다면 해당 값을 정수로 바꿀 수 없습니다.

match타입을 활용하여 에러 처리 도한 가능합니다.

match는 Ok 또는 Err을 가지고 있는 열거형이기 떄문입니다.



*/