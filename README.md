# chc-rust

```
use std::net::{TcpListener, TcpStream};
use std::io::Read;
use std::thread;
use ::chc;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 8192 * 2];
    stream.read(&mut buffer).unwrap();

    let mut chc = chc::chc::CHC::new(stream.try_clone().unwrap(), buffer.to_vec());

    let mut response = chc::response::Response::new(
        chc::response::ResponseType::TextHTML,
        chc::response::StatusCode::Ok,
    );

    let doc_str = r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Rust</title>
            </head>
            <body>
                <h1>Rust</h1>
                <p>Hello, world!</p>
            </body>
        </html>
    "#;

    response.set_body(doc_str.to_string());

    let parsed = response.parse();

    chc.respond(parsed);
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {
            handle_connection(stream);
        });
    }
}
```
