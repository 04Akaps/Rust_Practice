fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // 해당 함수 내부로 소유권이 넘어오지 않기 떄문에 drop이 발생하지 않습니다.
}

fn change_value(s: &String) {
    s.push_str(", world");
    // 해당 함수는 에러가 발생을 합니다.
    // 왜냐하면 넘어오는 인자가 단순히 참조값이기 떄문입니다.
    // 이를 해결 하기 위해서는 &String 을 &mut String으로 변경합니다.
}

/*

앞서 ownership부분을 통해서 rust의 메모리에 대해서 알아보았습니다.

이 코드는 앞선 예시를 어떻게 해결하는지를 적은 코드 입니다.

&키워드는 참조자로 어떤 소유권을 넘기지 않고 참조값만 넘기는 것을 믜미합니다.


*/