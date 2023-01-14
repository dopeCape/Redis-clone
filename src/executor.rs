#[derive(Debug)]
pub struct Command{

    pub ty:Option<String>,
    pub command:Option<String>,

}


impl Command{
       pub fn new(ty:Option<String>,command:Option<String>)->Command{
           Command {  ty,command }
        

       } 


      

}
