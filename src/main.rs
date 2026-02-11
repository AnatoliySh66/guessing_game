use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Угадай число!");
	let secret_number = rand::thread_rng().gen_range(1..=100);
	
	loop {
		println!("Пожалуйста, выскажите свое предположение.");

		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Не удалось прочитать строку");
			
		let guess: u32 = guess.trim().parse() {
			
		};

		println!("Ваше число: {guess}");
		
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Ваше число меньше!"),
			Ordering::Greater => println!("Ваше число больше!"),
			Ordering::Equal => {
				println!("Ура, Вы угадали!");
				break;
			}
		}
	}
	println!("Было загадано: {secret_number}");
}