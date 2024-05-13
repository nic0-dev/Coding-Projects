use std::io::*;

struct StudentEnrollInfo {
    sn: u64,
    is_eligible: bool,
    has_accountables: bool,
    has_taken_ge2017: [bool; 10],
}

impl StudentEnrollInfo {
    fn new() -> Self {
        StudentEnrollInfo {
            sn: 0,
            is_eligible: false,
            has_accountables: true,
            has_taken_ge2017: [false; 10],
        }
    }

    fn check_ge2017(&mut self, course: &str) -> bool {
        let ge_2017 = ["arts1", "fil40", "kas1", "philo1", "eng13", "speech30", "sts1", "drmaps", "socsci2", "socsci1"];
        for i in 0..10 {
            if course == ge_2017[i] {
                self.has_taken_ge2017[i] = true;
                return true;
            }
        }
        return false;
    }

    fn print_unsatisfied_ge2017(&self) -> bool {
        let ge_2017 = ["arts1", "fil40", "kas1", "philo1", "eng13", "speech30", "sts1", "drmaps", "socsci2", "socsci1"];
        let mut unsatisfied_courses = Vec::new();

        for i in 0..9 {
            if i < 4 && !self.has_taken_ge2017[i] {
                unsatisfied_courses.push(ge_2017[i].to_uppercase());
            } else if i >= 4 && i%2 == 0 && !self.has_taken_ge2017[i] && !self.has_taken_ge2017[i+1] {
                unsatisfied_courses.push(format!("{}/{}", ge_2017[i].to_uppercase(), ge_2017[i+1].to_uppercase()));
            }
        }
        if !unsatisfied_courses.is_empty() {
            print!("{}", unsatisfied_courses.join(" "));
            return false;
        }
        return true;
    }
}

fn main() {
    let mut str_in = String::new();

    stdin().read_line(&mut str_in).expect("Invalid input!");
    let n_students: usize = str_in.trim().parse().expect("Invalid number!");

    for t in 1..=n_students {
        str_in.clear();
        stdin().read_line(&mut str_in).expect("Invalid input!");

        let mut a_student = StudentEnrollInfo::new();

        let split_in: Vec <&str> = str_in.splitn(2, ' ').collect();

        let n_cmd: u64 = split_in[0].trim().parse().expect("Not a number!");
        let sn: u64 = split_in[1].trim().parse().expect("Not a number!");

        a_student.sn = sn;

        for _ in 0..n_cmd {
            str_in.clear();
            stdin().read_line(&mut str_in).expect("Invalid input!");

            let split_in: Vec <&str> = str_in.splitn(2, ' ').collect();

            let cmd: char = split_in[0].trim().parse().expect("Not a character!");
            let cmd_arg = split_in[1].trim();

            match cmd {
                'e' => { a_student.is_eligible = cmd_arg == "y"; },
                'a' => { a_student.has_accountables = cmd_arg == "y"; },
                'c' => { a_student.check_ge2017(&cmd_arg); },
                _ => {}
            }
        }
        if t > 1 { print!("\n") };
        println!("Student #{}:", t);
        println!("Record for SN {}", sn);
        println!("    Is eligible? {}", if a_student.is_eligible { "YES" } else { "NO" });
        println!("    Is accountable? {}", if a_student.has_accountables { "YES" } else { "NO" });
        print!("    Unsatisfied GE2017 Courses:\n    ");
        a_student.print_unsatisfied_ge2017();
    }
}

// cat in_w02b_pub_s01 | ./w02b.exe | Out-File -Encoding UTF8 output.txt
// Compare-Object (gc output.txt) (gc out_w02b_pub_s01)