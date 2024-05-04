//Operators

fn main() {
    // Mathematical

    let addition = 23 + 54;
    let sub = 23-54;
    let mul = 23 * 54;
    let div = 54/23;
    let modu = 54%23;

    println!("Addition: {}\nSubtraction: {}\nMultplication: {}\nDivision: {}\nModulus: {}\n",addition,sub,mul,div,modu);

    //Logical
    let int1 = 50;
    let int2 = 78;

    if int1 == int2 {println!("Equal");}
    if int1 != int2 {println!("UnEqual");}
    if int1 < int2 {println!("Less Than");}
    if int1 > int2 {println!("More Than");}
    if int1 <= int2 {println!("Less than or Equal");}
    if int1 >= int2 {println!("More than or Equal");}
    
    let b1 = true;
    let b2 = false;

    if b1 && b2{println!("AND");}
    if b1 || b2{println!("OR");}

    // Bitwise

    let ii1 = 20;
    let ii2 = 15;

    println!("{} {} {} {} {}",ii1&ii2,ii1|ii2,ii1^ii2,ii1<<ii2,ii1>>ii2);

    
}
