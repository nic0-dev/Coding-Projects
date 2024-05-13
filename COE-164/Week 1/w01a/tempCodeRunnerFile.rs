use std::io::*;

fn main() {
    let mut str_in = String::new();

    stdin().read_line(&mut str_in).expect("Invalid input!");

    let n_resistors: usize = str_in.trim().parse().expect("Invalid number!");

    let mut r_series = 0.0;
    let mut r_parallel = 0.0;

    for i in 1..=n_resistors {
        str_in.clear();
        stdin().read_line(&mut str_in).expect("Invalid input!");

        let r_val: f64 = str_in.trim().parse().expect("Invalid number!");
        r_series += r_val;
        if i == 1 {
            r_parallel = r_val;
        } else {
            r_parallel = (r_parallel * r_val)/(r_parallel + r_val);
        }
    }
    println!("{:.4}\n{:.4}", r_series, r_parallel);
}

// cat in_w01a_pub_s01 | ./w01a.exe | Out-File -Encoding UTF8 output.txt
// default UTF-16 -> change to UTF-8
// Compare-Object (gc output.txt) (gc out_w01a_pub_s01)

