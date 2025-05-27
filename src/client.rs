use clap::Parser;
use std::io::{self, Read};
use std::net::TcpStream;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 서버가 바인딩될 주소와 포트
    #[arg(short, long, default_value = "127.0.0.1:7272")]
    address: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let address = &args.address;

    let mut stream = TcpStream::connect(address)?;
    println!("서버에 연결됨: {address}");

    // let input = String::from("Hello");
    // let trimmed_input = input.trim();

    // stream.write_all(trimmed_input.as_bytes())?;
    // stream.write_all(b"\n")?; // 개행 문자 추가 (서버에서 ReadLine처럼 처리할 경우)
    // stream.flush()?;

    let mut buffer = Vec::new();
    let bytes_read = stream.read_to_end(&mut buffer)?;
    println!("서버로부터 {} 바이트 데이터를 받았습니다.", bytes_read);

    Ok(())
}
