fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("area is: {}", rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 13,
        height: 60,
    };

    println!("{}",rect1.can_hold(&rect2));
    println!("{}",rect1.can_hold(&rect3));
    }

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
        }
    }
