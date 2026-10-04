use std::io::{self, Write};
// Define Bear as a struct, include location and fish collected
struct Bear {
    location: i32, // Int to represent location
    fish_collected: i32, // Int to represent how many fish you have
}
impl Bear {

    // Location search
    fn print_status(&self) {
        println!("------------------------------------------------");
        if self.location == 1 {
            println!("You are at the cozy den. It feels nice and safe, but your poor cub is missing.");
        } else if self.location == 2 {
            println!("You are at the rushing river. The water is cold, and you can see fish jumping. You wonder if your cub is hungry.");
        } else if self.location == 3 {
            println!("You are in the gloomy deep forest. The trees are thick, and it's getting dark.");
        } else if self.location == 4 {
            println!("You are at the dark cave. It looks spooky, but you hear something inside...");
        }
    }

    // Exploring function: Go forward and location + 1
    fn explore(&mut self) {
        if self.location < 4 {
            self.location = self.location + 1;
            println!("You bravely travel forward to the next area...");
        } else {
            println!("You're already at the end of the trail! There's nowhere else to go.");
        }
    }

    // Go back function: Go backward and location - 1
    fn go_back(&mut self) {
        if self.location > 1 {
            self.location = self.location - 1;
            println!("You turn around and head back to the last area...");
        } else {
            println!("You're already at the start of the trail! You can't go back any further.");
        }
    }

    // Catch fish function: fish_collected + 1
    fn catch_fish(&mut self) {
        if self.location == 2 {
            self.fish_collected = self.fish_collected + 1;
            println!("You reached into the water and caught a tasty salmon!");
            println!("Total fish collected: {}", self.fish_collected);
        } else {
            println!("There are no fish here. You need to be at the river to go fishing!");
        }
    }

    // Search function: If you have 3 fish and you're in the dark cave you can find your cub!
    fn search_for_cub(&self) -> bool {
        if self.location == 4 {
            println!("You sniff the dark air and hear a tiny whimper!");
            
            if self.fish_collected >= 3 {
                println!("You gently offer the {} fish you caught...", self.fish_collected);
                println!("The hungry cub comes out of the shadows! YOU WIN!");
                return true; //Player wins
            } else {
                println!("The cub is too scared to come out. Maybe if you had some food (at least 3 fish) to coax him out...");
                return false; 
            }
        } else {
            println!("You sniff around carefully, but there is no sign of the cub here.");
            return false;
        }
    }
}

fn main() {
    let mut player = Bear {
        location: 1,
        fish_collected: 0,
    };

    println!("=== MAMA BEAR'S JOURNEY ===");
    println!("Oh no! Your little cub has wandered off!");
    println!("You need to find him before the sun sets. Let's get moving!\n");
    
    // Main loop
    loop {
        player.print_status();

        // Possible options
        println!("\nWhat do you want to do?");
        println!("1. Explore further down the trail");
        println!("2. Go back the way you came");
        println!("3. Go fishing");
        println!("4. Search for the cub");
        println!("5. Give up and go to sleep (Quit)");
        print!("> ");
        
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Oops! Something went wrong reading your input.");
        let choice = choice.trim();

        println!(""); 

        if choice == "1" {
            player.explore();
        } 
        else if choice == "2" {
            player.go_back();
        } 
        else if choice == "3" {
            player.catch_fish();
        } 
        else if choice == "4" {
            // Check if the search method returned "true"
            let did_win = player.search_for_cub();
            if did_win {
                break;
            }
        } 
        else if choice == "5" {
            println!("You're too tired to keep going. You return home to rest. Game Over.");
            break;
        } 
        else {
            println!("Not a valid choice. Please type 1, 2, 3, 4, or 5.");
        }
    }
}