use std::io;

const ALPHABET_MIN:u8 = b'A';
const ALPHABET_MAX:u8 = b'Z';

fn menu(){
    let mut choix: String = String::new();

    println!("This program allows you to cipher/decipher words or sentence with latin alphabet (A-Z), 
    lowercase are converted to uppercase");

    println!("Menu :
    1 - Cipher
    2 - Decipher
    3 - Brute force (Display all possibility)");
    io::stdin().read_line(&mut choix).expect("Echec lors de l'entrée utilisateur");
    let choix:u8 = choix.trim().parse().expect("Un nombe était attendu");

    match choix {
        1 => ciphering(),
        2 => deciphering(),
        3 => brute_forcing(),
        _ => println!("Please choose '1', '2' or '3'"),
    }
}

fn ciphering()
{
    let mut shifting = String::new(); // user entry
    let mut message = String::new(); // user entry
    let mut ciphering_char:u8; // character after ciphering

    println!("Message to cipher : ");
    io::stdin().read_line(&mut message).expect("Failed on user input");
    let message = message.to_uppercase();

    println!("Shifting to apply : ");
    io::stdin().read_line(&mut shifting).expect("Failed on user input");
    let shifting:u8 = shifting.trim().parse().expect("A number was expected");

    if shifting > ALPHABET_MAX - ALPHABET_MIN 
    {
        println!("Number must be lower than {}", ALPHABET_MAX - ALPHABET_MIN);
    }
    let mut ciphered_message: Vec<char> = Vec::new();
    println!("message : {}", message);

    for c in message.bytes() 
    {
        if (c + shifting) > ALPHABET_MAX
        {
            ciphering_char = (ALPHABET_MIN + shifting) - ((ALPHABET_MAX + 1) - c);
        }
        else 
        {
            ciphering_char = c + shifting;
        }
        ciphered_message.push(ciphering_char as char);
    }

    print!("Ciphered message: ");
    for s in ciphered_message 
    {
        print!("{}", s);
    }
    println!("\n")
}

fn deciphering()
{
    let mut shifting = String::new();
    let mut message = String::new();

    println!("Message to decipher : ");
    io::stdin().read_line(&mut message).expect("Failed on user input");

    println!("Shifting :");
    io::stdin().read_line(&mut shifting).expect("Failed on user input");
    let shifting:u8 = shifting.trim().parse().expect("A number was expected");

    println!("Ciphered message : {}", message);

    let deciphered_message: String = process_deciphering(shifting, &message);
    if deciphered_message.is_empty() == false
    {
        println!("{}", deciphered_message)
    }
}

fn process_deciphering(shifting:u8, message:&String) -> String
{
    let mut deciphered_message:String = String::new();

    if shifting > ALPHABET_MAX - ALPHABET_MIN 
    {
        println!("Number must be lower than {}", ALPHABET_MAX - ALPHABET_MIN);
    }
    else 
    {
        let mut deciphering_char:u8;
        
        for c in message.bytes() 
        {
            // Ignore if char is a trailer or an eof
            if ((c as char) != '\n') && ((c as char) != '\r')
            {
                if c < shifting
                {
                    deciphering_char = ALPHABET_MAX - (shifting - c);
                }
                else if (c - shifting) < ALPHABET_MIN
                {
                    let diff_with_min:u8 = c - ALPHABET_MIN;
                    deciphering_char = (ALPHABET_MAX + 1) - (shifting - diff_with_min);
                }
                else
                {
                    deciphering_char = c - shifting;
                }
                deciphered_message.push(deciphering_char as char);
            }
        }
    }
    return deciphered_message;
}

fn brute_forcing()
{
    let mut message = String::new();
    let mut shifting = 0;

    println!("Message to brute force : ");
    io::stdin().read_line(&mut message).expect("Failed on user input");

    while shifting < (ALPHABET_MAX - ALPHABET_MIN)
    {
        let deciphered_messages = process_deciphering(shifting, &message);
        if deciphered_messages.is_empty() == false
        {
            println!("{}", deciphered_messages);
        }
        shifting += 1;
    }

}
fn main() 
{
    menu();
}
