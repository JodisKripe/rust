//Collections
// Sequence and Map Collections
// Sequence: Stores entirely sequential nad can only store one data type at a time.
// Map: key-value pairs, key and value dont need to be on the same data type.
use std::collections::HashMap;
use std::collections::BTreeMap;

fn main() {
    // Vectors
    let mut Vector:Vec<u8> = Vec::new();
    let mut Vector2:Vec<i8> = vec![];

    Vector.push(1);
    Vector.push(2);
    Vector.push(3);
    Vector.push(4);
    Vector.push(5);
    Vector.push(6);

    pront(Vector.clone());

    Vector.insert(3,99);
    pront(Vector.clone());
    Vector.remove(3);
    let i = Vector.pop();
    match i{
        Some(d) => println!("{}",d),
        None => println!("Nothing to pop bruh!")
    }

    println!("{}",Vector.contains(&3));
    println!("{}",Vector.contains(&99));

    for _some in 1..=Vector.len(){
        let data = Vector.pop();
        match data{
            Some(d) => println!("{}",d),
            None => println!("Nothing to pop bruh!")
        }
    }
    let data = Vector.pop();
    match data{
        Some(d) => println!("{}",d),
        None => println!("Nothing to pop bruh!")
    }


    //MAPSSSSSS
    let mut hmap:HashMap<u8,&str> = HashMap::new();
    let mut bmap:BTreeMap<u8,&str> = BTreeMap::new();

    hmap.insert(1,"Number 1");
    hmap.insert(2,"Number 2");
    hmap.insert(3,"Number 3");
    hmap.insert(4,"Number 4");
    hmap.insert(5,"Number 5");

    bmap.insert(1,"Number 1");
    bmap.insert(2,"Number 2");
    bmap.insert(3,"Number 3");
    bmap.insert(4,"Number 4");
    bmap.insert(5,"Number 5");

    for yada in hmap.iter(){
        println!("{}:{}",yada.0,yada.1);
    }
    println!("");
    for yada in bmap.iter(){
        println!("{}:{}",yada.0,yada.1);
    }
    
    let value = bmap.get(&123);
    match value {
        Some(v) => println!("{}",v),
        None => println!("OOpsie!!!")
    }

}

fn pront(vector:Vec<u8>){
    for data in vector.iter(){
                println!("{}",data);
        }
    println!(" ");
}