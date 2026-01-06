// Import des crates
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {   
    // Déclaration d'une variable constante (pas mutable)
    // Initialisation à une valeur aléatoire entre 0 et 100
    let nombre_aleatoire = rand::thread_rng().gen_range(0..=100);

    println!("*** Jeu du plus ou moins ! ***");

    loop 
    {
        println!("Entrer un nombre : ");

        // Déclaration et initialisation d'une variable modifiable
        // Initialisation en créant une nouvelle chaine de caractères vide
        let mut rep_joueur = String::new();

        // Récupération de la saisie utilisateur avec read_line
        // "expect" permet de décrire le message d'erreur si une erreur se produit (permet d'éviter un warning)
        io::stdin()
            .read_line(&mut rep_joueur)
            .expect("Echec de la lecture de l'entrée utilisateur");
        println!("Votre nombre : {}", &rep_joueur);

        // Nouvelle déclaration avec le meme nom que precedemment pour convertir le type str en int
        // trim() pour retirer les espaces et \n, parse() pour interpreter une chaine de caracteres en nombres
        let rep_joueur: u32 = match rep_joueur.trim().parse() {
            Ok(nombre) => nombre,
            Err(_) => continue,
        };
        // Comparaison du nombre secret avec le nombre de l'utilisateur
        match rep_joueur.cmp(&nombre_aleatoire)
        {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("C'est gagné !");
                break;
            }
        }
    }

}
