use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("¡Bienvenido, este es un juego de adivinanzas! ¿Estás listo para jugar?");
    println!(
        "¡Genial! Estoy pensando en un número entre 1 y 100. ¿Puedes adivinar cuál es? Introduce tu respuesta: "
    );

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error al leer la entrada");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Tu respuesta es: {}", guess);

        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("El número secreto era: {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muy pequeño!"),
            Ordering::Greater => println!("Muy grande!"),
            Ordering::Equal => {
                println!("¡Has ganado!");
                break;
            }
        }
    }
}
