//use std::thread;
//use std::thread::JoinHandle;
//use std::time::Duration;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use servlib::worker::worker::{Worker};
use servlib::listener::listener::{Listener};

use std::{thread, time};

use config::Config;
use std::collections::HashMap;

//extern crate servlib;




fn main() 
{   
    

    let settings = Config::builder().add_source(config::File::with_name("/home/dato/wserv.json")).build().unwrap();

    print!("{:?}",settings.try_deserialize::<HashMap<String,String>>());

    let mut listener =Worker::<Listener>::new();

    
    listener.start(None);
    

    //* Commented Temporary

    let term = Arc::new(AtomicBool::new(false));
    let  res = signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term));
    match res {
        Ok(_) => println!("Signal register OK"),
        Err(e) => println!("Signal register Error{} ",e.to_string()),

    }
    //*/
    
    // TEST

    let mut delay = time::Duration::from_millis(5000); 
    thread::sleep(delay);
   

    let a=listener.worker_type.clone();
    for _i in 0..5{
        
        let b =&a.data;
        {
            let mut data_guard = b.lock().unwrap();
            (*data_guard).id=(*data_guard).id+1;
        }
    }
    
    delay = time::Duration::from_millis(2000); 
    thread::sleep(delay);
    {
        let b =&a.data;
        let  data_guard = b.lock().unwrap();
        println!("========={}",(*data_guard).id);
    }

    

    //

    
    //* Commented temporary
    while !term.load(Ordering::Relaxed)
    {
       //todo
       //listener.stop();
    }
    //*/
    
    println!("END");
      
}
