use rand::Rng;

fn main() {
    let magic_number: u8 = rand::thread_rng().gen_range(1..=100);

    println!("Welcome to the magical Guessing Game!!");
    println!("Choose your difficulty:");
    println!("1 for easy");
    println!("2 for normal");
    println!("3 for hard");
    println!("4 for hardcore!");

    let mut difficulty: String = String::new();
    let mut chances: u8;

    std::io::stdin()
                .read_line(&mut difficulty)
                .expect("Failed to read line");

    let difficulty: u8 = match difficulty.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

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
