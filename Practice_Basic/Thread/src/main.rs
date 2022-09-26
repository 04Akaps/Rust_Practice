use std::thread;

fn main() {
    let join_handle = thread::spawn(|| {
        println!("i am printin something");
        // 프린트를 하지 않는다. 왜냐하면 쓰레드가 여러개이기 떄문에
        // 즉 main을 담당하는 쓰레드 차례가 오기 전에 종료가 되기 떄문에
        // 그러기 떄문에 강제로 for문을 추가해 실행하는데에 좀더 시간이 걸리게 조정
    });
    // thread는 return 타입으로 joinHandlef를 가지고 있다.
 
    join_handle.join(); // wait
    // 이렇게 실행을 하면 해당 main이 끝나기 전에 join_handle가 끝나기를 기다리게 됩니다.

    



    // for _ in 0..100000 {
    //     let _ = 8;
    // }


}
