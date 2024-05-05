// Data Strructures and Traits
use std::{thread, time};


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
enum Status{
    ONLINE,
    OFFLINE
}

#[derive(Clone)]
struct System {
    os: String,
    processor: String,
    version: f32,
    status: Status
}

impl System{
    fn toggle(&mut self){
        if self.status == Status::ONLINE{
            self.status = Status::OFFLINE;
        }
        else{
            self.status = Status::ONLINE;
        }
    }
    fn getVersion(&self) -> f32{
        self.version
    }
}

impl System{
    fn new(os: String,processor: String, version: f32, status: Status) -> Self{
        Self{
            os,
            processor,
            version,
            status
        }
    }
}

fn details(S:&System) {
    match S.status{
        Status::ONLINE => println!("OS: {}\nProcessor: {}\nVersion: {}\nStatus: {:?}\n",S.os,S.processor,S.version,S.status),
        _ => println!("{} is down!\n",S.os)
    }
}

trait LoadingTime {
    fn sleepForLoadingTime(&self);
    fn printError(&self){
        println!("ERROR!!")
    }
}

impl LoadingTime for System{
    fn sleepForLoadingTime(&self){
        println!("{}",self.os);
        match self.os.as_str(){
            "Windows" => sleep(5),
            "Linux"   => sleep(2),
            "MacOS"   => sleep(3),
            &_        => println!("WHOOPSIE!!!")
        }
    }
}

fn sleep(timePeriod: u64){
    let millis = time::Duration::from_millis(timePeriod);
    let now = time::Instant::now();
    thread::sleep(millis);
    assert!(now.elapsed() >= millis);
    println!("System Booted!");
}

fn main() {
    let mut sys = System{
        os: String::from("Windows"),
        processor: String::from("AMD"),
        version: 5.2,
        status: Status::ONLINE
    };
    let mut sys2 = System{
        os: String::from("Linux"),
        processor: String::from("Intel"),
        version: 5.3,
        status: Status::OFFLINE
    };
    sys.toggle();
    sys2.toggle();
    details(&sys);
    details(&sys2);
    println!("{}",sys.getVersion());
    let mut s3 = System::new(String::from("MacOS"),String::from("M1"),5.1,Status::ONLINE);
    s3.toggle();
    s3.toggle();
    details(&s3);
    println!("{}\n\n",s3.getVersion());
    sys.sleepForLoadingTime();
    sys2.sleepForLoadingTime();
    s3.sleepForLoadingTime();
    sys.printError();
    sys2.printError();
    s3.printError();
}