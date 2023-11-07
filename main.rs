extern crate rand;

use std::io;
use rand::Rng;

fn gencode() -> String{
    let mut secode = String::new();
    let poscol = ['R', 'G', 'B', 'Y', 'O', 'P'];
    let mut rng = rand::thread_rng();
    for _ in 0..4 {
        let randcolor = poscol[rng.gen_range(0..6)];
        secode.push(randcolor);
    }
    secode
}

fn evaluate (secret: &str, guess: &str) -> (u32, u32) {
    let mut black = 0;
    let mut white = 0;
    let mut secret: Vec<char> = secret.chars().collect();
    let mut guess: Vec<char> = guess.chars().collect();

    for i in 0..4 {
        if secret[i] == guess[i] {
            black += 1;
            secret[i] = 'X';
            guess[i] = 'X';
        }
    }

    for i in 0..4 {
        if guess[i] != 'X' && secret.contains(&guess[i]) {
            white += 1;
            let index = secret.iter().position(|&c| c == guess[i]).unwrap();
            secret[index] = 'x';
            guess[i] = 'X';
        }
    }
    (black, white)
}

fn main() {
    let  secretcod = gencode();
    let mut attempts = 0;
    println!("Sup, and welcome to mstrmind!");
    loop {
        println!("Enter guess (RGBYOP):");
        let mut guesss = String::new();
        io::stdin().read_line(&mut guesss).expect("Wahhh something went wrong");
        guesss = guesss.trim().to_uppercase();

        if guesss.len() != 4 {
            println!("Invalid");
            continue;
        }
        let (black, white) = evaluate(&secretcod, &guesss);
        println!("Result: {} black, {} white", black, white);

        if black == 4 {
            println!("GGs, you won, in {}", attempts + 1);
            break;
        }
        attempts += 1;
    }
}
