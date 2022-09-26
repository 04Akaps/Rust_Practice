fn main() {
    let mut v : Vec<i32> = Vec::new();
    // 비어있는 백터를 만드는 입니다.

    let v2 = vec![1,2,3];
    // 이외적으로 이렇게도 활용 가능합니다.

    v.push(1);
    // 이런식으로 값을 집어 넣을 수 있게 반드시 mut타입이어야 합니다.

    let read_three = &v[3];
    let read_three2 : &i32 = &v[3];
    let read_three3 : Option<&i32> = v.get(3);

    println!("{}", read_three);
    // 값을 읽을떄에는 이렇게 확용합니다.

    //값을 읽는데에 두가지 방법이 잇는 이유는 값이 존재하지 않을떄를 고려하기 위해서 입니다.

    // 일반적인 [3]같은 경우에는 만약 해당 인덱스가 존재하지 않으면 panic이 발생하지만, get같은 경우에는 None이 반환됩니다.


}
