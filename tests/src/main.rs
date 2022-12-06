
use serde::{Serialize, Deserialize};
use std::net::TcpStream;
use std::io::Write;

use std::os;
use getopts::{optopt, optflag, getopts, usage};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}


fn main() {
    //Parto 1: unuopigo & malunuopigo
    let point = Point { x: 1, y: 2 };

    let serialized = serde_json::to_string(&point).unwrap();
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    println!("serialized = {}", serialized);
    println!("deserialized = {:?}", deserialized);
    println!("members are x={x}, y={y}", x=deserialized.x, y=deserialized.y);


    //Parto 2: Operaciumaj opcioj
    let args: Vec<String> = os::args();
    let program = args[0].clone();
    let opts = [
	optopt("h", "host", "gasto por alkonekto", "HOST"),
	optopt("f", "file", "dosiero por sendo", "FILE"),
	optopt("j", "json", "json data to send", "JSON"),
	optopt("p", "port", "port to end data to", "PORT"),
	optflag("", "help", "elpremu cxi tiu mesagxon")
	];
    
    
}
