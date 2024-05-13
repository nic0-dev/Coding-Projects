fn str_push_take_give(mut s: String) -> String {
    s.push_str(" World!");

    return s;
}

fn str_push_mut(s: &mut String) {
    s.push_str(" World!");
}

// FIXME: Code does not work because it returns a dangling
//        pointer
// fn str_push_mut_v2(s: &mut String) -> &String {
//     let a = "World!".to_string();
//     *s += &a;

//     return &a;
// }

fn main() {
    let r = "Hello".to_string();

    let mut r2 = str_push_take_give(r);
    println!("{r2}");

    // FIXME: Below line does not work. If we move it jsut before
    //        the "println!("{r3}")" line, it will work.
    // let r3 = &r2;

    str_push_mut(&mut r2);
    println!("R2: {r2}");

    // println!("{r3}");
}