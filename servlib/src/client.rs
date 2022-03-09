//pub mod worker;
//use crate::worker;
//use std::thread;
//use std::sync::Arc;
//use std::sync::Mutex;


pub mod client{

    //use crate::worker::worker;
    use crate::worker::worker::Runnable;
    use std::net::TcpStream;
    use std::sync::Arc;
    use std::sync::Mutex;
    //use std::time;
    use std::io::Read;
    use std::sync::atomic::{AtomicBool,Ordering};

    struct ClientData{
        id: u32,
        recv_buffer: [u8;1024]
    }
    
    impl  ClientData {
        fn new()->ClientData{
            ClientData{
                id: 0,
                recv_buffer: [0;1024],
            }
        }
    }

    pub struct Client{
        data: Arc<Mutex<ClientData>>,
        is_running: AtomicBool,
    }

    impl Client {
        pub fn on_read(&self, buf: [u8;1024]){
            println!("On Read");
        }
    }

    impl Runnable for Client{
        fn new()->Self{
            Client{
                data: Arc::new(Mutex::new(ClientData::new())),
                is_running: AtomicBool::new(false),
            }
        }
        
        fn run(&self, _sock: Option<TcpStream>)
        {
            
            println!("Client started");
            self.is_running.store(true, Ordering::Relaxed);
            while self.is_running.load(Ordering::Relaxed){
                
                {
                    let mut data = self.data.lock().unwrap();
                    (*data).id=1;
                    println!("Client run {}",(*data).id);
                    (*data).recv_buffer=[0;1024];
                }
                                
                
                let mut buffer: [u8;1024]=[0;1024];
                
                let result = _sock.as_ref().unwrap().read(&mut buffer);
                match result {
                    Ok(sz)=> {
                        println!("Size={}",sz);
                        if sz == 0{
                            break;
                        }
                        {
                            let mut data = self.data.lock().unwrap();
                            (*data).id=1;
                            println!("Client run {}",(*data).id);
                            self.on_read((*data).recv_buffer);
                        }
                        
                    }
                    Err(e) => println!("Read Error{} ",e.to_string()),

                }

            }
            self.is_running.store(false, Ordering::Relaxed);
            println!("Disconnected, Client thread closed");
        }
            
        fn stop(&self){
            self.is_running.store(false, Ordering::Relaxed); 
        }



      
    }

}