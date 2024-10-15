#![crate_name = "rtreq"]
extern crate zmq;
extern crate rand;

use zmq::Context;
use std::{thread, time::{self, Duration}};
use rand::Rng;

const NBR_WORKERS: u32 = 10;



fn worker_task() {
    let context = Context::new();
    let worker = context.socket(zmq::REQ).unwrap();
    let mut rng = rand::thread_rng();
    let identity: u32 = rng.gen();

    worker.set_identity(&identity.to_be_bytes()).unwrap();

    worker.connect("tcp://localhost:5671").unwrap();

    let mut total = 0;

    println!("Started worker {}", identity);

    loop {
        worker.send("Hi Boss", 0).unwrap();
        let msg = worker.recv_msg(0).unwrap();
        let workload = msg.as_str().unwrap();
        println!("{}: {}", identity, workload);
        let finished = workload.eq("Fired!");
        if finished {
            println!("Complete: {} tasks", total);
            break;
        }
        total += 1;
        
        thread::sleep(time::Duration::from_millis(rng.gen_range(200, 500)));
    }
    
}


// While this example runs in a single process, that is only to make
// it easier to start and stop the example. Each thread has its own 
// context and conceptually acts as a separate process. 
fn main() {
    let context = Context::new();
    let broker = context.socket(zmq::ROUTER).unwrap();

    broker.bind("tcp://*:5671").unwrap();

    for _ in 1..NBR_WORKERS {
        thread::spawn(move || worker_task());
    } 

    // run for five seconds and then tell the workers to end
    let end_time = time::Instant::now() + Duration::from_secs(5);
    let mut workers_fired = 0;

    loop {
        let id = broker.recv_msg(0).unwrap(); // identity
        broker.recv_msg(0).unwrap(); // envelope delimiter
        let m = broker.recv_msg(0).unwrap(); // response from worker

        println!("{:?}, {}",id,  m.as_str().unwrap());
        if time::Instant::now() < end_time {
            broker.send("Work Harder", 0).unwrap();
        }
        else {
            println!("fired");
            broker.send("Fired!", 0).unwrap();
            workers_fired += 1;
            if workers_fired == NBR_WORKERS {
                break;
            }
        }
    }
}
