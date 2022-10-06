use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("서버 열기 실패쓰~");
    // .unwrap(); // unwrap는 에러 발생시에 프로그램을 멈추게 합니다. => 단순한 panic

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        // println!("Connection established!"); // 이렇게 작성이 되면 localhost:8080에 트래픽이 올떄마다 해당 구문이 print됩니다.
    }
}
fn handle_connection(mut stream: TcpStream) {
    // mut값을 준 이유는 stream내부에서 자체적으로 데이터를 추적하거나 읽기 떄문에 상태가 바뀌게 되기 떄문에 주게 됩니다.
    let mut buffer = [0; 52];

    stream.read(&mut buffer).expect("읽기 실패");

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
