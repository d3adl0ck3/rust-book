use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener,TcpStream},
    thread,
    time::Duration,
};
use hello::ThreadPool;
fn main() {
    
    let method = std::env::var("HELLO_METHOD").unwrap_or("BLAZING".to_string());
    let num_threads: usize = std::env::var("HELLO_THREADS").unwrap_or("4".to_string()).parse().expect("Positive integer required for HELLO_THREADS");
    let max_requests: usize = std::env::var("HELLO_MAX_REQUESTS").unwrap_or(u32::MAX.to_string()).parse().expect("Positive integer required for HELLO_LIFETIME");
    println!("Using handle_connection type {method} and num_threads {num_threads} with max_requests {max_requests}");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(max_requests) {
        let stream = stream.unwrap();
        match method.as_str() {
            "DUMB" => handle_connection(stream),
            "SLOW" => handle_connection_slow(stream),
            "SPAWN" => handle_connection_spawn_thread(stream),
            "BLAZING" => handle_connection_thread_pool(stream,&pool),
            _ => handle_connection(stream),
        }
        //handle_connection_slow(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let(status_line, filename) = if request_line == "GET / HTTP/1.1" {
       ("HTTP/1.1 200 OK", "hello.html") 
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
fn handle_connection_slow(mut stream: TcpStream) {
    actually_handle(stream);
}
fn handle_connection_spawn_thread(mut stream: TcpStream) {
    thread::spawn(move || {
        actually_handle(stream);
    });
}
fn handle_connection_thread_pool(mut stream: TcpStream, pool: &ThreadPool) {
   pool.execute(move || {
        actually_handle(stream);
   }

    
    ); 
}
fn actually_handle(mut stream: TcpStream) {

    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
