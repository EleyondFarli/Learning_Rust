#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
struct Point(i32, i32);
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.width <= other.width || self.height <= other.height {
            return false;
        }
        true
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }


    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }


    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area()
    );
    println!("rect 1 is {:#?}", rect1);

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: dbg!(50 * scale),
    };

    dbg!(&rect2);

    let rect1 = Rectangle { ..rect1 };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!(
        "The area of the square is {} square pixels",
        sq.area()
    );

    let p = Point(1, 2);


    impl Point {

        fn x(&self) -> i32 { self.0 }

    }



    println!("{}", p.x());
}
