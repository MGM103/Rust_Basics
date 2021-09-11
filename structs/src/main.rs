#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other_rec: &Rectangle) -> bool {
        self.width > other_rec.width && self.height > other_rec.height
    }

    //associated functions let you namespace functionality that is particular to your struct
    //without having an instance available.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {

    let rectangle1 = Rectangle {
        width: 50,
        height: 100,
    };

    let rectangle2 = Rectangle {
        width: 40,
        height: 10,
    };

    let rectangle3 = Rectangle {
        width: 51,
        height: 101,
    };

    let square1 = Rectangle::square(10);

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle1.area()
    );

    println!("Can rect2 fit in rect1: {}, can rect3 fit in rect1: {}", 
        rectangle1.can_hold(&rectangle2),
        rectangle1.can_hold(&rectangle3),
    );

    println!("My square: {:#?}", square1);
}


