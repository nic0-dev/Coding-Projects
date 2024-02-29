use std::io;

struct StudentEnrollInfo {
    sn: u64,
    is_eligible: bool,
    has_accountables: bool,
    has_taken_ge2017: [bool; 10],
}

impl StudentEnrollInfo {
    fn new() -> Self {
        // HINT: Create a routine that populates a new instance of
        //       StudentEnrollInfo with defaults according to the specs
        todo!("Create")
    }

    fn check_ge2017(&mut self, course: String) -> bool {
        // HINT: Create a routine that edits self.has_taken_ge2017
        //       depending on the course argument
        let ge_2017 = [
            "arts1", "fil40", "kas1", "philo1", "eng13",
            "speech30", "sts1", "drmaps", "socsci2", "socsci1"];
    
        return false;
    }

    fn print_unsatisfied_ge2017(&self) -> bool {
        // HINT: Create a routine that prints the state of self.has_taken_ge2017
        //       according to the specs
        let ge_2017 = [
            "arts1", "fil40", "kas1", "philo1", "eng13",
            "speech30", "sts1", "drmaps", "socsci2", "socsci1"];
        
        return false;
    }
}

fn main() {
    let mut str_in = String::new();

    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");

    let n_students: usize = str_in.trim()
        .parse()
        .expect("Invalid number!");

    for t in 1..=n_students {
        str_in.clear();
        io::stdin().read_line(&mut str_in)
            .expect("Invalid input!");

        let mut a_student = StudentEnrollInfo::new();

        let split_in: Vec <&str> = str_in.splitn(2, ' ').collect();

        let n_cmd: u64 = split_in[0].trim().parse().expect("Not a number!");
        let sn: u64 = split_in[1].trim().parse().expect("Not a number!");

        for _ in 0..n_cmd {
            str_in.clear();
            io::stdin().read_line(&mut str_in)
                .expect("Invalid input!");

            let split_in: Vec <&str> = str_in.splitn(2, ' ').collect();

            let cmd: char = split_in[0].trim().parse().expect("Not a character!");
            let cmd_arg = split_in[1].trim();

            // TODO: Create a routine that populates a struct with the provided values
            //       either through methods or initialization
            // TODO: Create a routine that prints to the standard output according to
            //       the specs
        }
    }
}