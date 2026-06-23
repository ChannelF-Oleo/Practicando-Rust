use std::io;

fn main() {
    println!("¡Bienvenido!");
    println!("Esta es una calculadora");
    println!("Estas son las opciones");
    println!("1- Suma");
    println!("2- Resta");
    println!("3- Multiplicacion");
    println!("4- Division");
    println!("Escribe el numero de la opcion deseada");

    let operacion: i32 = loop {
        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Ocurrio un error inesperado");

        match entrada.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Ocurrio un error inesperado, por favor selecciona nuevamente");
                continue;
            }
        };
    };
    println!("Ha escogido la opcion {}", operacion);

    println!("Ingrese el primer numero");
    let a: i32 = loop {
        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Ocurrio un error inesperado");

        match entrada.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Ocurrio un error inesperado, por favor ingrese nuevamente");
                continue;
            }
        };
    };
    println!("Ingrese el segundo numero");
    let b: i32 = loop {
        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Ocurrio un error inesperado");

        match entrada.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Ocurrio un error inesperado, por favor ingrese nuevamente");
                continue;
            }
        };
    };

    fn suma(a: i32, b: i32) -> i32 {
        a + b
    }
    fn resta(a: i32, b: i32) -> i32 {
        a - b
    }
    fn multiplicacion(a: i32, b: i32) -> i32 {
        a * b
    }
    fn division(a: i32, b: i32) -> i32 {
        a / b
    }

    let resultado = match operacion {
        1 => suma(a, b),
        2 => resta(a, b),
        3 => multiplicacion(a, b),
        4 => division(a, b),
        _ => {
            println!("Opcion invalida");
            return;
        }
    };
    println!("El resultado es: {}", resultado);
}
