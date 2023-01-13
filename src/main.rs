// Uncomment this block to pass the first stage
use std::{net::TcpListener, io::Read,io::Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.


    // Uncomment this block to pass the first stage   
     let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
     for stream in listener.incoming() {
         match stream {
             Ok(mut stream) => {

                    let  mut buf = [0;128];
                    let i = stream.read(&mut buf).expect("error encodoing to string");
                    let stream_string = String::from_utf8(buf[..i].to_vec()).expect("asas");
                    let vec_of_command = convert_to_vec_of_msg(stream_string);
            println!("{:?}",vec_of_command);
                if vec_of_command[0].1 == "PING" || vec_of_command[0].1 == "ping"{
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


fn convert_to_vec_of_msg(s:String)->Vec<(i32, String)>{
   let mut vec_of_commands:Vec<(i32,String)> = Vec::new();
   for i in s.lines(){
        let mut t:(i32,String)  = (0,"".to_string());
        if i.contains("*"){

        }
        if i.contains("$"){
        t.0 = i[1..].parse::<i32>().expect("error while parsing to i32");
         
        }else{
            t.1 =i.to_string();

        }

            vec_of_commands.push(t);

   }
   vec_of_commands
}
