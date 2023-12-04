mod num_in_line;
use num_in_line::NumInLine;

mod symbol_in_line;
use symbol_in_line::SymbolInLine;

fn main() {
    // Read in the input file 
    let input = include_str!("../input.txt");
}

// What have I learned?
// - cargo lets you create modules with new files and run projects in a similar way to the dotnet CLI
// - mod refers to a file, use refers to an imported member