use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut answer: [u32; 4] = [0, 0, 0, 0];
    for x in 0..4 {
        answer[x] = gen(answer);
    }
    let mut tries: i32 = 0;

    println!("-----------------------------------------------------------------------");
    println!("Guess a 4-digit number with the least number of attempts possible!
All four digits will be different.

With eeach guess, you will get a hint in the form of As and Bs.

A means: n digits match perfectly.
B means: n digits match but are not at the correct position.

For example, with an answer of 7130, a guess of 3610 will receive 1A2B.
1*A for the 0, rightly in the 4th position; 2*B are 1 and 3.");
    println!("-----------------------------------------------------------------------");
    // println!("The answer is {}{}{}{}.", answer[0], answer[1], answer[2], answer[3]);
    // println!("-----------------------------------------------------------------------");

    loop {
        println!("> ({}) Guess a 4-digit number:", tries);
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("> Failed to read line");

        guess = guess.trim().to_string();

        let _number: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️  Must be a number.");
                continue;
            },
        };

        let length: usize = guess.chars().count();
        let expected_length: usize = 4;

        match length.cmp(&expected_length) {
            Ordering::Less => {
                println!("⚠️  Less than 4 digits.");
                continue;
            },
            Ordering::Greater => {
                println!("⚠️  More than 4 digits.");
                continue;
            },
            Ordering::Equal => {
                tries = tries + 1;
                let result = check_answer(&answer, &guess);
                let mut noun = "tries";
                if tries == 1 {
                    noun = "try";
                }
                if result == "4A0B" {
                    println!("You got it in {} {}! 👾 The answer is {}!", tries, noun, guess);
                    break;
                } else {
                    println!("{}", result);
                }
            }
        }
    }
}

fn gen(answer: [u32; 4]) -> u32 {
    loop {
        let number: u32 = rand::thread_rng().gen_range(0, 10);
        if !answer.contains(&number) {
            break number;
        }
    }
}

fn check_answer(answer: &[u32; 4], guess: &String) -> String {
    let mut perfect_match: u32 = 0;
    let mut number_match: u32 = 0;
    let mut i: usize = 0;
    let mut handled = Vec::new();

    for char in guess.chars() {
        let num = char.to_digit(10).unwrap();
        let indexed_answer = answer[i];
        i = i + 1;
        if handled.contains(&num) && indexed_answer != num {
            break;
        }
        if indexed_answer == num {
            perfect_match = perfect_match + 1;
            if handled.contains(&num) {
                number_match = number_match - 1;
            }
            handled.push(num);
            continue;
        }
        if answer.contains(&num) {
            number_match = number_match + 1;
            handled.push(num);
        }
    }

    return format!("{}A{}B", perfect_match, number_match);
}
