fn main() {
    println!("Hello, world!");

    println!("Hello, world!");
}

/*


mainr함수는 중요합니다.
go와 같이  모든 실행 가능한 러스트 프로그램에서 무조건적으로 첫번째로 실행이 되는 코드 입니다.

println!또한 특별합니다.

이는 일반적으로 매크로라고 부르며 !이 없다면 보통의 함수를 의미합니다.
- 매크로는 나중에 알아보겠습니다.

컴파일 하는 명령어 : rustc main.rs

이후 만들어진 파일을 ./main 이라고 입력하여 실행합니다.


cargo를 활용할 떄에는 다음과 같습니다.

cargo build, 

cargo run

하지만 이외에도 cargo check라는 형태가 있습닌다.

cargo check는 오로지 컴파일 하는데에 의미를 두기 떄문에 cargo build보다 훨씬 빠릅니다.

그러기 떄문에 개발 단계라면 cargo check를 주로 활용하면 됩니다.

*/
