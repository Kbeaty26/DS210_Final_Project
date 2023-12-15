mod bfs;
mod readfile;
mod friendrecs;
mod tests;

use crate::readfile::Graph;
use crate::friendrecs::recommendations;
use std::io;
use rand::seq::SliceRandom;

fn user_input(prompt: &str, range: std::ops::Range<usize>) -> usize {
    // This function is the basic outline to get numeric value from user input and handle various errors.
    loop {
    println!("{}", prompt);
    let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse() {
            Ok(vertex) if range.contains(&vertex) => return vertex,
            Ok(_) => println!("Invalid input. Please enter a valid number between {} and {}.", range.start, range.end -1),
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn main() {
    // Sets the initial distance to 2 since any distance of 0 would be itself and any distance of 1 would be friends the user already has.
    // Fills Graph struct based on the file being read.
    let mut distance = 2;
    let mut graph = Graph::new();
    graph.read_file("PTBR_edges.txt");

    // Prompts user to input their user id (since this is hypothetical any number in the range will work.)
    let start_vertex = user_input(
        &format!("Enter the your user id (must be between 0 and {}):", graph.n - 1), 0..graph.n);
    
    // Loop continues until user inputs n or there are no more friend recommendations.
    loop {
        // Finds recommended friendships based on information gathered from the file, a starting vertex (user id), and the desired connection distance.
        let recommended_friends = recommendations(&graph, start_vertex, distance);

        // Checks if the user has any recommended friends for the desired distance. If not exits from loop and code terminates.
        if recommended_friends.len() == 0 {
            println!("No friend recommendations. Exiting.");
            break;
        }

        // Prompts user to state how many of friendships they would like to display based on how many are avaliable.
        let num_friends = 
        user_input(
            &format!("You have a total of {} friend recommendations. How many would you like to display?", recommended_friends.len()),
            0..recommended_friends.len() + 1);

        // Collects the list of all the the recommended friends and shuffles the order to output random friends from the 
        // list based on the desired number by the user.
        let mut rng = rand::thread_rng();
        let all_friends = recommended_friends.iter().collect::<Vec<_>>();
        let mut shuffled_friends = all_friends.clone();
        shuffled_friends.shuffle(&mut rng);

        println!("Recommended friendships for {} with a distance of {}: {:?}", start_vertex, distance, shuffled_friends.iter().take(num_friends).collect::<Vec<_>>());

        // Prompts user to state if they would like to see friendships of farther connections, within the same distance, or neither. If niether then exits from loop and code terminates.
        println!("Would you like to display more friends from this distance or explore more distant friend connections? (s = same/d = distant/n = neither)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice = input.trim().to_lowercase();
        if choice == "d" {
            distance += 1;
        } else if choice == "s" {
            // If the choice is yes it will show more recommendations and displays the new total number of friends the user wants to display.
            // Prompts user with same question as before about checking friendships on same or different level or neither. 
            // If not same then break from inner loop and continue to conditions of outer loop.
            let mut remaining_friends = recommended_friends.len() - num_friends;

            while remaining_friends > 0 {
                // Checks if the length of the friends left in the list after displaying the user inputed amount is above 0.
                let more_friends = user_input(
                    &format!(
                        "You have a total of {} friend recommendations remaining. How many would you like to display?",
                        remaining_friends
                    ),
                    0..remaining_friends + 1,
                );

                println!("Displaying {} more friendships at distance {}: {:?}",more_friends,distance,shuffled_friends.iter().skip(num_friends).take(more_friends).collect::<Vec<_>>());
                
                remaining_friends -= more_friends;

                if remaining_friends > 0 {
                    loop {
                        println!(
                            "You have {} friend recommendations remaining. Would you like to display more friends from this distance or explore more distant friend connections? (s = same/d = distant/n = neither)",
                            remaining_friends
                        );
            
                        let mut input = String::new();
                        io::stdin().read_line(&mut input).expect("Failed to read line");
                        let inner_choice = input.trim().to_lowercase();
            
                        if inner_choice == "n" {
                            println!("Exiting.");
                            // Exit the outer loop as well, terminating the program.
                            return;
                        } else if inner_choice == "d" {
                            distance += 1;
                            break;
                        } else if inner_choice == "s" {
                            break;
                        } else {
                            println!("Invalid input. Please enter 's', 'd', or 'n'.");
                        }
                    }
                } else if choice == "n" {
                    println!("Exiting.");
                    break;
                } else {
                    println!("Please enter a valid input.")
                }
            }
        }
    }
}