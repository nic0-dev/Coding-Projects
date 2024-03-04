use std::io::*;

#[derive(Debug, Copy, Clone, PartialEq)] 
enum ItemType{                              
    Beverage,
    Cleaners,
    Electronics,
    Fruits,
    Meat,
}

#[derive(Debug, Copy, Clone, PartialEq)] 
struct GroceryItem{                         
    price: f64,
    weight: f64,
    item: Option <ItemType>
}

impl GroceryItem{
    fn set_item_type(&mut self, item_name: &str) -> bool {
        self.item = match item_name {
            "coke"    | "sprite" | "royal"         =>  Some(ItemType::Beverage),
            "bleach"  | "soap"                     =>  Some(ItemType::Cleaners),
            "battery" | "bulb"                     =>  Some(ItemType::Electronics),
            "banana"  | "mango"   | "strawberries" =>  Some(ItemType::Fruits),
            "beef"    | "chicken" | "pork"         =>  Some(ItemType::Meat),
            _ => return false,
        };
        true
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct SmartCart{                           
    max_budget: f64,
    max_weight: f64,
    items: [GroceryItem; 10],
    current_value: f64,
    current_weight: f64,
    current_size: usize,
}

impl SmartCart{
    fn new(max_budget: f64) -> Self {
        SmartCart{
            max_budget: max_budget,
            max_weight: 12.0,
            items: [GroceryItem { price: 0.0, weight: 0.0, item: None }; 10],
            current_value: 0.0,
            current_weight: 0.0,
            current_size: 0,
        }
    }

    fn add_item(&mut self, grocery_item: GroceryItem){ 
        if self.current_value + grocery_item.price > self.max_budget {
            println!("[SYSTEM] Maximum budget reached! Item unsuccessfully added.");
        } else if self.current_weight + grocery_item.weight > self.max_weight {
            println!("[SYSTEM] Maximum weight reached! Item unsuccessfully added.");
        } else if self.current_size == 10 {
            println!("[SYSTEM] Maximum number of items reached! Item unsuccessfully added.")
        } else {
            self.current_value += grocery_item.price;
            self.current_weight += grocery_item.weight;
            self.items[self.current_size] = grocery_item;
            self.current_size += 1;
            println!("[SYSTEM] Item successfully added!");
        }
    }

    fn remove_item(&mut self, index: usize){
        if index > self.current_size {
            println!("[SYSTEM] No item removed!");
        } else {
            let pivot = index - 1;
            self.current_value -= self.items[pivot].price;
            self.current_weight -= self.items[pivot].weight;

            for i in pivot..self.current_size - 1 {   // Shift items to the left
                self.items[i] = self.items[i + 1];
            }
            self.items[self.current_size - 1] = GroceryItem { price: 0.0, weight: 0.0, item: None };
            self.current_size -= 1;
            println!("[SYSTEM] Item removed!");
        }
    }

    fn show_info(self){
        println!("------GROCERY CART------");
        for i in 0..self.current_size {
            let _ = match self.items[i].item {
                Some(ItemType::Beverage) => println!("{}: Beverage", i+1),
                Some(ItemType::Cleaners) => println!("{}: Cleaners", i+1),
                Some(ItemType::Electronics) => println!("{}: Electronics", i+1),
                Some(ItemType::Fruits) => println!("{}: Fruits", i+1),
                Some(ItemType::Meat) => println!("{}: Meat", i+1),
                None => {},
            };
        }
        println!("Total price: Php {:.2}", self.current_value);
        println!("Total weight: {:.2} kg", self.current_weight);
        println!("------------------------");
    }
}


fn main(){
    let mut str_in = String::new(); 

    stdin().read_line(&mut str_in).expect("Invalid input!");
    let budget: f64 = str_in.trim().parse().expect("Not a number!"); 
    if budget < 0.0 || budget > 5000.00 {               // Input Constraint
        println!("[ERROR] Command not found!");
        return; 
    }

    str_in.clear();
    stdin().read_line(&mut str_in).expect("Invalid input!");
    let mut n_cmd: u64 = str_in.trim().parse().expect("Input is not a decimal number!");
    if n_cmd == 0 || n_cmd > 20 {                       // Input Constraint
        println!("[ERROR] Command not found!");
        return; 
    }

    let mut cart = SmartCart::new(budget);

    while n_cmd > 0 {
        str_in.clear();
        stdin().read_line(&mut str_in).expect("Invalid input!");
        let split_in: Vec <&str> = str_in.split_whitespace().collect();

        // Check number of args in input line, and check if command is add, remove, or show_info
        if vec![1, 2, 4].contains(&split_in.len()) { 
            match split_in[0] {
                "add" => {
                    let mut curr_item = GroceryItem { price: 0.0, weight: 0.0, item: None };
                    if !curr_item.set_item_type(&split_in[1]) {                         // Set Item Type
                        println!("[ERROR] Item not classified!");
                    } else { 
                        match split_in[2].trim().parse::<f64>() {
                            Ok(price) if price > 0.0 && price <= 750.0 => {             // Price is valid
                                match split_in[3].trim().parse::<f64>() {
                                    Ok(weight) if weight > 0.0 && weight <= 10.0 => {   // Weight is valid
                                        curr_item.price = price;
                                        curr_item.weight = weight;
                                        cart.add_item(curr_item); 
                                        n_cmd -= 1;
                                    },
                                    _ => println!("[ERROR] Weight error!"),             // Handle invalid weight
                                }
                            },
                            _ => println!("[ERROR] Price error!"),                      // Handle invalid price
                        }
                    }
                },
                "remove" => {
                    match split_in[1].trim().parse::<usize>() {
                        Ok(index) if index > 0 && index <= 10 => {
                            cart.remove_item(index);
                            n_cmd -= 1;
                        },
                        _ => println!("[ERROR] Index does not exist!"),                 // Handle invalid index
                    }
                },
                "show_info" => {
                    cart.show_info();
                    n_cmd -= 1;
                },
                _ => { println!("[ERROR] Command not found!"); }                        // Handle invalid command
            }
        } else {
            println!("[ERROR] Command not found!");                                     // Handle invalid command
        }
    }
}

// cat in_w02c_pub_s01.txt | ./w02c.exe | Out-File -Encoding UTF8 output.txt
// Compare-Object (gc output.txt) (gc out_w02c_pub_s01.txt)