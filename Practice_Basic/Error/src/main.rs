use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // panic!("panic test");

    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) =>{
            println!("testst");
            panic!("No File");
        },
    };
    // 이렇게도 활용 가능하며

    let b = File::open("hello.txt");

    let b = match b {
        Ok(file) => file,
        Err(error) =>{
            println!("testst");
            panic!("No File");
        },
    };
    // 이런식으로도 작성 가능합니다,

    // 에러에 따라서 다른 처리도 가능합니다

    let c = File::open("hello.txt");

    match c {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };

    // 이러한 형태로 에러 안에서 에러를 감지하여 다른 처리도 가능합니다.

    // 이외에도 unwrap, expect메서드가 있습니다.
    // 해당 메서드들은 모두 자동으로 panic을 해주는 역할으 합니다.

    let abc = File::open("dfjsldkfcx.txt").unwrap();
    // -> unwrap는 자동으로 panic을 발생시켜 줍니다 -> Error 인 경우

    let cba = File::open("abcsd.txt").expect("error 입니다.")
    // -> expect또한 자동으로 panic을 시켜주지만 메세지를 담을수 있습니다.


}


/*

panic은 실패 메시지를 출력하고 스택을 청소한뒤에 종료시키는 역할을 합니다.

보통 panic을 사용하면 데이터를 삭제하게 되는데 데이터를 삭제하지 않는 방법인 abort도 있습니다.

에러를 다루는 방법의 또 한가지는 Result타입입니다.

enum Result<T, E> {
    Ok(T),
    Err(E),
}

해당 형태로 구성이 되어있고 T는 성공할 경우 반환타입, E는 에러 타입을 의미합니다.
- Result타입을 반환하는 라이브러리가 있고 반환하지 않는 라이브러리가 존재합니다.

*/