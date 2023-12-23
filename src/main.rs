use std::io;

fn main() {

    // function to get the player input and checks if it is correct
    fn get_player_input() -> String{
        let mut user_fighter = String::new();
        loop{
            println!("Please input a fighter: rock, paper, or scissors");
            io::stdin().read_line(&mut user_fighter).expect("User fighter input error");
            let user_fighter= user_fighter.trim();
            let user_fighter= user_fighter.to_lowercase();
            if user_fighter == "rock" {
                break;
            }else if user_fighter == "paper" {
                break;
            }else if user_fighter == "scissors" {
                break;
            } else {
                println!("Please put a valid fighter");
                continue;
            }
        }
        return user_fighter.to_uppercase()
    }

    println!("you choose {}", get_player_input());
}
