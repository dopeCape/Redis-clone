// Uncomment this block to pass the cstage
use std::{ net::{ TcpListener, TcpStream }, io::Read, io::Write };

use threds::ThreadPool;
mod threds;
mod executor;
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let th = ThreadPool::new(5);
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {

                th.execute(move||{

                    loop {
                        responder(&mut stream);
                    }

                });
            }

            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn simple_string_encoder(data: &String) -> String {
    format!("+{}\r\n", data)
}

fn responder(stream: &mut TcpStream) {
    let mut buf = [0; 128];
    let mut vec_of_commands:Vec<executor::Command> = Vec::new();
    let i = stream.read(&mut buf).expect("error encodoing to string");
    let stream_string = String::from_utf8(buf[..i].to_vec()).expect("asas");
    convert_to_vec_of_msg(stream_string,&mut vec_of_commands);

    println!("{:?}",vec_of_commands);
    for tup in vec_of_commands.iter() {
        if tup.command == Some("PING".to_string()) || tup.command == Some("ping".to_string()) {
            println!("{}", simple_string_encoder(&"PONG".to_string()));

            write!(stream, "{}", simple_string_encoder(&"PONG".to_string())).expect(
                "error writeing to stream"
            );

        }
        else if tup.ty == Some("print".to_string()){
        

            match &tup.command {
                Some(x)=>{

            write!(stream, "{}", simple_string_encoder(&x)).expect("erooorrrr");

                },
                _=>{}
            }
        }
    }
}
fn convert_to_vec_of_msg(s: String,vec_of_commands: &mut Vec<executor::Command>)  {

    let mut count = 0;

        let t  :executor::Command  = executor::Command { ty: None, command: Some("".to_string()) };
           vec_of_commands.push(t);
    for i in s.lines() {
        let t  :executor::Command  = executor::Command { ty: None, command: Some("".to_string()) };

        if i.contains("*") {
        } else if i.contains("$") {
        
        } else {
            if  vec_of_commands[count].ty != None && vec_of_commands[count].command ==None {


           vec_of_commands.push(t);
            }

            if vec_of_commands[count].ty == None{
                    if i.contains("PING")|| i.contains("ping"){
                 
            vec_of_commands[count].ty = Some("print".to_string());

            vec_of_commands[count].command =Some("PING".to_string());

        let t  :executor::Command  = executor::Command { ty: None, command: Some("".to_string()) };
           vec_of_commands.push(t);
            count += 1;
            }
                    if i.contains("ECHO") || i.contains("echo"){

println!("{}",count);
            vec_of_commands[count].ty = Some("ECHO".to_string());
            }
            else if vec_of_commands[count].ty!=None{
                
     vec_of_commands[count].command = Some(i.to_string());

            count += 1;
            }
            }


        }
    }

}
