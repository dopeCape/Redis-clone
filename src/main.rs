// Uncomment this block to pass the first stage
use std::{net::{TcpListener, TcpStream}, io::Read,io::Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.


    // Uncomment this block to pass the first stage   
     let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
     for stream in listener.incoming() {
         match stream {
             Ok( stream) => {
        responder(stream);
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


fn responder(mut stream:TcpStream){
                    let  mut buf = [0;128];
                    let i = stream.read(&mut buf).expect("error encodoing to string");
                    let stream_string = String::from_utf8(buf[..i].to_vec()).expect("asas");
                    let vec_of_command = convert_to_vec_of_msg(stream_string);
            println!("{:?}",vec_of_command);
            for tup in vec_of_command.iter(){

                if tup.1 == "PING" || tup.1 == "ping"{
                    println!("{}",simple_string_encoder("PONG".to_string()));

                    write!(stream,"{}",simple_string_encoder("PONG".to_string())).expect("error writeing to stream");

       
 
                }
}
}
fn convert_to_vec_of_msg(s:String)->Vec<(i32, String)>{
   let mut vec_of_commands:Vec<(i32,String)> = Vec::new();
   let mut count = 0;
   for i in s.lines(){
        let  t:(i32,String)  = (0,"".to_string());

        if i.contains("*"){

        }
        else if i.contains("$"){
            vec_of_commands.push(t);

        vec_of_commands[count].0 = i[1..].parse::<i32>().expect("error while parsing to i32");
         
        }else{
            vec_of_commands[count].1 =i.to_string();
    count+=1;
        }



   }
   vec_of_commands
}
