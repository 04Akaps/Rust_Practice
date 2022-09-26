#[derive(Debug)]


struct Rectangle {
    length : u32,
    width : u32
}

fn main() {
    let rectangle : Rectangle  = Rectangle{
        length : 3,
        width :5
    };

    println!("{:?}", area(&rectangle));

    println!("{:#?}", rectangle);

}

fn area(rec : &Rectangle) -> u32 {
    rec.length * rec.width
}


/*

일반적으로 구조체를 println!할 수가 없습니다.

하지만 구조체 데이터를 보고자 할댸에는 다음과 같이 

#[derive(Debug)]를 추가해 준뒤에

{:?}를 활용하면 됩니다.
- 이것 말고도 {:#?} 으로도 작성 가능하며 좀더 짜임새 있는 출력을 보여줍니다.



*/