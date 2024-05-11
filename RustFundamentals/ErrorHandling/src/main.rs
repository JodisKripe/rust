use std::fs;
use std::fs::File;
use std::io::{Error, Read};

fn main() {
    let vector:Vec<i32> = vec![1,2,3,4,5];
    for i in 0..=5{
        let option = vector.get(i);
        match option{
            None => println!("Nothings here"),
            Some(d) => println!("Here is {}",d)
        }
    }
    let result = fs::File::open("/tmp/partyAllNight");
    match result{
        Ok(_) => println!("File Opened"),
        Err(e) => println!("Error opening file: {}",e)
    }
    let result2 = fs::File::open("/etc/shadow");
    match result2{
        Ok(_) => println!("File Opened"),
        Err(e) => println!("Error opening file: {}",e)
    }
    println!("\n\n\n\n\n");
    let result3 = read_file("/etc/passwd");
    match result3{
        Ok(c) => println!("Contents are: {}",c),
        Err(e) => println!("Whoopsie!")
    }
    let result4 = read_file("/tmp/whopdoodle");
    match result4{
        Ok(c) => println!("Contents are: {}",c),
        Err(e) => println!("{}",e)
    }
    let result5 = read_file("/etc/lmao");
    match result5{
        Err(r) => {dbg!(r);},
        Ok(_) => println!("File Opened")
    }
}

fn read_file(path: &str) -> Result<String,Error>{
    let mut file = open_file(path)?;                // The Question mark here says that if the open_file returns some error, then we stop the execution of the code from here. and send the error to main.
    let mut buf = String::new();
    println!("runnning!");
    let _ = file.read_to_string(&mut buf);
    Ok(buf)                                         // If the code does reach this part then Ok(string) will be returned and not an error.
}

fn open_file(path: &str) -> Result<File,Error>{
    fs::File::open(path)
}
