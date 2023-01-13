// Uncomment this block to pass the first stage
use std::{net::TcpListener, io::Read,io::Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.


    // Uncomment this block to pass the first stage   
     let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
     for stream in listener.incoming() {
         match stream {
             Ok(mut stream) => {
                 let mut stream_string:String = String::new();
                println!("connectd to someone");

                 stream.read_to_string(&mut stream_string).expect("error encodoing to string");
                    println!("{}",stream_string) ;
                 if stream_string == "PING" || stream_string == "ping"{
                     println!("{}",simple_string_encoder("PONG".to_string()));
                     write!(stream,"{}",simple_string_encoder("PONG".to_string())).expect("error writeing to stream");
                     
                 }
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}


fn simple_string_encoder(data:String)->String{
       format!("+{}\r\n",data) 

}
