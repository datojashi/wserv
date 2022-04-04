//pub mod worker;
//use crate::worker;
//use std::thread;
//use std::sync::Arc;
//use std::sync::Mutex;





pub mod client{

    //use crate::worker::worker;
    use crate::worker::worker::Runnable;
    use std::convert::TryInto;
    use std::iter::FromIterator;
    use std::net::TcpStream;
    use std::sync::Arc;
    use std::sync::Mutex;
    //use std::time;
    use std::io::Read;
    use std::sync::MutexGuard;
    use std::sync::atomic::{AtomicBool,Ordering};
    use serde::{Serialize, Deserialize};
    //use crate::client::CMD1;

    
    const CMD1: [u8;4] = [0x01,0xaa,0x55,0x01];
    const CMD2: [u8;4] = [0x01,0xaa,0x55,0x02];

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
        fn on_read(&self, data: &MutexGuard<ClientData>){
            println!("On Read");
            let cmd: &[u8;4] = &data.recv_buffer[0..4].try_into().unwrap(); 
            print!("{:?}",cmd);
            print!("{:?}",CMD1);
            print!("{:?}",CMD2);

            //*
            match *cmd {
                CMD1=>{

                }
                CMD2=>{

                }
                _  =>{

                }
            }
            //*/

            /*
            if cmd==CMD1
            {
              println!("****************************** YES");  
            }
            */

            let ss = &data.recv_buffer[4..12];
            let s: String = String::from_utf8(ss.to_vec()).unwrap();
            println!("{}",s);
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
                    data.id=1;
                    println!("Client run {}",data.id);
                    data.recv_buffer=[0;1024];
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
                            data.id=1;
                            data.recv_buffer=buffer;
                            println!("Client run {}",(*data).id);
                            self.on_read(&mut data);
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