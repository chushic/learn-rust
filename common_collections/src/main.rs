mod strs;
mod hashmaps;

use strs::*;
use hashmaps::*;


fn main() {
    update_hashmap();
}


// pub fn drops_self() {
//     {
//         let v = vec![1, 2, 3, 4];
//         // do stuff with v
//     }// <- v goes out of scope and is freed
// }


// pub fn multiple_types() {
//     enum SpreadsheetCell {
//         Int(i32),
//         Float(f64),
//         Text(String),
//     }

//     let row = vec![
//         SpreadsheetCell::Int(3),
//         SpreadsheetCell::Text(String::from("blue")),
//         SpreadsheetCell::Float(10.12),
//     ];
// }




// pub fn iterate_vec() {
//     let mut v = vec![100, 32, 57];
//     for i in &mut v {
//         *i += 50;
//     }

//     for j in &v {
//         println!("{j}")
//     }
// }


// fn out_of_boundry() {
//     let v = vec![1, 2, 3, 4, 5];

//     let _does_not_exist = &v[100];
//     // let does_not_exist = v.get(100);
// }


// pub fn get_syn() {
//     let v = vec![1, 2, 3, 4, 5];
//     // let third: &i32 = &v[2];
//     let third: i32 = v[2];
//     println!("The third element is {third}");

//     let third: Option<&i32> = v.get(2);
//     match third {
//         Some(third) => println!("The third element is {}", third),
//         None => println!("No third element."),
//     }
// }


// pub fn vec_macro() -> Vec<i32>{
//     let v = vec![1, 2, 3];
//     v
// }