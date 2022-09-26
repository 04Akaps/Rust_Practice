use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:8080";
const MSG_SIZE: usize = 32;

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

fn main() {
    let server = match TcpListener::bind(LOCAL) {
        Ok(server) => {
            println!("Server Start on {}", LOCAL);
            server
        }
        Err(err) => {
            println!("Error Occur , {}", err);
            panic!("Error");
        }
    };

    server
        .set_nonblocking(true)
        .expect("failed to initialize non-blocking");

    let mut clients = vec![];

    let (sender, receiver) = mpsc::channel::<String>();
    // 새로운 채널을 만드는 함수
    println!("channel sender : {:?}", sender); // {..}
    println!("channel receiver : {:?}", receiver); // {..}

    // 리턴값으로 오는 sender에 들어오는 모든 데이터는 receiver에서 사용이 가능하다.

    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            // 해당 주소로 새로운 연결을 허용하는 것
            // 리턴 값으로 TCpStream과 peer주소가 return 된다.
            println!("client {} connected", addr);
            println!("socker is : {:?}", socket);

            let sender = sender.clone();

            clients.push(socket.try_clone().expect("failed to socket clone"));

            println!("한번만 실행?? - 1");
            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];
                // 0을 MSG_SiZE만큼 만들어 주라는 코드

                // loop로 인해서 socket에 연결되어 있으면 thread::spawn에서 계속해서 실행이 된다.

                // 이후 값이 들어오면 socket에 변화가 생길 것이고 해당 값을 읽어오는 것을 시도한다.

                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        // 계속 listening하고 있다가 데이터가 들어오면 이졲 Ok를 타게 된다.

                        // 왜냐하면 read_exact에서 읽을 값이 있기 떄문에

                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        // 이후 buff에 있는 데이터를 msg에 옮겨 준다.
                        // 이후 take_while 조건문을 통해서 0을 제거해 준다.
                        // Coolect는 해당 값을 다시 Collection으로 바꾸어 주는 메서드 입니다.

                        let msg = String::from_utf8(msg).expect("Invalid utf8 message");

                        // 일반적으로 [i32, i32, i32]이러한 형태가 되기 떄문에 이를 다시 String으로 바꾸어 주고 출력합니다.

                        println!("{}: {:?}", addr, msg);

                        sender.send(msg).expect("failed to send msg to rx");
                    }
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => {
                        (println!("접속 하였지만 데이터 전송 x"))
                    }
                    Err(_) => {
                        println!("closing connection with: {}", addr);
                        break;
                    }
                }
                sleep();
            });
        }

        if let Ok(msg) = receiver.try_recv() {
            clients = clients
                .into_iter()
                .filter_map(|mut client| {
                    let mut buff = msg.clone().into_bytes();
                    buff.resize(MSG_SIZE, 0);

                    client.write_all(&buff).map(|_| client).ok()
                })
                .collect::<Vec<_>>();
        }

        sleep();
    }
}
