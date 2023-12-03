fn main() {
    // Set up game rules
    const RED_COUNT: i32 = 12;
    const GREEN_COUNT: i32 = 13;
    const BLUE_COUNT: i32 = 14;
    
    // Read input file into an array of strings
    let games = include_str!("input.txt").lines().collect::<Vec<_>>();

    // Initalize a mutable list to store each valid game
    let mut valid_games = Vec::new();

    // Iterate over each game
    for game in games {        
        // If the game is an empty string, skip it
        if game == "" {
            continue;
        }        
        
        // First determine the game id. This is the first number in the string found after the first space, before the colon
        let game_id = game[game.find(" ").unwrap()+1..game.find(":").unwrap()].parse::<i32>().unwrap();
        
        // Separate each game into a list of pulls
        let pulls = game[game.find(":").unwrap()+2..].split(";").collect::<Vec<_>>();
        // Ensure each pull doesnt exceed the total number of cubes per color
        let mut valid_game = true;
        for pull in pulls {
            let colors = pull.split(",").collect::<Vec<_>>();
            for color_pull in colors {  
                let items = color_pull.trim().split(" ").collect::<Vec<_>>();
                let count = items[0].parse::<i32>().unwrap();
                let color = items[1];
                
                // If the number of cubes pulled exceeds the total number of cubes per color, the game is invalid
                if color == "red" && count > RED_COUNT {
                    valid_game = false;
                    break;
                }
                else if color == "green" && count > GREEN_COUNT {
                    valid_game = false;
                    break;
                }
                else if color == "blue" && count > BLUE_COUNT {
                    valid_game = false;
                    break;
                }
            }
        }

        // If the game is valid, add it to the list of valid games
        if valid_game {
            valid_games.push(game_id);
        }
    }

    // Print the sum of valid games
    println!("Sum of valid games: {}", valid_games.iter().sum::<i32>());
}

// What have I learned?
// - include_str!() loads data at compile time, while fs::read_to_string() loads data at runtime
// - Several Rust macros return an Option type, which can be unwrapped using .unwrap(), similar to C#'s null-coalescence pattern
// - Rust has an interesting way of handling Generic types - when passing to a function, 
//   several functions seem to use c++ style syntax to specify the type - e.g. .parse::<i32>()
// - "_" is still a discard during assignment, but can also be used for duck typing - e.g. Vec<_>
// - The difference between *, &, and && is confusing. It doesn't seem to be similar to other reference operators in other languages