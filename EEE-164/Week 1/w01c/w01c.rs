use std::io::*;

fn get_network_from_prefix(prefix: u64, last_digit: u64) -> u64 {
    let globe_tm_prefix_1 = vec![817, 905, 906, 915, 916, 917, 926, 927, 935, 936, 937, 945, 953, 954, 955, 956, 965, 966, 967, 975, 976, 977, 978, 979, 995, 996, 997];
    let globe_tm_prefix_2 = vec![9173, 9175, 9176, 9178, 9253, 9255, 9256, 9257, 9258];
    let smart_sun_tnt_prefix = vec![922, 923, 924, 925, 931, 932, 933, 934, 940, 941, 942, 943, 973, 974, 907, 909, 910, 912, 930, 938, 946, 948, 950, 908, 918, 919, 920, 921, 928, 929, 939, 946, 947, 949, 951, 961, 998, 999];
    let dito_prefix = vec![895, 896, 897, 898, 991, 992, 993, 994];
    
    // println!("{} {}\n", prefix, last_digit);
    if globe_tm_prefix_1.contains(&prefix) || globe_tm_prefix_2.contains(&(prefix*10 + last_digit)){
        return 1;
    } else if smart_sun_tnt_prefix.contains(&prefix) {
        return 2;
    } else if dito_prefix.contains(&prefix) {
        return 3;
    }
    return 0;
}

fn main() {
    let mut str_in = String::new();

    str_in.clear();
    stdin().read_line(&mut str_in).expect("Failed to read input");

    let n_testcases: u64 = str_in.trim().parse().expect("Input is not an integer!");

    for t in 1..=n_testcases {
        str_in.clear();
        stdin().read_line(&mut str_in).expect("Failed to read input");
        
        let chars: Vec<char> = str_in.chars().collect();
        let prefix: String = chars[1..4].iter().collect();
        let uid_1: String = chars[4..7].iter().collect();
        let uid_2: String = chars[7..11].iter().collect();

        let prefix_as_int: u64 = prefix.trim().parse().expect("Input is not an integer!");
        let unique_id_as_int: u64 = uid_1.chars().next().unwrap().to_digit(10).expect("Input is not an integer!") as u64;

        let network = get_network_from_prefix(prefix_as_int, unique_id_as_int);

        match network {
            1 => println!("Case #{t}: Globe/TM | +63 {prefix} {uid_1} {uid_2}"),
            2 => println!("Case #{t}: Smart/Sun/TNT | +63 {prefix} {uid_1} {uid_2}"),
            3 => println!("Case #{t}: DITO | +63 {prefix} {uid_1} {uid_2}"),
            _ => println!("Case #{t}: Invalid"),
        }
    }
}

// cat in_w01c_pub_s01 | ./w01c.exe | Out-File -Encoding UTF8 output.txt
// default UTF-16 -> change to UTF-8
// Compare-Object (gc output.txt) (gc out_w01c_pub_s01)