// Uncomment this block to pass the cstage
use std::{ net::{ TcpListener, TcpStream }, io::Read, io::Write, collections::HashMap };

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
                th.execute(move || {
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
    let mut vec_of_commands: Vec<executor::Command> = Vec::new();
    let i = stream.read(&mut buf).expect("error encodoing to string");
    let stream_string = String::from_utf8(buf[..i].to_vec()).expect("asas");
    convert_to_vec_of_msg(stream_string, &mut vec_of_commands);

    println!("{:?}", vec_of_commands);
    for tup in vec_of_commands.iter() {
        if tup.command[0] == Some("PING".to_string()) || tup.command[0] == Some("ping".to_string()) {
            println!("{}", simple_string_encoder(&"PONG".to_string()));

            write!(stream, "{}", simple_string_encoder(&"PONG".to_string())).expect(
                "error writeing to stream"
            );
        } else if tup.ty == Some("print".to_string()) {
            for i in &tup.command{



                match i {
                    Some(x) => {
                        write!(stream, "{}", simple_string_encoder(&x)).expect("erooorrrr");
                    }
                    None => {
                        println!("some thing weird");
                    }
                }
            }
        }    else if tup.ty == Some("SET".to_string()){
                   

                        write!(stream, "{}", get_set_cahcer(tup.ty.to_owned().unwrap(), &tup.command) ).expect("erooorrrr");

            }
        else if tup.ty == Some("GET".to_string()){
                   

                        write!(stream, "{}", get_set_cahcer(tup.ty.to_owned().unwrap(), &tup.command) ).expect("erooorrrr");

            }

    }
}

fn get_set_cahcer(method:String,commands:&Vec<Option<String>>)->String{


let mut store:HashMap<String,String> = std::collections::HashMap::new();
if method =="SET"    {
    let key  = &commands[0].to_owned().unwrap();
    let value = &commands[1].to_owned().unwrap();


        let res = store.insert(key.to_string(), value.to_string());

        if res ==None{
    
               return      simple_string_encoder(&"Ok".to_string());

        }else{
        return simple_string_encoder(&res.unwrap().to_string());


        }


}else{
    let key  = &commands[0].to_owned().unwrap();
        let res = store.get(key);
 if res ==None{
    
               return      simple_string_encoder(&"nil".to_string());

        }else{
        return simple_string_encoder(&res.unwrap().to_string());


        }


}


}
fn convert_to_vec_of_msg(s: String, vec_of_commands: &mut Vec<executor::Command>) {
    let mut count = 0;

    let mut t: executor::Command = executor::Command { ty: None, command: Vec::new() };
    t.command.push(None);
    vec_of_commands.push(t);
    {}
    for i in s.lines() {
        let mut t: executor::Command = executor::Command { ty: None, command: Vec::new() };

    t.command.push(None);
        if i.contains("*") {
        } else if i.contains("$") {
        } else {
            if vec_of_commands[count].ty != None && vec_of_commands[count].command[0]!= None {
                vec_of_commands.push(t);
                count += 1;
            }

            if vec_of_commands[count].ty == None {
                if i.contains("PING") || i.contains("ping") {
                    vec_of_commands[count].ty = Some("print".to_string());

                    vec_of_commands[count].command.push( Some("PING".to_string())) ;

                    continue;
               }
                if i.contains("ECHO") || i.contains("echo") {
                    vec_of_commands[count].ty = Some("print".to_string());
                    continue;
                }if i.contains("GET") || i.contains("get"){
                    vec_of_commands[count].ty = Some("GET".to_string());


                }}if vec_of_commands[count].ty != None {

                    
                    vec_of_commands[count].command.push( Some(i.to_string())) ;
                
                    
                
            }
        }
    }
}
