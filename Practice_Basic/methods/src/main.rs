#[derive(Debug)]

struct Rectangle {
    width : u64,
    length : u64
}

fn main() {
    let mut rect : Rectangle  = Rectangle {
        width :3 ,
        length : 5
    };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("{:#?}", rect.area());

    println!("{:#?}", rect.change_area());

    println!("{:#?}", rect.length);
    
    println!("Can rect1 hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect.can_hold(&rect3));
}


impl Rectangle {
    fn area(&self) -> u64 {
        self.length * self.width
    }

    fn change_area(&mut self) -> u64 {
        self.length = 3;

        self.length * self.width
    }

    fn can_hold(&self , test : &Rectangle) -> bool{
        self.length > test.length && self.width > test.width
    }
    // 해당 값의 인자는 반드시 self로 적혀야 합니다.

    // 두번쨰 다른 인자로 들어가는 값은 변수의 명에 제약이 없습니다.
}