use std::io;
use std::time::{SystemTime, UNIX_EPOCH}; // Importamos SystemTime y UNIX_EPOCH para obtener el tiempo actual en microsegundos

#[derive(Debug)]
enum Clase {
    Guerrero,
    Mago,
    Arquero,
}

#[derive(Debug)]
enum Enfoque {
    Fisico,
    Magico,
}

#[derive(Debug)]
enum Tecnica {
    Golpe,
    Hechizo,
    Disparo,
}

enum Estado {
    Vivo,
    Muerto,
}

#[derive(Clone)]
enum Ataque {
    GolpeColosal,
    FiloDelTitan,
    CorteDevastador,
    LlamaArcana,
    RayoAncestral,
    OrbeDelCaos,
    FlechaDeLaMuerte,
    DisparoPreciso,
    OjosDeHalcon,
}

impl Ataque {
    fn nombre(&self) -> &str {
        match self {
            Ataque::GolpeColosal => "Golpe Colosal",
            Ataque::FiloDelTitan => "Filo del Titan",
            Ataque::CorteDevastador => "Corte Devastador",
            Ataque::LlamaArcana => "Llama Arcana",
            Ataque::RayoAncestral => "Rayo Ancestral",
            Ataque::OrbeDelCaos => "Orbe del Caos",
            Ataque::FlechaDeLaMuerte => "Flecha de la Muerte",
            Ataque::DisparoPreciso => "Disparo Preciso",
            Ataque::OjosDeHalcon => "Ojos de Halcón",
        }
    }

    fn descripcion(&self) -> &str {
        match self {
            Ataque::GolpeColosal => "Un golpe devastador que aplasta a tu enemigo.",
            Ataque::FiloDelTitan => "Un corte poderoso que atraviesa la armadura del enemigo.",
            Ataque::CorteDevastador => "Un corte rápido y letal que deja al enemigo sin defensa.",
            Ataque::LlamaArcana => "Una llamarada mágica que quema a tus enemigos.",
            Ataque::RayoAncestral => "Un rayo de energía ancestral que impacta con fuerza.",
            Ataque::OrbeDelCaos => "Un orbe de caos que desata destrucción a su paso.",
            Ataque::FlechaDeLaMuerte => "Una flecha mortal que encuentra su objetivo sin fallo.",
            Ataque::DisparoPreciso => "Un disparo exacto que no deja margen de error.",
            Ataque::OjosDeHalcon => "Un ataque con precisión sobrehumana, imposible de esquivar.",
        }
    }

    fn daño(&self) -> u32 {
        match self {
            Ataque::GolpeColosal => 50,
            Ataque::FiloDelTitan => 45,
            Ataque::CorteDevastador => 40,
            Ataque::LlamaArcana => 35,
            Ataque::RayoAncestral => 30,
            Ataque::OrbeDelCaos => 25,
            Ataque::FlechaDeLaMuerte => 20,
            Ataque::DisparoPreciso => 15,
            Ataque::OjosDeHalcon => 10,
        }
    }

    fn tipo(&self) -> &str {
        match self {
            Ataque::GolpeColosal | Ataque::FiloDelTitan | Ataque::CorteDevastador => "Fisico",
            Ataque::LlamaArcana | Ataque::RayoAncestral | Ataque::OrbeDelCaos => "Magico",
            Ataque::FlechaDeLaMuerte | Ataque::DisparoPreciso | Ataque::OjosDeHalcon => "Fisico",
        }
    }

    fn clase(&self) -> &str {
        match self {
            Ataque::GolpeColosal | Ataque::FiloDelTitan | Ataque::CorteDevastador => "Guerrero",
            Ataque::LlamaArcana | Ataque::RayoAncestral | Ataque::OrbeDelCaos => "Mago",
            Ataque::FlechaDeLaMuerte | Ataque::DisparoPreciso | Ataque::OjosDeHalcon => "Arquero",
        }
    }

