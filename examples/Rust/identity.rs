#![crate_name = "identity"]

extern crate zmq;

use zmq::Context;

fn main() {
    let ctx = Context::new();
    let sink = ctx.socket(zmq::ROUTER).unwrap();
    sink.bind("inproc://example").unwrap();


    // First allow 0MQ to set the identity
    let anonymous = ctx.socket(zmq::REQ).unwrap();
    anonymous.connect("inproc://example").unwrap();
    anonymous.send("ROUTER uses a generated 5 byte identity", 0).unwrap();


    // Then set the identity ourselves
    let identified = ctx.socket(zmq::REQ).unwrap();
    identified.set_identity(b"PEER2").unwrap();
    identified.connect("inproc://example").unwrap();
    identified.send("ROUTER socket uses REQ's socket identity", 0).unwrap();

    // print messages received in router
    while let Ok(msg) = sink.recv_msg(0) {
        match msg.as_str() {
            None => println!("{:?}", msg), // handles generated 5 byte identity
            Some(s) => println!("{}", s) // all other cases which can be interpreted as UFT-8 strings
        }
    }
}
