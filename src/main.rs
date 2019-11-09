fn main() {
    let mut user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user.email = String::from("someone_else@example.com");
    println!("The value of user.username is: {}.", user.username); // someusername123

    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123")
    );
    println!("The value of user1.username is: {}.", user1.username); // someusername123

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("The value of user2.username is: {}.", user2.username); // someusername123

    let color = Color(0, 0, 0);
    println!("The value of the first color is: {}.", color.0); // 0

    let rect = Rectangle {
        width: 10,
        height: 10
    };
    let area = rect.area();
    println!("The value of area is: {}.", area); // 100 
    let rect1 = Rectangle {
        width: 7,
        height: 7
    };
    let rect1_fits = rect.can_hold(&rect1);
    println!("The value of rect1Fits is: {}.", rect1_fits); // true

    let square = Rectangle::square(8);
    let square_area = square.area();
    println!("The value of squareArea is: {}.", square_area); //  64
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
