use std::io::*;

fn main() {
    let mut str_in = String::new();

    stdin().read_line(&mut str_in).expect("Invalid input!");

    let n_testcases: u64 = str_in.trim().parse().expect("Not an integer!"); 

    // Pre-compute for the first 20 factorial values
    let mut factorial: [f64; 21] = [1.0; 21]; // Initialize the array with 1s
    for i in 1..21 {
        factorial[i] = factorial[i-1] * (i as f64);
    }
    
    for i in 1..=n_testcases {
        str_in.clear();
        stdin().read_line(&mut str_in).expect("Invalid input!");

        let split_in: Vec <&str> = str_in.splitn(3, ' ').collect();

        let c = split_in[0];
        let r_desired: f64 = split_in[1].trim().parse().expect("Not a float!");
        let x: f64 = split_in[2].trim().parse().expect("Not a number!");
        
        let mut ans: f64 = 0.0; 
        let mut n = 1;
        let mut temp: f64 = 1.0;
        // When function is sine and n = 0, taylor expansion = x 
        if c == "s" { temp = x; }

        // Solve for n
        while temp > r_desired {
            temp = x.powf(n as f64)/factorial[n];
            n += 1;
        }
        // println!("{} ", n);

        // Compute for the Taylor Expansion
        for j in 0..=n {
            let sign = if j % 2 == 0 { 1.0 } else { -1.0 };
            match c {
                "e" => ans += x.powf(j as f64) / factorial[j],
                "s" => ans += sign * x.powf((2*j + 1) as f64) / factorial[2*j + 1],
                "c" => ans += sign * x.powf((2 * j) as f64) / factorial[2 * j],
                _ => (),
            }
        } 
        println!("Function #{}: {:.2}", i, ans);
    }
}

// cat in_w01b_pub_s01 | ./w01b.exe | Out-File -Encoding UTF8 output.txt
// default UTF-16 -> change to UTF-8
// Compare-Object (gc output.txt) (gc out_w01b_pub_s01)