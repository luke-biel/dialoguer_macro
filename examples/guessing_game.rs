use yaga::Dialogue;
use rand::Rng;

#[derive(Dialogue)]
struct Guess {
    #[dialogue(prompt = "Your pick")]
    answer: usize,
}

fn main() {
    let number = rand::thread_rng().gen_range(0, 100);

    loop {
        let Guess { answer } = Guess::compose("Guess a number!").unwrap();

        if number == answer {
            break;
        } else if number > answer {
            println!("Too small")
        } else {
            println!("Too big")
        }
    }

    println!("Congrats, you guessed correctly");
}
