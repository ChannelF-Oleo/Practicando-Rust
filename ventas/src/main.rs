use std::io; // Importamos el módulo de entrada/salida para leer datos del usuario

// Definimos una estructura para representar un producto con su nombre y precio
struct Producto {
    nombre: String,
    precio: f64,
}

// Implementamos métodos para la estructura Producto
impl Producto {
    // Método constructor para crear un nuevo producto
    fn nuevo(nombre: String, precio: f64) -> Self {
        Producto { nombre, precio }
    }

    // Método para aplicar un descuento al precio del producto
    fn aplicar_descuento(&mut self, porcentaje: f64) {
        self.precio -= self.precio * (porcentaje / 100.0);
    }
}

// Función principal del programa
fn main() {
    // Creamos un vector para almacenar los productos
    let mut inventario: Vec<Producto> = Vec::new();

    // Agregamos algunos productos de ejemplo al inventario
    inventario.push(Producto::nuevo("Laptop".to_string(), 1200.00));
    inventario.push(Producto::nuevo("Smartphone".to_string(), 499.99));
    inventario.push(Producto::nuevo("Tablet".to_string(), 299.99));
    inventario.push(Producto::nuevo("Auriculares".to_string(), 89.99));
    inventario.push(Producto::nuevo("Monitor".to_string(), 199.99));
    inventario.push(Producto::nuevo("Teclado".to_string(), 49.99));

    loop {
        println!("¡Bienvenido a su sistema de facturacion!");
        println!("¿Que desea hacer?");
        println!("1. Ver inventario");
        println!("2. Agregar producto");
        println!("3. Aplicar descuento a un producto");
        println!("4. Salir");
        println!("Ingrese su opción: ");

        let mut opcion = String::new(); // Creamos una variable mutable para almacenar la opción del usuario    
        io::stdin()
            .read_line(&mut opcion)
            .expect("Error al leer la entrada"); // Leemos la entrada del usuario y la almacenamos en la variable opcion
        let opcion: u32 = match opcion.trim().parse() {
            // Intentamos convertir la entrada a un número entero
            Ok(num) => num, // Si la conversión es exitosa, almacenamos el número en la variable opcion
            Err(_) => continue, // Si la conversión falla, continuamos con la siguiente iteración del bucle
        };

        match opcion {
            // Evaluamos la opción ingresada por el usuario
            1 => {
                // Si la opción es 1, mostramos el inventario
                println!("Inventario:");
                if inventario.is_empty() {
                    // Verificamos si el inventario está vacío
                    println!("El inventario está vacío."); // Si está vacío, mostramos un mensaje
                } else {
                    // Si no está vacío, mostramos los productos
                    println!("Productos disponibles:");
                }
                // Iteramos sobre los productos en el inventario y los mostramos en pantalla
                for producto in &inventario {
                    println!("{} - ${:.2}", producto.nombre, producto.precio); //
                }
            }
            2 => {
                // Si la opción es 2, agregamos un nuevo producto al inventario
                let mut nombre = String::new();
                let mut precio = String::new();
                println!("Ingrese el nombre del producto: ");
                io::stdin()
                    .read_line(&mut nombre)
                    .expect("Error al leer la entrada"); // Leemos el nombre del producto ingresado por el usuario y lo almacenamos en la variable nombre
                println!("Ingrese el precio del producto: ");
                io::stdin()
                    .read_line(&mut precio)
                    .expect("Error al leer la entrada"); // Leemos el precio del producto ingresado por el usuario y lo almacenamos en la variable precio
                // Intentamos convertir el precio ingresado a un número flotante
                let precio: f64 = match precio.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                inventario.push(Producto::nuevo(nombre.trim().to_string(), precio)); // Creamos un nuevo producto con el nombre y precio ingresados y lo agregamos al inventario
            }
            3 => {
                // Si la opción es 3, aplicamos un descuento a un producto existente
                let mut nombre = String::new();
                let mut porcentaje = String::new();
                println!("Ingrese el nombre del producto al que desea aplicar el descuento: ");
                io::stdin()
                    .read_line(&mut nombre)
                    .expect("Error al leer la entrada");
                println!("Ingrese el porcentaje de descuento: ");
                io::stdin()
                    .read_line(&mut porcentaje)
                    .expect("Error al leer la entrada");
                let porcentaje: f64 = match porcentaje.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                // Iteramos sobre los productos en el inventario y aplicamos el descuento al producto que coincida con el nombre ingresado por el usuario
                for producto in &mut inventario {
                    if producto.nombre == nombre.trim() {
                        producto.aplicar_descuento(porcentaje);
                        println!(
                            "Descuento aplicado. Nuevo precio de {}: ${:.2}",
                            producto.nombre, producto.precio
                        );
                    }
                }
            }
            4 => break, // Si la opción es 4, salimos del bucle y terminamos el programa
            _ => println!("Opción no válida. Por favor intente de nuevo."), // Si la opción no es válida, mostramos un mensaje de error
        }
    }
}