    fn tecnica (&self) -> &str {
        match self {
            Ataque::GolpeColosal | Ataque::FiloDelTitan | Ataque::CorteDevastador => "Golpe",
            Ataque::LlamaArcana | Ataque::RayoAncestral | Ataque::OrbeDelCaos => "Hechizo",
            Ataque::FlechaDeLaMuerte | Ataque::DisparoPreciso | Ataque::OjosDeHalcon => "Disparo",
        }
    }


}

struct Heroe {
    nombre: String,
    clase: Clase,
    nivel: u32,
    salud: u32,
    ataque: u32,
}

impl Heroe {
    fn new(nombre: &str, clase: Clase) -> Self {
        let (salud, ataque) = match clase {
            Clase::Guerrero => (150, 20),
            Clase::Mago => (100, 30),
            Clase::Arquero => (120, 25),
        };
        Heroe {
            nombre: nombre.to_string(),
            clase,
            nivel: 1,
            salud,
            ataque,
        }
    }

    fn mostrar_info(&self) {
        println!("Nombre: {}", self.nombre);
        println!("Clase: {:?}", self.clase);
        println!("Enfoque: {:?}", self.enfoque());
        println!("Técnica: {:?}", self.tecnica());
        println!("Nivel: {}", self.nivel);
        println!("Salud: {}", self.salud);
        println!("Ataque: {}", self.ataque);
    }

    fn estado(&self) -> Estado {
        if self.salud > 0 {
            Estado::Vivo
        } else {
            Estado::Muerto
        }
    }

    fn esta_vivo(&self) -> bool {
        matches!(self.estado(), Estado::Vivo)
    }

    fn enfoque(&self) -> Enfoque {
        match self.clase {
            Clase::Guerrero => Enfoque::Fisico,
            Clase::Mago => Enfoque::Magico,
            Clase::Arquero => Enfoque::Fisico,
        }
    }

    fn tecnica(&self) -> Tecnica {
        match self.clase {
            Clase::Guerrero => Tecnica::Golpe,
            Clase::Mago => Tecnica::Hechizo,
            Clase::Arquero => Tecnica::Disparo,
        }
    }



    fn recibir_dano(&mut self, dano: u32) {
        self.salud = self.salud.saturating_sub(dano);
    }

    fn atacar(&self, enemigo: &mut Heroe) {
        println!(
            "{} ataca a {} causando {} de daño!",
            self.nombre, enemigo.nombre, self.ataque
        );
        enemigo.recibir_dano(self.ataque);
    }

    fn subir_nivel(&mut self) {
        self.nivel += 1;
        self.salud += 20;
        self.ataque += 5;
        println!("{} ha subido al nivel {}!", self.nombre, self.nivel);
    }

    fn ataques_disponibles(&self) -> Vec<Ataque> {
        match self.clase {
            Clase::Guerrero => vec![
                Ataque::GolpeColosal,
                Ataque::FiloDelTitan,
                Ataque::CorteDevastador,
            ],
            Clase::Mago => vec![
                Ataque::LlamaArcana,
                Ataque::RayoAncestral,
                Ataque::OrbeDelCaos,
            ],
            Clase::Arquero => vec![
                Ataque::FlechaDeLaMuerte,
                Ataque::DisparoPreciso,
                Ataque::OjosDeHalcon,
            ],
        }
    }
}

fn crear_heroe() -> Heroe {
    let mut nombre = String::new();
    println!("Ingresa el nombre del héroe: ");
    io::stdin()
        .read_line(&mut nombre)
        .expect("Error al leer la entrada");
    let nombre = nombre.trim();

    println!("Selecciona la clase del héroe:");
    println!("1. Guerrero");
    println!("2. Mago");
    println!("3. Arquero");

    let mut clase_input = String::new();
    io::stdin()
        .read_line(&mut clase_input)
        .expect("Error al leer la entrada");
    let clase_input: u32 = match clase_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor ingresa un número válido.");
            return Heroe::new("", Clase::Guerrero); // Retorna un héroe vacío en caso de error
        }
    };

    let clase = match clase_input {
        1 => Clase::Guerrero,
        2 => Clase::Mago,
        3 => Clase::Arquero,
        _ => {
            println!("Clase no válida. Se asignará Guerrero por defecto.");
            Clase::Guerrero
        }
    };

    Heroe::new(nombre, clase)
}

