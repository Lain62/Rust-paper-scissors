use std::io;

// function to get the player input and checks if it is correct
fn get_player_input() -> String{
    loop{
        let mut user_fighter = String::new();
        println!("Please input a fighter: rock, paper, or scissors");
        io::stdin().read_line(&mut user_fighter).expect("User fighter input error");
        let user_fighter= user_fighter.trim().to_lowercase();
        match user_fighter.as_str() {
            "rock" | "paper" | "scissors" => break user_fighter.to_uppercase(),
            _ => println!("Please enter a valid fighter.")
        }
    }
}

fn main() {
    let user_fighter = get_player_input();
    println!("you choose {user_fighter}");
}
