use std::io;

const ALPHABET_MIN:u8 = b'A';
const ALPHABET_MAX:u8 = b'Z';
const MAX_SHIFTING:u8 = ALPHABET_MAX - ALPHABET_MIN;
const SELECT_CIPHER:u8 = 1;
const SELECT_DECIPHER:u8 = 2;
const SELECT_BRUTE_FORCE:u8 = 3;
const SELECT_EXIT:u8 = 4;

fn menu() -> bool
{
    let mut to_exit:bool = false;
    let mut select: String = String::new();

    println!("This program allows you to cipher/decipher words or sentence with latin alphabet (A-Z), 
    lowercase are converted to uppercase");

    println!("Menu :
    1 - Cipher
    2 - Decipher
    3 - Brute force (Display all possibility)
    4 - Exit");
    io::stdin().read_line(&mut select).expect("Failed on user input");
    match select.trim().parse()
    {
        Ok(value) => {
            match value {
                SELECT_CIPHER => ciphering(),
                SELECT_DECIPHER => deciphering(),
                SELECT_BRUTE_FORCE => brute_forcing(),
                SELECT_EXIT => { 
                    println!("Good bye !");
                    to_exit = true;
                },
                _ => println!("Please choose '1', '2', '3' or '4'"),
            }
        },
        Err(_) => println!("Input error : a number was expected")
    }
    
    return to_exit
}

fn ciphering()
{
    let mut shifting = String::new(); // user entry
    let mut message = String::new(); // user entry

    println!("Message to cipher : ");
    io::stdin().read_line(&mut message).expect("Failed on user input");
    let message = message.to_uppercase();

    println!("Shifting to apply : ");
    io::stdin().read_line(&mut shifting).expect("Failed on user input");

    match shifting.trim().parse()
    {
        Ok(value) => process_ciphering(value, &message),
        Err(_) => println!("Input error : a number was expected"),
    }
}

fn process_ciphering(shifting:u8, message:&String)
{
    let mut ciphering_char:u8; // character after ciphering

    let mut ciphered_message: Vec<char> = Vec::new();
    if shifting > MAX_SHIFTING
    {
        println!("Input error : number must be lower than {}", MAX_SHIFTING);
    }
    else    
    {
        println!("message : {}", message);

        for character in message.bytes() 
        {
            // Check if character is in alphabet range
            match character
            {
                ALPHABET_MIN..ALPHABET_MAX => {
                    if (character + shifting) > ALPHABET_MAX
                    {
                        ciphering_char = (ALPHABET_MIN + shifting) - ((ALPHABET_MAX + 1) - character);
                    }
                    else 
                    {
                        ciphering_char = character + shifting;
                    }
                    ciphered_message.push(ciphering_char as char);
                },
                b'\n' | b'\r' => (), 
                _ => println!("Character : {} is not ciphered because not in allowed alphabet", character as char),
            }
        }

        print!("Ciphered message: ");
        for s in ciphered_message 
        {
            print!("{}", s);
        }
        println!("\n")
    }
}

fn deciphering()
{
    let mut shifting = String::new();
    let mut message = String::new();

    println!("Message to decipher : ");
    io::stdin().read_line(&mut message).expect("Failed on user input");
    let message = message.to_uppercase();

    println!("Shifting :");
    io::stdin().read_line(&mut shifting).expect("Failed on user input");
    match shifting.trim().parse()
    {
        Ok(value) => {
            let deciphered_message:String = process_deciphering(value, &message);
            if deciphered_message.is_empty() == false
            {
                println!("Deciphered message : {}", deciphered_message)
            }
        },
        Err(_) => println!("Input error : a number was expected"),
    }
}

fn process_deciphering(shifting:u8, message:&String) -> String
{
    let mut deciphered_message:String = String::new();

    if shifting > MAX_SHIFTING
    {
        println!("Input error : number must be lower than {}", MAX_SHIFTING);
    }
    else 
    {
        let mut deciphering_char:u8;
        
        for character in message.bytes() 
        {
                        // Check if character is in alphabet range
            match character
            {
                ALPHABET_MIN..ALPHABET_MAX => {
                    if character < shifting
                    {
                        deciphering_char = ALPHABET_MAX - (shifting - character);
                    }
                    else if (character - shifting) < ALPHABET_MIN
                    {
                        let diff_with_min:u8 = character - ALPHABET_MIN;
                        deciphering_char = (ALPHABET_MAX + 1) - (shifting - diff_with_min);
                    }
                    else
                    {
                        deciphering_char = character - shifting;
                    }
                    deciphered_message.push(deciphering_char as char);
                },
                b'\n' | b'\r' => (), 
                _ => println!("Character : {} is not deciphered because not in allowed alphabet", character as char),
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

    while shifting < (MAX_SHIFTING)
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
    loop {
        let to_exit:bool = menu();
        if to_exit == true
        {
            break;
        }    
    }
}
