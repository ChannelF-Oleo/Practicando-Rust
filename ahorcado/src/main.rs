use std::io;
use std::time::{SystemTime, UNIX_EPOCH}; // Importamos SystemTime y UNIX_EPOCH para obtener el tiempo actual en microsegundos   

// Definimos un enum para representar las dificultades del juego
enum Dificultad {
    Facil,      // 8 vidas
    Intermedio, // 6 vidas
    Dificil,    // 4 vidas
}

// Definimos tres bancos de palabras según la dificultad, usamos const para que sean inmutables y de tamaño fijo
const PALABRAS_CORTAS: [&str; 5] = ["UI", "UX", "code", "data", "web"];
const PALABRAS_MEDIAS: [&str; 5] = ["codigo", "bucle", "objeto", "funcion", "consola"];
const PALABRAS_LARGAS: [&str; 5] = [
    "programacion",
    "desarrollo",
    "computadora",
    "algoritmo",
    "interfaz",
];

fn main() {
    println!("¡Bienvenido!");
    println!("Este es un juego de ahorcado");
    println!("Selecciona la dificultad:");
    println!("1- Facil");
    println!("2- Intermedio");
    println!("3- Dificil");

    // Solicitamos al usuario que ingrese la dificultad deseada
    let dificultad = loop {
        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Ocurrio un error inesperado");

        match entrada.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Ocurrio un error inesperado, por favor selecciona nuevamente");
                continue;  // Continuamos el bucle para solicitar la entrada nuevamente
            }
        };
    };
    println!("Ha escogido la dificultad {}", dificultad);

    // Convertimos la entrada del usuario en el enum Dificultad correspondiente
    let dificultad = match dificultad {
        1 => Dificultad::Facil, // el numero representa la dificultad seleccionada por el usuario y se asigna al enum correspondiente
        2 => Dificultad::Intermedio,
        3 => Dificultad::Dificil,
        _ => {
            println!("Opcion invalida, se seleccionara dificultad facil por defecto");
            Dificultad::Facil // Si el usuario ingresa un número fuera del rango, se asigna la dificultad facil por defecto
        }
    };

    // Asignamos el número de vidas según la dificultad seleccionada
    let vidas = match dificultad {
        Dificultad::Facil => 8,  // Aqui el enum Dificultad es como un tipo de dato que puede tener diferentes variantes, y cada variante representa una dificultad diferente. En este caso, la variante Dificultad::Facil representa la dificultad facil y se le asigna el número de vidas correspondiente (8). De manera similar, las otras variantes del enum representan las otras dificultades y se les asignan sus respectivos números de vidas.
        Dificultad::Intermedio => 6,
        Dificultad::Dificil => 4,
    };

    // Seleccionamos el banco de palabras según la dificultad
    let banco_palabras = match dificultad {
        Dificultad::Facil => &PALABRAS_CORTAS
            .iter() // Iteramos sobre el array 
            .map(|s| s.to_string()) // Convertimos cada elemento en un String
            .collect::<Vec<String>>(), // Convertimos el iterador en un Vec<String> para poder indexarlo al azar
        Dificultad::Intermedio => &PALABRAS_MEDIAS
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
        Dificultad::Dificil => &PALABRAS_LARGAS
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(), // Convertimos los arrays de &str a Vec<String> para poder indexarlos al azar
    };


    // Bucle principal del juego, se repite hasta que el jugador decida salir
    'partidas: loop {
        println!("¡Iniciando nueva partida!");

        // Seleccionamos una palabra al azar del banco de palabras usando el tiempo actual en microsegundos como método para generar un índice aleatorio
        let microseconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Error al obtener el tiempo")
            .as_micros();

        // Calculamos el índice de la palabra seleccionada usando el módulo del tamaño del banco de palabras
        let index = (microseconds % banco_palabras.len() as u128) as usize;

        // Obtenemos la palabra seleccionada del banco de palabras usando el índice calculado
        let palabra_seleccionada = &banco_palabras[index];

        // Convertimos la palabra seleccionada en un vector de caracteres (letras)
        let palabra_letras: Vec<char> = palabra_seleccionada.chars().collect();

        // Creamos el progreso inicial lleno de guiones bajos '_'
        let mut progreso: Vec<char> = vec!['_'; palabra_letras.len()];

        // Necesitamos una variable mutable para ir restando las vidas
        let mut vidas_restantes = vidas;

        println!("Seleccionando banco de palabras...");
        println!("Estamos listos para comenzar el juego");

        println!("El juego ha comenzado con {} vidas", vidas);
        println!("La palabra tiene {} letras", palabra_letras.len());
        println!("¡Buena suerte!");

        loop {
            // 1. Mostrar el estado actual al jugador
            print!("\nPalabra: ");
            for letra in &progreso {
                print!("{} ", letra);
            }
            println!("\nVidas restantes: {}", vidas_restantes);

            // 2. Condición de Victoria / Derrota
            if !progreso.contains(&'_') {
                println!(
                    "¡Felicidades! Ganaste. La palabra era: {}",
                    palabra_seleccionada
                );
                break;
            }
            if vidas_restantes == 0 {
                println!(
                    "¡Te has quedado sin vidas! Perdiste. La palabra era: {}",
                    palabra_seleccionada
                );
                break;
            }

            // 3. Pedir una letra al usuario
            println!("Introduce una letra:");
            let mut intento = String::new();
            io::stdin()
                .read_line(&mut intento)
                .expect("Error al leer la línea");

            // Obtenemos el primer carácter ingresado
            let letra_usuario = match intento.trim().chars().next() {
                Some(c) => c.to_ascii_lowercase(), // Lo pasamos a minúscula para evitar problemas
                None => {
                    println!("Por favor, introduce una letra válida.");
                    continue;
                }
            };

            // 4. Verificar si la letra está en la palabra
            let mut acierto = false;
            for i in 0..palabra_letras.len() {
                // Pasamos la letra de la palabra a minúscula para comparar de forma justa
                if palabra_letras[i].to_ascii_lowercase() == letra_usuario {
                    progreso[i] = palabra_letras[i]; // Revelamos la letra en su posición real
                    acierto = true;
                }
            }

            // 5. Penalizar si falló
            if !acierto {
                println!(
                    "¡Incorrecto! La letra '{}' no está en la palabra.",
                    letra_usuario
                );
                vidas_restantes -= 1;
            } else {
                println!("¡Buen intento! Encontraste una letra.");
            }
        }

        println!("¿Deseas jugar otra partida? (s/n)");
        let mut respuesta = String::new();
        io::stdin()
            .read_line(&mut respuesta)
            .expect("Error al leer la línea");
        if respuesta.trim().to_lowercase() != "s" {
            println!("Gracias por jugar. ¡Hasta la próxima!");
            break 'partidas;
        };
    }
}
