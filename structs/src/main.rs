fn main() {
    rectangles();
}
 
fn rectangles() {  
    let rect1 = Rectangle {
        width: 30, 
        height: 50,
    };

    println!("area of rect1 is {}", rect1.area());

}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width &&  self.height > other.height
    }
}


struct Rectangle {
    width: u32,
    height: u32,
}

// fn users() {
//     struct User {
//         active: bool,
//         username: String,
//         email: String,
//         sign_in_count: u64,
//     }

//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };
// }
