use std::io::*;

struct Player {
    name: String,                   // Player Name
    pos: i64,                       // Player's Current Position
    item: Option <PlayerItem>,      // Optional Item of the player
}

struct PlayerItem {
    name: String,                   // Item Name
    item_type: PlayerItemQtyType,   // Type and Quantity of the Item
}

enum PlayerItemQtyType {
    Once,                           // Item can be used Once
    Consumable(u64),                // Item Quantity ()> 1)
}

impl PlayerItemQtyType {
    fn get_qty(&self) -> u64 {      // Returns the quantity of the item
        match self {
            PlayerItemQtyType::Consumable(qty) => *qty,
            PlayerItemQtyType::Once => 0,
        }
    }

    fn decrement_qty(&mut self) {   // Decreases the quantity by 1, if consumable; Updates item type if quantity is 1
        if let PlayerItemQtyType::Consumable(qty) = self {
            *self = if *qty > 1 { PlayerItemQtyType::Consumable(*qty - 1) } else { PlayerItemQtyType::Once };
        }
    }
}

fn main() {
    let mut str_in = String::new();
    stdin().read_line(&mut str_in).expect("Invalid input!");
    let test_case: u64 = str_in.trim().parse().expect("Not an integer!"); 

    // Iterate each Test Case
    for i in 1..=test_case {
        str_in.clear();
        stdin().read_line(&mut str_in).expect("Invalid input!");
        let split_in: Vec <&str> = str_in.splitn(3, ' ').collect();
        
        // Extract from input
        let (p_name, ui_name, ui_qty) = (split_in[0], split_in[1], split_in[2].trim().parse::<u64>().expect("Not a number!"));

        // Instantiate Player 
        let mut my_player = Player {
            name: p_name.to_string(),
            pos: 0,
            item: match ui_qty {
                0 => None,              // If Quantity is 0
                _ => Some(PlayerItem {  
                    name: ui_name.to_string(),
                    item_type: if ui_qty == 1 { PlayerItemQtyType::Once } else { PlayerItemQtyType::Consumable(ui_qty) },
                }),
            },
        };
        
        // Print Player Info
        println!("Player #{}:\nName: {}", i, my_player.name);
        match &my_player.item {
            Some(player_item) => println!("Item: {}x {}", player_item.item_type.get_qty(), player_item.name),
            None => println!("Item: NONE"),
        }
        println!("----------LOG----------");

        // Process Commands
        str_in.clear();
        stdin().read_line(&mut str_in).expect("Invalid input!");
        let n_cmd: u64 = str_in.trim().parse().expect("Not an integer!");

        for _ in 0..n_cmd {
            str_in.clear();
            stdin().read_line(&mut str_in).expect("Invalid input!");
            let cmd = str_in.trim();

            match cmd {
                "left" => { // Move Player to the Left
                    my_player.pos -= 1;     
                    println!("New position: {}", my_player.pos);
                }
                "right" => {// Move Player to the Right
                    my_player.pos += 1;
                    println!("New position: {}", my_player.pos);
                }
                _ => {      // Handle Item usage
                    if let Some(player_item) = &mut my_player.item {
                        player_item.item_type.decrement_qty();
                        let new_qty = player_item.item_type.get_qty();
                    
                        match new_qty {
                            0 => println!("Player used <{}>. It is now gone", player_item.name),
                            _ => println!("Player used <{}>. {}x of <{}> remains.", player_item.name, new_qty, player_item.name),
                        }
                    } else {
                        println!("Cannot use item as player does not have one.");
                    }
                    
                }
            }
        }   
    }
}

// cat in_w02a_pub_s01 | ./w02a.exe | Out-File -Encoding UTF8 output.txt
// default UTF-16 -> change to UTF-8
// Compare-Object (gc output.txt) (gc out_w02a_pub_s01)