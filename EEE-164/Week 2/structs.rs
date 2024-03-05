struct UserAcct {
    active: bool,
    username: String,
    sign_in_count: u64,
}

fn is_useracct_active(user: &UserAcct) -> bool {
    user.active
}

impl UserAcct {
    fn get_name(&self) -> &String {
        &self.username
    }

    fn is_active(self: &Self) -> bool {
        self.active
    }

    fn add_signup(&mut self) {
        self.sign_in_count += 1
    }

    fn prefix_name(&self, title: String) -> String {
        title + &String::from(" ") + &self.username.to_string()
    }
}

impl UserAcct {
    fn new() -> Self {
        Self {
            active: false,
            username: String::from(""),
            sign_in_count: 0,
        }
    }
}

fn main() {
    let mut user_a = UserAcct {
        active: true,
        username: "abcxyz".to_string(),
        sign_in_count: 0,
    };

    let user_b = UserAcct {
        username: "userB".to_string(),
        ..user_a
    };

    let user_c = UserAcct::new();

    user_a.sign_in_count += 1;
    
    println!("user_a: {} {} {}", user_a.active, user_a.username, user_a.sign_in_count);
    println!("user_b: {} {} {}", user_b.active, user_b.username, user_b.sign_in_count);
    println!("user_c: {} {} {}", user_c.active, user_c.username, user_c.sign_in_count);

    user_a.add_signup();
    println!("user_a_method: {} {}, is active (fn)? {} is active (method)? {}", user_a.get_name(), user_a.sign_in_count, is_useracct_active(&user_a), user_a.is_active());

    println!("{}", user_a.prefix_name(String::from("Mr.")));
}