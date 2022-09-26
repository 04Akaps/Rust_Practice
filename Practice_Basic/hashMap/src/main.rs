use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"),10);
    scores.insert(String::from("red"), 20);

    // 가장 덜 사용되는 컬렉션 입니다.
    // key, value를 통해서 저장을 하게 됩니다.

    // 이외에도 좀 복잡하게 사용하면 다음과 같이 작성 가능합니다.

    let keys : Vec<String> = vec![String::from("hoijn"), String::from("hojinse2")];

    let values  : Vec<i32> = vec![1,2];

    let test : HashMap<_, _> = keys.iter().zip(values.iter()).collect();
    // 하지만 마찬가지로 값을 Hash안으로 복사하는 것이기 떄문에 이전 값을 사용이 불가능합니다.
    // -> 메모리가 이동하기 떄문입니다.

    // 값을 가져올 떄엔느 다음과 같습니다.

    let test_get = scores.get(&String::from("blue"));

    // return type이 Option이기 떄문에 없을시에는 None가 return 됩니다.

    // 일반적으로 같은 키값이 있다면 해당 값을 덮어 씁니다.
    //  scores.insert(String::from("blue"),20);
    // -> scores의 blue값이 20으로 갱신 됩니다.

    //하지만 만약 없는 키값을 확인한뒤에 넣고 싶다면 이렇게 확용합니다.

    scores.entry(String::from("yellow")).or_insert(50);
    scores.entry(String::from("blue")).or_insert(100);
    // 그러면 blue라는 키 값이 없다면 100이 들어가게 될 것입니다.

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        println!("{}",count);
    }

    println!("{:?}", map);

}
