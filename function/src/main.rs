fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let a = [10, 20, 30, 40, 50];

    for element in a.iter()  {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn test1() -> u64{
    5
    // or return 5;
}

/*

세미콜론의 효능에 대해서 알 수가 잇습니다.

일반적으로 ;가 들어가면 구문으로 구분한다는 의미입니다.

하지만 ; 가 들어가지 않는다면 해당 구문은 return으로 구분 됩니다.

또한 함수의 반환타입을 지정할 떄에는 -> 으로 표기합니다.

*/
