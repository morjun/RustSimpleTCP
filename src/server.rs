use clap::Parser;
use std::{
    io::{self, Write}, net::{TcpListener, TcpStream, SocketAddr}, sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    }, thread
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    data_size: usize,
    #[arg(short, long, default_value = "0.0.0.0:7272")]
    address: String,
}

fn handle_client(mut stream: TcpStream, length: usize) -> io::Result<()> {
    println!("클라이언트 연결: {}", stream.peer_addr()?);

    let data = vec![0u8; length];

    println!("클라이언트에게 {}바이트 데이터를 보냅니다.", data.len());

    stream.write_all(&data)?; // length 길의의 0으로 채워진 buffer를 그대로 씀
    stream.flush()?;
    println!(
        "데이터 전송 완료. 클라이언트 연결 종료: {}",
        stream.peer_addr()?
    );
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let data_size = args.data_size;
    let address = &args.address;
    let address = address.parse::<SocketAddr>().unwrap();

    let listener = TcpListener::bind(address)?;
    println!("서버 시작: {}, 보낼 데이터 크기: {} 바이트. 종료하려면 Enter 키를 누르세요.", address, data_size);

    let listener_arc = Arc::new(listener);
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    let listener_clone = listener_arc.clone();
    let address_clone = address.clone();

    // 입력 감지 스레드
    thread::spawn(move || {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        println!("Enter 키가 눌렸습니다. 서버를 종료합니다...");
        running_clone.store(false, Ordering::SeqCst);
        // self-connect 시도하여 accept()를 깨움
        TcpStream::connect(address_clone).ok();
        // listener를 drop하여 새로운 연결 수락을 중단
        drop(listener_clone);
    });

    // 연결 수락 스레드
    while running.load(Ordering::SeqCst) {
        match listener_arc.accept() {
            Ok((stream, addr)) => {
                println!("새로운 클라이언트 연결: {}", addr);
                let cloned_data_size = data_size;
                thread::spawn(move || {
                    handle_client(stream, cloned_data_size).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
            Err(e) => {
                if running.load(Ordering::SeqCst) {
                    eprintln!("연결 수락 오류: {}", e);
                } else {
                    println!("새로운 연결 수락 중단.");
                    break;
                }
            }
        }
    }

    println!("서버 종료 완료.");
    Ok(())
}