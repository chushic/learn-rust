pub fn create_string() -> String {
    // let mut s = String::from("heyy");
    // s

    let data = "initial contents";
    let s = data.to_string();

    s
}

pub fn iterate_strings() {
    for c in "Зд".chars() {
        println!("{c}")
    }
    for c in "Зд".bytes() {
        println!("{c}")
    }
}

pub fn indexing_strings() {
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("{s}, {}", hello.len())

}

pub fn formatter() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s1} {s2} {s3} {s}")

}

pub fn update_string() {
    let mut s = String::from("foo");

    let s2 = "bar";
    s.push_str(s2);

    println!("s2 is {s2}");



    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2; // s1 is moved here, and can no longer be used
    println!("s3: {s3}");

    
}

