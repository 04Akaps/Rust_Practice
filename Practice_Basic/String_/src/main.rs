fn main() {
    let mut s : String = String::new();
    // 일반적인 string의 호출

    let data = "chagne to String Type";
    
    let to_string = data.to_string();
    // 리터럴 타입의 str에서 String이로 만들어 주기

    let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; 

    // println!("{:?}", s1);
    // Err -> s2와 다르게 s1의 소유권을 이동시켰기 떄문에 error 발생

    let temp = "hoijn";

    let temp_test = s2 + &temp;
    // println!("{:?}", s2)
    // Err -> 마찬가지로 s2의 복사본을 이동시켰기 떄문에 에러

    // +즉 더하는 함수는 rust측에서 두번쨰 인자로 &str타입을 받게 됩니다.
    // 위에 &s2는 다른 타입이지만 가능한 이유는 역참조강제가 발생하기 떄문입니다.

    let ss1 = String::from("hello");
    // let h = ss1[0];
    // Err -> string은 인덱스 접근을 허용하지 않습니다.

    // 인덱스 값에 접근을 하고자 할떄에는 이렇게 접근을 합니다.
    let ss2 = "hoijnioij";
    let answer = &ss2[0..4];
    let answer2 = &ss1[0..2];

    // 이외에도 이런식으로 접근을 합니다.

    for i in ss1.chars(){
        println!("{}", i);
    }
}
