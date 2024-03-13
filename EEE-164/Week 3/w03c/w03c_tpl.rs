use std::io;
use std::fmt;
use std::collections::HashMap;

// TODO: Create the following structs according to the specs here
// - SplitDate
// - LentItem
// - Borrower

impl LentItem {
    // TODO: Add the following LentItem methods according to the specs here.
    //       Note that you may need to add lifetime annotations to some or all
    //       of them.
    // - new
    // - borrow
    // - unborrow
    // - transfer
}

impl Borrower {
    // TODO: Add the following Borrower methods according to the specs here
    //       Note that you may need to add lifetime annotations to some or all
    //       of them.
    // - new
    // - borrowed_items

    // HINT: Read https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html
    //       to know more on how to sort a vector of structs
}

// This has been implemented for you as an example
impl fmt::Display for Borrower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Borrower({}) [Registered {}]", self.name, self.reg_date)
    }
}

impl fmt::Display for LentItem <'_> {
    // TODO: Implement the fmt() method for this struct according to the specs
}

impl fmt::Display for SplitDate {
    // TODO: Implement the fmt() method for this struct according to the specs
}

// No need to edit starting from this line!
fn main() {
    let mut str_in = String::new();
    let mut borrower_list: HashMap <String, Borrower> = HashMap::new();
    let mut items_list: HashMap <String, LentItem> = HashMap::new();

    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");

    let n_borrowers: usize = str_in.trim().parse().expect("Invalid number!");

    for _ in 0..n_borrowers {
        str_in.clear();
        io::stdin().read_line(&mut str_in)
            .expect("Invalid input!");

        let split_in: Vec <&str> = str_in.splitn(2, ' ').collect();
        let split_date: Vec <&str> = split_in[0].trim().splitn(3, '-').collect();

        borrower_list.insert(split_in[1].trim().to_string(), Borrower::new(
            split_in[1].trim().to_string(),
            split_date[0].parse::<u64>().expect(""),
            split_date[1].parse::<u8>().expect(""),
            split_date[2].parse::<u8>().expect(""),
        ));
    }

    str_in.clear();
    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");

    let n_items: usize = str_in.trim().parse().expect("Invalid number!");

    for _ in 0..n_items {
        str_in.clear();
        io::stdin().read_line(&mut str_in)
            .expect("Invalid input!");

        let split_in: Vec <&str> = str_in.splitn(2, ' ').collect();
        let split_date: Vec <&str> = split_in[0].trim().splitn(3, '-').collect();
        
        items_list.insert(split_in[1].trim().to_string(), LentItem::new(
            split_in[1].trim().to_string(),
            split_date[0].parse::<u64>().expect(""),
            split_date[1].parse::<u8>().expect(""),
            split_date[2].parse::<u8>().expect(""),
        ));
    }

    str_in.clear();
    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");

    let n_cmd: usize = str_in.trim().parse().expect("Invalid number!");

    for _ in 0..n_cmd {
        str_in.clear();
        io::stdin().read_line(&mut str_in)
            .expect("Invalid input!");

        let split_in: Vec <&str> = str_in.splitn(2, ' ').collect();
        let cmd: &str = split_in[0].trim();
        let subcmd: String = split_in[1].trim().to_string();

        match cmd {
            "bi" => {
                if let Some(borrower) = borrower_list.get(&subcmd) {
                    println!("[BINFO] {}", borrower);
                    println!("    -----BORROWED ITEMS-----");

                    let items_list_vals = items_list.values().collect();
                    let borrowed_items_list = borrower.borrowed_items(&items_list_vals);

                    if borrowed_items_list.is_empty() {
                        println!("    NONE");
                    }
                    else {
                        for v in &borrowed_items_list {
                            println!("    {}", v);
                        }
                    }
                }
                else {
                    println!("[BINFO] Borrower \"{}\" not found!", subcmd);
                }
            }
            "ii" => {
                if let Some(item) = items_list.get(&subcmd) {
                    println!("[IINFO] {}", item);
                }
                else {
                    println!("[IINFO] Item \"{}\" not found!", subcmd);
                }
            }
            "t" => {
                let split_subcmd: Vec <&str> = subcmd.splitn(3, ' ').collect();
                let item_t: String = split_subcmd[0].trim().to_string();
                let from_t: String = split_subcmd[1].trim().to_string();
                let to_t: String = split_subcmd[2].trim().to_string();

                if let None = items_list.get(&item_t) {
                    println!("[TRANSFER] Item \"{}\" not found!", item_t);
                    continue;
                }

                if let None = borrower_list.get(&from_t) {
                    println!("[TRANSFER] FROM borrower \"{}\" not found!", from_t);
                    continue;
                }

                if let None = borrower_list.get(&to_t) {
                    println!("[TRANSFER] TO borrower \"{}\" not found!", to_t);
                    continue;
                }

                if let (Some(item), Some(from_b), Some(to_b)) = (items_list.get_mut(&item_t), borrower_list.get(&from_t), borrower_list.get(&to_t)) {
                    let (old_from_b_w, new_b_w) = item.transfer(from_b, to_b);

                    if let Some(_) = new_b_w {
                        println!("[TRANSFER] Item \"{}\" transfered from \"{}\" to \"{}\"!", item.name, from_b.name, to_b.name);
                    }
                    else if let Some(old_from_b) = old_from_b_w {
                        if from_b.name == old_from_b.name {
                            println!("[TRANSFER] Item \"{}\" is already borrowed by requester \"{}\"!", item.name, to_b.name);
                        }
                        else {
                            println!("[TRANSFER] Item \"{}\" cannot be transferred as it is currently borrowed by \"{}\", not \"{}\"!", item.name, old_from_b.name, from_b.name);
                        }
                    }
                    else {
                        println!("[TRANSFER] Item \"{}\" does not have a borrower!", item.name);
                    }
                }
            }
            "b" => {
                let split_subcmd: Vec <&str> = subcmd.splitn(2, ' ').collect();
                let item_t: String = split_subcmd[0].trim().to_string();
                let borrower_t: String = split_subcmd[1].trim().to_string();

                if let None = items_list.get(&item_t) {
                    println!("[BORROW] Item \"{}\" not found!", item_t);
                    continue;
                }

                if let None = borrower_list.get(&borrower_t) {
                    println!("[BORROW] Borrower \"{}\" not found!", borrower_t);
                    continue;
                }

                if let (Some(item), Some(borrower)) = (items_list.get_mut(&item_t), borrower_list.get(&borrower_t)) {
                    if let Some(old_borrower) = item.borrow(&borrower) {
                        if old_borrower.name == borrower.name {
                            println!("[BORROW] Item \"{}\" already borrowed by requester and current borrower \"{}\"!", item.name, borrower.name);
                        }
                        else {
                            println!("[BORROW] Item \"{}\" cannot be borrowed by \"{}\" as it is currently borrowed by \"{}\"!", item.name, borrower.name, old_borrower.name);
                        }
                    }
                    else {
                        println!("[BORROW] Item \"{}\" borrowed by \"{}\"!", item.name, borrower.name);
                    }
                }
            }
            "u" => {
                if let Some(item) = items_list.get_mut(&subcmd) {
                    if let Some(borrower) = item.unborrow() {
                        println!("[UNBORROW] Item \"{}\" unborrowed from \"{}\"!", item.name, borrower.name);
                    }
                    else {
                        println!("[UNBORROW] Item \"{}\" has no borrower!", item.name);
                    }
                }
                else {
                    println!("[UNBORROW] Item \"{}\" not found!", subcmd);
                }
            }
            _ => {}
        }
    }
}