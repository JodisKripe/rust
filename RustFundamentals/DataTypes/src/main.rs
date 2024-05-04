// Two Data Subtypes: Scalar and Compound
// Scalar: Integers, Floating-Point Numbers, Booleans, Characters  --> primitive
// Compound: Tuples, Arrays --> i.e. collections

fn main() {
    // Integers and Floating Points
    let _int : i8 = -20;
    let _uint : u8 = 20;
    let _float : f32 = 20.0;
    let _float64 : f64 = 20.0;

    // Boolean and characters
    let _boolean : bool = true;
    let _charr : char = 'A';

    // Arrays and Tuples
    let _arr : [i32 ; 8] = [1,2,3,4,5,6,7,8];
    let _bigarr : [i32 ; 10000] = [0;10000];

    println!("{}\n",_arr[2]);
    println!("{}\n\n",_bigarr[2]);

    let tup : (&str,&str,&str,i8) = ("Arthur","Conan","Doyle",71);
    println!("{} {} {} was {} when he died",tup.0,tup.1,tup.2,tup.3);

    let(first_name , middle_name, last_name, age ) = tup;
    println!("{} {} {} was {} when he died",first_name,middle_name,last_name,age);

    // Strings
    // Two Types: String and &str(string slice)
    // String is mutable hence on heap, &str is immutable hence on stack

    let s:&str = "Arthur Conan Doyle";
    let str1:String = s.to_string();
    let str2:String = String::from(s);
    println!("{} {} {}",s,str1,str2);

    let astr:&str = &str1;
    let astr2:&str = str2.as_str();
    println!("{} {}",astr,astr2);

    // Concatenate
    let full_name = format!("{} {} {}",first_name,middle_name,last_name);
    println!("{}",full_name);

    let full__name = [first_name," ",middle_name," ",last_name].concat();
    println!("{}",full__name);

    let mut name:String = String::from("Jodis");
    name.push_str(" Kripe");

    let lname:&str = " Kripe";
    let fname:String = String::from("Jodis");
    let FNAME = fname + lname;
    
    println!("{} {}",name,FNAME);
}
