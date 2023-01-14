use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc;

    pub trait FnBox{
    fn call_box(self:Box<Self>);
    }
impl <F:FnOnce()>FnBox for F{
    fn call_box(self:Box<Self>) {
        (*self)()
    }

}
    type Job = Box<dyn FnBox + Send + 'static>;
    pub struct ThreadPool {
        workers :Vec<Worker>,
            sender:mpsc::Sender<Job>,

    }  
    impl ThreadPool{

    pub fn new(size:usize)->ThreadPool{
       assert!(size>0); 

       let mut vec_of_treads = Vec::with_capacity(size);
       let (sender,reciver) = mpsc::channel();

       let     reciver = Arc::new(Mutex::new(reciver));
        for id in 0..=size{
        vec_of_treads.push(Worker::new(id,Arc::clone(&reciver)));
        

        };
        ThreadPool{

            workers:vec_of_treads,
            sender
        }
    }
    pub fn execute<F>(&self,f:F)
        where
            F:FnOnce() + Send + 'static
            {

        let job =Box::new(f);
        self.sender.send(job).unwrap();
            }
    }
    pub struct Worker{
        id:usize,
        worker:thread::JoinHandle<()>
    
    }
    impl Worker{

    pub fn new(id:usize,reciver:Arc<Mutex<mpsc::Receiver<Job>>>)->Worker{
        let w =thread::spawn(move||{

loop{

    let job = reciver.lock().unwrap().recv().unwrap(); 
    job.call_box();

}



        });





        Worker{
        id ,
        worker :        w}

    }
    }




