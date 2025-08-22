use std::{fs,io::{BufRead, BufReader, Write},net::{TcpListener, TcpStream},};
fn main() {
    let listner = match TcpListener::bind("127.0.0.1:3000") {
        Ok(value) => value,
        Err(err) => {
            eprintln!("something went wrong {}", err);
            return;
        }};
    for stream in listner.incoming() {
        let streams = match stream {
            Ok(value) => value,
            Err(err) => {
                eprintln!("connection failed {}", err);
                continue;
            }
        };
        handle(streams);
    }}
fn handle(mut s: TcpStream) {
    let bufreader = BufReader::new(&s);

    let _v: Vec<String> = bufreader
        .lines()
        .map(|r| match r {
            Ok(m) => m,
            Err(_) => String::new(),
        })
        .take_while(|x| !x.is_empty())
        .collect();

    let content = match fs::read_to_string("hello.html"){
        Ok(value)=>value,
        Err(_)=>{
            eprintln!("error while reading file");
            return;
        }
    };
    let length = content.len();
    let response = format!("HTTP/1.1 200 OK\r\nContent-length: {length}\r\n\r\n{content}");

    let _res = match s.write_all(response.as_bytes()) {
        Ok(con) => con,
        Err(e) => {
            eprintln!("error happened {}", e)
        }
    };
}
