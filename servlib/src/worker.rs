

pub mod worker{

    use std::{thread};
    use std::thread::JoinHandle;
    use std::sync::Arc;
    use std::net::{TcpStream};

   
    //use std::sync::Mutex;
    //use std::thread::JoinHandle;
    
   
    


    pub struct Worker<T>{
        pub worker_type: Arc<T>,
        handle_list:  Vec<JoinHandle<()>>,
    }
    
    impl<T: 'static+Runnable+std::marker::Send+std::marker::Sync> Worker<T>{
        pub fn new()->Worker<T>{
            Worker { 
                worker_type: Arc::new(T::new()),
                handle_list: Vec::new(),
            }
        }

        pub fn start(& mut self, sock: Option<TcpStream>)
        {
            let  worker_type = self.worker_type.clone();
            let  handle =  thread::spawn(move || {
                worker_type.run(sock);
		    });
            self.handle_list.push(handle);
        }

      

        pub fn stop(&mut self){
            self.worker_type.stop();
        }
       
    }

    pub trait Runnable {
        fn new()->Self;
        fn run(&self,sock: Option<TcpStream>);
        fn stop(&self);
        
    }
    
}
