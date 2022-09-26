fn main() {
    let bol : bool = true;
    let bol2 = true;

    let num : u64 = "42".trim().parse().expect("Not a number!");

    let tup : (i32, f64, u8) = (500, 6.4 , 1);

    let (x,y, z) : (i32, f32, u64) = (1, 0.1, 5);

    let abc = tup.0;
    let bbb = tup.1;

    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];
}

/*

Rust는 한번 선언된 타입의 변수는 고정되어 있습니다.

타입을 적지 않으면 자동으로 default타입으로 결정되며 적을경우 적는 타입으로 적용이 됩니다.

또한 여러가지 변수 선언도 가능합니다.

배열도 이와 같이 선언이 가능한데 특이한 점이 있습니다.

해당 코드는 compile = cargo check 할 시에는 문제가 발생을 하지는 않지만 cargo run을 하면 문제가 발생을 합니다.

왜냐하면 해당 배열의 값이 없기 떄문에

이러한 이유가 Rust의 안전 원칙이 동작하는 첫번쨰 이유 입니다.

Rust입장에서는 해당 배열의 메모리에 접근이 불가능하기 떄문에 신속하게 바로 종료하여 오류를 해결하는 것 입니다.

*/