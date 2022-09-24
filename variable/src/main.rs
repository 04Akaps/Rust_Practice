fn main() {
    let mut x = 5;
    println!("value : {}", x);

    x = 6;
    println!("value : {}", x);

    let a = 5;
    println!("value : {}", a);

    let a = 1;
    println!("value : {}", a);

    let x= 3;
    println!("value : {}", x);

    let x = x + 5;
    println!("value : {}", x);

    let space = "abcdd";
    println!("value : {}", space);
    let space = space.len();
    println!("value : {}", space);

    let mut space2 = "abcdd";
    println!("value : {}", space2);


    // space2 = space2.len()
    // println!("value : {}", space2);
    // Error 발생
}

/*

mut 키워드를 쓰지 않은 변수는 불변성으로 다시 값을 바꿀수 없습니다.
- 즉 const와 같은 역할을 합니다.

물론 rust에도 const가 있고 이 둘은 미묘한 차이점이 있습니다.

1. 상수는 mut를 사용하는 것이 허락되지 않습니다.

2. 상수는 전체 영역을 포함하여 어떤 영역에서도 선언 가능합니다.

3. 상수는 오직 표현식만 설정될 뿐이지 결과값이나 실행시간에 결정되는 값이 설정 불가능합니다.

shadow라고 부르는 재선언은 굉장히 특이하게 작동을 합니다.

기본적으로 선언된 a라는 변수는 무시하고 새로운 a를 인식하기 떄문입니다.

mut를 사용하면 되지 왜 shadow를 사용하냐에 대한 질문은 spaces변수를 통해서 볼 수 있습니다.

shadow를 재 선언을 하는 것과 같기 떄문에 문자열에서 정수값으로 변환이 가능합니다.

하지만 mut같은 경우에는 이미 결정된 타입의 값을 바꾸는 행위이기 떄문에 타입 변환이 불가능 합니다.


*/
