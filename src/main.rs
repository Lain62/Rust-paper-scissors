use std::io;
use rand::Rng;

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

fn get_computer_input() -> String{
    let computer_selection: u32 = rand::thread_rng().gen_range(1..=3);
    match computer_selection {
        1 => return "rock".to_string(),
        2 => return "paper".to_string(),
        3 => return "scissors".to_string(),
        _ => return "Computer Calculation Error".to_string()
    }

}

fn main() {
    let user_fighter = get_player_input();
    let computer_fighter = get_computer_input();
    println!("you choose {}", user_fighter);
    println!("The bot choose {}", computer_fighter)
}
