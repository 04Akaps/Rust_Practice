fn main() {
    let s = String::from("hoij nbb");
    let length  = first_word(&s);
    println!("{}",length);

    let test  = revise_word(&s);
    println!("{}", test);

    let test_world = String::from("hello world");

    let hello = &test_world[0..3];
    let hello2 = &test_world[..3];

    let hello3 = &test_world[3..test_world.len()];

    let hello4 = &test_world[..];
    let hello5 = &test_world[0..test_world.len()];

    println!("{}", hello2)

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


/*
    일단 해당 값으 bytes배열러 존환을 합니다.

    이후 iter()을 통해서 반환을 하는데 이뗴 enumerate를 통해서 튜플로 반환을 합니다.
    - 이 함수들은 나중에 다루겠습니다.

    반환된 튜플의 첫번쨰은 이넥스, 두번쨰는 참조값입니다.

    이후 바이트 리터럴 문ㄴ법을 활용하여 공백문자를 찾으면 해당 인덱스를 반환합니다.

    이떄 리턴되는 length값은 String이랑은 완전히 분리된 값입니다.
    - 즉 string값이 사라져도 사라지지 않습니다.

    하지만 이러한 방법은 조금 불편하빈다.

    왜냐하면 length변수와 S의 값이 동기화가 되지 않기 떄문입니다.
    - 쉽게 말하면 얕은복사 관계까 되기 떄문입니다.

    이러한 이유로 슬라이스를 사용합니다.

    간단한 슬라이스 사용법은 이와 같고 그후 함수를 수정하면 아래와 같습니다.
*/

fn revise_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
