fn main() {
    // Read input file into an array of strings
    let games = include_str!("input.txt").lines().collect::<Vec<_>>();

    // Initalize a mutable list to store each power set to be summed later
    let mut power_sets = Vec::new();

    // Iterate over each game
    for game in games {
        // If the game is an empty string, skip it
        if game == "" {
            continue;
        }
        
        // Set up a dictionary to store the highest number of cubes pulled per color for each game
        let mut max_cubes = std::collections::HashMap::new();
        max_cubes.insert("red", 1);
        max_cubes.insert("green", 1);
        max_cubes.insert("blue", 1);

        // Separate each game into a list of pulls
        let pulls = game[game.find(":").unwrap()+2..].split(";").collect::<Vec<_>>();
        // Iterate over each pull
        for pull in pulls {
            // Separate each color pull into a list of colors
            let colors = pull.split(",").collect::<Vec<_>>();
            // Iterate over each color
            for color_pull in colors {
                // Separate the color pull into a list of items
                let items = color_pull.trim().split(" ").collect::<Vec<_>>();
                // Get the number of cubes pulled
                let count = items[0].parse::<i32>().unwrap();
                // Get the color of the cubes pulled
                let color = items[1];
                
                // If the number of cubes pulled exceeds the highest number of cubes pulled for that color, update the highest number of cubes pulled for that color
                if count > *max_cubes.get(color).unwrap() {
                    max_cubes.insert(color, count);
                }
            }
        }

        // Add the power set of the highest number of cubes pulled per color to the list of power sets
        power_sets.push(max_cubes["red"] * max_cubes["green"] * max_cubes["blue"]);
    }

    // Print the sum of the power sets
    println!("Sum of valid games: {}", power_sets.iter().sum::<i32>());
}