fn elegir_heroe(heroes: &Vec<Heroe>) -> Option<usize> {
    println!("Selecciona un héroe de la lista:");
    for (index, heroe) in heroes.iter().enumerate() {
        println!("{}. {}", index + 1, heroe.nombre);
    }

    let mut eleccion = String::new();
    io::stdin()
        .read_line(&mut eleccion)
        .expect("Error al leer la entrada");
    let eleccion: usize = match eleccion.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor ingresa un número válido.");
            return None;
        }
    };

    if eleccion == 0 || eleccion > heroes.len() {
        println!("Selección no válida.");
        return None;
    }

    Some(eleccion - 1)
}

fn mostrar_ataques(heroe: &Heroe) {
    let ataques = heroe.ataques_disponibles();
    println!("Ataques disponibles para {}:", heroe.nombre);
    for (index, ataque) in ataques.iter().enumerate() {
        println!(
            "{}. {} - {} (Daño: {}, Tipo: {}, Clase: {})",
            index + 1,
            ataque.nombre(),
            ataque.descripcion(),
            ataque.daño(),
            ataque.tipo(),
            ataque.clase()
        );
    }
}

fn elegir_ataque(heroe: &Heroe) -> Option<Ataque> {
    let ataques = heroe.ataques_disponibles();
    println!("Selecciona un ataque:");
    for (index, ataque) in ataques.iter().enumerate() {
        println!(
            "{}. {} - {} (Daño: {}, Tipo: {}, Clase: {})",
            index + 1,
            ataque.nombre(),
            ataque.descripcion(),
            ataque.daño(),
            ataque.tipo(),
            ataque.clase()
        );
    }

    let mut eleccion = String::new();
    io::stdin()
        .read_line(&mut eleccion)
        .expect("Error al leer la entrada");
    let eleccion: usize = match eleccion.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Por favor ingresa un número válido.");
            return None;
        }
    };

    if eleccion == 0 || eleccion > ataques.len() {
        println!("Selección no válida.");
        return None;
    }

    Some(ataques[eleccion - 1].clone())
}

fn ataque_aleatorio(heroe: &Heroe) -> Ataque {
    let ataques = heroe.ataques_disponibles();
    let micros = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros();
    let idx = (micros % ataques.len() as u128) as usize;
    ataques[idx].clone()
}

fn combate(heroe: &mut Heroe, enemigo: &mut Heroe) {
    println!("---------------ARENA DE COMBATE--------------");
    println!(
        "¡Comienza el combate entre {} y {}!",
        heroe.nombre, enemigo.nombre
    );
    let mut turno = 0;
    while heroe.esta_vivo() && enemigo.esta_vivo() {
        turno += 1;
        println!("\n--- Turno {} ---", turno);
        println!("{}: Salud = {}", heroe.nombre, heroe.salud);
        println!("{}: Salud = {}", enemigo.nombre, enemigo.salud);

        mostrar_ataques(heroe);
        let ataque_heroe = elegir_ataque(heroe);
        if let Some(ataque) = ataque_heroe {
            let danio = calcular_dano(&ataque, heroe, enemigo);
            println!(
                "{} usa {} ({}, {}) causando {} de daño!",
                heroe.nombre,
                ataque.nombre(),
                ataque.tipo(),
                ataque.tecnica(),
                danio
            );
            enemigo.recibir_dano(danio);
        } else {
            println!("{} no realizó ningún ataque, usa ataque básico.", heroe.nombre);
            heroe.atacar(enemigo);
        }
        if !enemigo.esta_vivo() {
            println!("{} ha sido derrotado!", enemigo.nombre);
            heroe.subir_nivel();
            break;
        }

        let ataque_enemigo = ataque_aleatorio(enemigo);
        let danio_enemigo = calcular_dano(&ataque_enemigo, enemigo, heroe);
        println!(
            "{} usa {} ({}, {}) causando {} de daño!",
            enemigo.nombre,
            ataque_enemigo.nombre(),
            ataque_enemigo.tipo(),
            ataque_enemigo.tecnica(),
            danio_enemigo
        );
        heroe.recibir_dano(danio_enemigo);
        if !heroe.esta_vivo() {
            println!("{} ha sido derrotado!", heroe.nombre);
            break;
        }
    }
}

