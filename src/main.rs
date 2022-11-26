use rand::Rng;

fn main() {
    let magic_number: u8 = rand::thread_rng().gen_range(1..=100);

    println!("Welcome to the magical Guessing Game!!");
    println!("Choose your difficulty:");
    println!("1 for easy");
    println!("2 for normal");
    println!("3 for hard");
    println!("4 for hardcore!");

    let difficulty: u8;
    let mut chances: u8;

    loop {
        let mut difficulty_option: String = String::new();

        std::io::stdin()
                    .read_line(&mut difficulty_option)
                    .expect("Failed to read line");

        let difficulty_as_int: u8 = match difficulty_option.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        if difficulty_as_int > 4 {
            println!("Please choose only between difficulty 1, 2, 3, or 4!");
        } else {
            difficulty = difficulty_as_int;
            break;
        }
    }

    if difficulty == 1 {
        chances = 13;
    } else if difficulty == 2 {
        chances = 10;
    } else if difficulty == 3 {
        chances = 7;
    } else {
        chances = 3;
    }

    while chances > 0 {

        println!("Please type in your guess between 1 to 100: ");

        let mut guess: String = String::new();

        std::io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        if (guess >= 1) && (guess <= 100) {
            println!("Your guess {guess}");
        } else {
            println!("Remember, the number must be between 0 and 100!!");
            continue;
        }

        match guess.cmp(&magic_number) {
            std::cmp::Ordering::Less => println!("Too small! {chances} chances left!"),
            std::cmp::Ordering::Greater => println!("Too big! {chances} chances left!"),
            std::cmp::Ordering::Equal => {
                println!("You W. I. N.!!");
                println!("The magic number is {magic_number}");
                return;
            }
        }

        chances -= 1;

        if chances == 0 {
            println!("The magic number is {magic_number}");
            println!("Oops.. No more chances, you L. O. S. T. !");
            return;
        }
    }
}
