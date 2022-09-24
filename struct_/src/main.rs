struct User {
    username : String,
    email : String,
    int : u64
}


fn main() {
  
    let mut user1 = User {
        email : String::from("sdl155@naver.com"),
        username : String::from("hojin"),
        int : 3
    };

    let user2 = User {
        email : String::from("sdl155@naver.com"),
        username : String::from("hojin"),
        int : 3
    };
    
    println!("{}", user1.username);
    println!("{}", user1.email);

    user1.email = String::from("test");

    // user1.username = "test";
    // Error : 리터럴 타입이기 떄문에 &str로 타입이 잡히게 딘다.


    // user2.email = String::from("test2");
    // Error

    println!("{}", user1.email);

    let new_user : User = new_user(String::from("nave.rc"), String::from("hoijn"));

    let borrow_user = User{
        email : String::from("hoijn"),
        ..user1
    };
    // 이와 같이 시네틱을 활용하여 이전의 값을 재활용 가능합니다.

}


fn new_user(email : String, username : String) -> User{
    User {
        email : email,
        username : username,
        int : 3
    }
    // 이렇게도 작성이 가능하지만 만약 key값과 value값이 일치할 경우

    // User {
    //     email,
    //     username
    // }
    // 이렇게도 작성 가능합니다.
}