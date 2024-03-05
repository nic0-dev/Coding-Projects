fn take(s: String) {
    println!("take: {s}");
}

fn take_borrow(s: &String) {
    println!("take_borrow: {s}");
}

fn take_copy(d: i64) {
    println!("take_copy: {d}");
}

fn take_give(s: String) -> String {
    println!("take_give: {s}");

    return s;
}

fn main() {
    let s = String::from("Hello!");
    let s2 = String::from("World!");
    let d = 7;
    
    take(s);
    take_copy(d);
    let s3 = take_give(s2);
    take_borrow(&s3);

    println!("{s3}"); // OK
}