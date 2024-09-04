fn main() {
    slices();
}

fn slices() {
    let s = String::from("hello");

    let slice = &s[0..s.len()];
    println!("slice: {slice}");

}


// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         } 
//     }

//     &s[..]
// }

// fn references() {
//     let mut s1 = String::from("hello");

//     // create a reference to variables don't take ownership
//     let len = calculate_length(&s1);
//     println!("The length of '{s1}' is {len}");

//     let r1 = &mut s1;

// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// fn change(s: &mut String) {
//     s.push_str(", world");
// }

// fn calculate_length(s: &String) -> usize { // s is a reference to String
//     s.len()
// }

// fn heap_data() {
//     let s1: String = String::from("hello");
//     let s2: String = s1.clone();

//     println!("s1 = {s1}, s2 = {s2}");
// }

// fn stack_data() {
//     let x: u32 = 5;
//     let y = x;

//     println!("x = {x}, y = {y}");
// }

// fn ownership() {
//     let s = String::from("hello");

//     takes_ownership(s); // s lost ownership

//     let x = 5;

//     makes_copy(x); // x still valid

//     let s1 = gives_ownership(); 



// }

// fn takes_ownership(some_string: String) {
//     println!("{some_string}");
// }

// fn makes_copy(some_integer: i32) {
//     println!("{some_integer}");
// }

// fn gives_ownership() -> String {
//     let some_string = String::from("yours");
//     some_string
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string
// }
