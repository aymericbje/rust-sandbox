use std::{char, collections::HashMap};

pub struct Plugboard
{
    plugboard:HashMap<char, char>
}
impl Plugboard {
    pub fn new(pairs: &[(char, char); 10]) -> Self
    {
        let mut all_pairs: Vec<(char, char)> = Vec::new();
        // Create an array with same pairs but swapped
        for pair in pairs
        {
            all_pairs.push((pair.0, pair.1));
            all_pairs.push((pair.1, pair.0));
        }
        let mut p = HashMap::new();
        for pair in all_pairs
        {
            p.insert(pair.0, pair.1);
        }
        Plugboard {
            plugboard: p
        }
    }

    pub fn swap(&self, c:char) -> char
    {
        let mut output_char: char = c;
        if self.plugboard.contains_key(&c)
        {
            output_char = self.plugboard[&c]
        }
        return output_char
    }

    pub fn display_plugboard(&self)
    {
        for key in self.plugboard.keys()
        {
            println!("{} -> {}", key, self.plugboard[key]);
        }
    }
}

// fn create_plugboard() -> HashMap<char, char>
// {
//     let mut plugboard = HashMap::new();
//     plugboard.insert('B', 'Q');
//     plugboard.insert('Q', 'B');
//     plugboard.insert('C', 'R');
//     plugboard.insert('R', 'C');
//     plugboard.insert('D', 'I');
//     plugboard.insert('I', 'D');
//     plugboard.insert('E', 'J');
//     plugboard.insert('J', 'E');
//     plugboard.insert('K', 'W');
//     plugboard.insert('W', 'K');
//     plugboard.insert('M', 'T');
//     plugboard.insert('T', 'M');
//     plugboard.insert('O', 'S');
//     plugboard.insert('S', 'O');
//     plugboard.insert('P', 'X');
//     plugboard.insert('X', 'P');
//     plugboard.insert('U', 'Z');
//     plugboard.insert('Z', 'U');
//     plugboard.insert('G', 'H');
//     plugboard.insert('H', 'G');

//     return plugboard;
// }



fn main() {
    // Init plugboard
    let plugboard = Plugboard::new(
        &[('B', 'Q'), ('C', 'R'), ('D', 'I'), ('E', 'J'), ('K', 'W'), ('M', 'T'), ('O', 'S'), ('P', 'X'), ('U', 'Z'), ('G', 'H')]
    );
    plugboard.display_plugboard();
    println!("{}", plugboard.swap('H'));
}

// STEPS for Enigma M3

/* INITIALIZE */
// Plugboard (10 different associations (20 letters))
// 3 rotors to choose (8 available) with a random order
// initial positions of rotors

/* CIPHERING */
// 1 - User input
// 2 - Plugboard
// 3 - rotors 
// 4 - reflector
// 5 - rotors
// 6 - plugboard

