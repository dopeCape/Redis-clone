#[derive(Debug)]
pub struct Command{

    pub ty:Option<String>,
    pub command:String,

}


impl Command{
       pub fn new(ty:Option<String>,command:String)->Command{
           Command {  ty,command }
        

       } 


      

}
