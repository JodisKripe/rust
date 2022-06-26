use std::io;

#[derive(Debug)]
struct Rectangle {
    length:u32,
    breadth:u32,
}

fn main() {
    #[derive(Debug)]
    struct user{
        active : bool,
        username : String,
        email : String,
        sign_in_count: i32,
    }

    let mut user1 = user{
        active: true,
        username: String::from("user1"),
        email: String::from("user1@tomail.com"),
        sign_in_count: 5,
    };

    user1.email = String::from("user1@tomaill.com");
    println!("{}",user1.email);

    let user2 = user {
        username: String::from("user2"),
        email: String::from("user2@tomaill.com"),
        ..user1
    };

    println!("{} {} {} {}\n\n\n",user2.username,user2.email,user2.active,user2.sign_in_count);
    ///////////////////////////////////////////////////////////////////////////////////
    println!("Enter length of the rectangle: ");
    let mut length = String::new();
    io::stdin()
        .read_line(&mut length)
        .expect("Error");
    let mut breadth = String::new();
    println!("Enter breadth of the rectangle: ");
    io::stdin()
        .read_line(&mut breadth)
        .expect("Error");
    let length = length.trim().parse().expect("The Value isnt valid");
    let breadth = breadth.trim().parse().expect("The Value isntn Valid");
    let area:i32 = rectMult((length,breadth));
    println!("\n{}",area);
    let mut rect = (length,breadth);
    let rectangle1 = Rectangle{
        length:rect.0 as u32,
        breadth:rect.1 as u32,
    };
    let A = Area(&rectangle1);
    println!("{}\n{}\n\n",A,rectangle1.rectArea());
    println!("rectangle1 is {:?}",rectangle1);
    println!("user1 is {:#?}",user1);
    ///////////////////////////////////////////////////////////////////////////////////
    let sqr = Rectangle::square(4);
    let rect1 = Rectangle {
        breadth: 30,
        length: 50,
    };
    let rect2 = Rectangle {
        breadth: 10,
        length: 40,
    };
    let rect3 = Rectangle {
        breadth: 60,
        length: 45,
    };
    println!("#####");
    println!("{:#?}",sqr);
    println!("Can rect1 hold rect2?: {}\nCan rect2 hold rect3: {}",rect1.can_hold(&rect2),rect2.can_hold(&rect3));

}

impl Rectangle{
    fn rectArea(&self) -> u32{
        self.length * self.breadth
    }
}
impl Rectangle{
    fn square(size:i32) -> Rectangle{
        Rectangle{
        length:size as u32,
        breadth:size as u32,
        }
    }
}

impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.breadth > other.breadth && self.length > other.length
    }
}

fn Area(rectangle: &Rectangle) -> u32{
    let prod = dbg!(rectangle.length * rectangle.breadth);
    prod
}

fn rectMult(dimensions: (i32,i32)) -> i32{
    let area:i32 = dimensions.0 * dimensions.1;
    return area
}