fn calcular_dano(ataque: &Ataque, atacante: &Heroe, _defensor: &Heroe) -> u32 {
    let base_dano = ataque.daño();
    let modificador = match (ataque.tipo(), atacante.enfoque()) {
        ("Fisico", Enfoque::Fisico) => 1.2,
        ("Magico", Enfoque::Magico) => 1.2,
        _ => 1.0,
    };
    (base_dano as f32 * modificador) as u32
}

fn main() {
    let mut heroes: Vec<Heroe> = vec![
        Heroe::new("Arnold", Clase::Guerrero),
        Heroe::new("Gandalf", Clase::Mago),
        Heroe::new("Ron", Clase::Arquero),
    ];
loop {
    
    println!("Bienvenido al juego de héroes!");
    println!("¿Que deseas hacer?");
    println!("1. Iniciar una nueva partida");
    println!("2. Ver lista de héroes");
    println!("3. Crear un nuevo héroe");
    println!("4. Salir");

    println!("Ingresa tu elección: ");


        let mut eleccion = String::new();
        io::stdin()
            .read_line(&mut eleccion)
            .expect("Error al leer la entrada");
        let eleccion: u32 = match eleccion.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Por favor ingresa un número válido.");
                continue;
            }
        };

        match eleccion {
            1 => {
                println!("Iniciando una nueva partida...");
                match elegir_heroe(&heroes) {
                    None => println!("Selección inválida, volviendo al menú."),
                    Some(heroe_idx) => {
                        // Elegir enemigo aleatorio diferente al héroe
                        let enemigo_idx = {
                            let micros = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_micros();
                            let mut idx = (micros % heroes.len() as u128) as usize;
                            // Asegurarse que sea diferente
                            while idx == heroe_idx {
                                idx = (idx + 1) % heroes.len();
                            }
                            idx
                        };
                        println!("Has elegido: {}", heroes[heroe_idx].nombre);
                        println!("¡Tu enemigo es: {}!", heroes[enemigo_idx].nombre);
                        println!(
                            "Iniciando combate entre {} y {}",
                            heroes[heroe_idx].nombre, heroes[enemigo_idx].nombre
                        );
                        let (i, j) = if heroe_idx < enemigo_idx {
                            (heroe_idx, enemigo_idx)
                        } else {
                            (enemigo_idx, heroe_idx)
                        };
                        let (izq, der) = heroes.split_at_mut(j);
                        if heroe_idx < enemigo_idx {
                            combate(&mut izq[i], &mut der[0]);
                        } else {
                            combate(&mut der[0], &mut izq[i]);
                        }
                         
                    }
                }
               
            }
            2 => {
                println!("Mostrando lista de héroes...");
                for hero in &heroes {
                    hero.mostrar_info();
                }
            }
            3 => {
                println!("Creando un nuevo héroe...");

                let nuevo_heroe = crear_heroe();
                if nuevo_heroe.nombre != "" {
                    println!("Héroe creado exitosamente!");
                    nuevo_heroe.mostrar_info();
                    heroes.push(nuevo_heroe);
                }
            }
            4 => {
                println!("Saliendo del juego. ¡Hasta luego!");
                break;
            }
            _ => {
                println!("Opción no válida. Por favor ingresa un número entre 1 y 4.");
            }
        }
    }
} 


