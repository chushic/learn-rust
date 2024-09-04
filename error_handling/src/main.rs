use std::fs::File;
use std::io::ErrorKind;

fn main() {
    open_file();
}

fn open_file() {
    let r = File::open("hello.txt");

    let f = match r {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem Creating the file {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        }
    };
}

// fn manual_panic() {
//     panic!("crash and burn");
// }

// fn passive_panic() {
//     let v = vec![1, 2, 3];
//     let i = &v[50];
//     println!("i: {i}");
// }
