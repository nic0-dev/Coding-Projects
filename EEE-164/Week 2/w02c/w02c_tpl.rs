/*
    Note: the derive attribute, #[derive(Debug, Copy, Clone, Partial Eq)],
    placed at the beginning of enums and structs allows them to be printable
    using the "{:?}". Furthermore, this would allow them to be copied from
    memory and do logic operations on them. 
*/

use std::io;

#[derive(Debug, Copy, Clone, PartialEq)] 
enum ItemType{
    // TODO: declare ItemType variants
}

#[derive(Debug, Copy, Clone, PartialEq)] 
struct GroceryItem{
    // TODO: declare GroceryItem fields
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct SmartCart{
    // TODO: declare SmartCart fields
}

impl SmartCart{
    fn new(max_budget: f64) -> SmartCart{
        // TODO: Create a routine that creates a new instance of
        //       SmartCart with defaults according to the specs
    }

    fn add_item(&mut self, grocery_item: GroceryItem){ 
        // TODO: Create a routine that modifies the SmartCart when
        //       an item is added
    }

    fn remove_item(&mut self, index: usize){
        // TODO: Create a routine that modifies the SmartCart when
        //       an item is removed
    }

    fn show_info(self){
        // TODO: Create a routine that prints the contents of the
        //       cart according to specs.
}


fn main(){
    let mut str_in = String::new(); 

    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");
    let budget: f64 = str_in.trim().parse().expect("Not a number!"); 

    str_in.clear();
    io::stdin().read_line(&mut str_in)
        .expect("Invalid input!");
    let n_cmd: u64 = str_in.trim().parse().expect("Input is not a decimal number!");

    let mut cart = SmartCart::new(budget);
    
    // TODO: Create a routine that reads and executes the listed command, as 
    //       well as the error handling requirements. 
}