pub mod listener{
    use crate::worker::worker::Worker;
    use crate::worker::worker::Runnable;
    use crate::client::client::Client;
    //use std::sync::Arc;
    use std::time;
    use std::thread;
    use std::sync::Mutex;
    use std::net::{TcpListener, TcpStream};
    use std::sync::atomic::{AtomicBool,Ordering};


    pub struct ListenerData {
        pub id: u32,   
    }

    impl ListenerData {
        fn new()->Self{
            ListenerData{
                id: 0,
            }
        }
    }

    pub struct Listener{
        pub data: Mutex<ListenerData>,
        listener: std::net::TcpListener,
        pub is_running: AtomicBool,
    }

    impl Listener{
        pub fn handle_connection(stream: Option<TcpStream>) {
            let mut w: Worker<Client>;
            w=Worker::new();
            match stream {
               Some(_)=>{
                println!("Starting client");
                w.start(stream);
               }
               None => {
                println!("No scoket");
               } 
            }     
        }
    }

    impl Runnable for Listener{
        fn new()->Self{
            Listener{
                data: Mutex::new(ListenerData::new()),
                listener: TcpListener::bind("127.0.0.1:1234").unwrap(),
                is_running: AtomicBool::new(false),
            }
        }

        fn run(&self,_sock: Option<TcpStream>){
           
            self.is_running.store(true, Ordering::Relaxed);
            println!("Started listener");
            
            
            {
                let mut data_guard = self.data.lock().unwrap();
                (*data_guard).id=0;
            }
            
            
            while self.is_running.load(Ordering::Relaxed) {
                match self.listener.accept() {
                    Ok((_socket, addr)) => {
                        println!("new client: {:?}", addr);
                        Listener::handle_connection(Some(_socket));
                    },
                    Err(e) => println!("couldn't get client: {:?}", e),
                }
                
               
                let mut data_guard = self.data.lock().unwrap();
                (*data_guard).id=(*data_guard).id+1;
                println!("*************{}",(*data_guard).id);
                drop(data_guard);
                let delay = time::Duration::from_millis(2000); 
                thread::sleep(delay);
                
                

            }
            
        }
        
        fn stop(&self){
            self.is_running.store(false, Ordering::Relaxed); 
        }

       

    }
}