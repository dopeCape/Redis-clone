#[derive(Debug)]
pub struct Command{

    pub ty:Option<String>,
    pub command:Vec<Option<String>>,
    
}


impl Command{
       pub fn new(ty:Option<String>,command:Option<String>)->Command{
           Command {  ty,command:Vec::new() }
        

       } 


      

}
