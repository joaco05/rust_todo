use serde::{Deserialize, Serialize};
use std::io::{BufReader, Write};
use std::path::Path;
use std::{fs::File, io};

#[derive(Serialize, Deserialize, Debug)]
struct Tarea {
    descripcion: String,
    completada: bool,
}

impl Tarea {
    fn mostrar(&self, id: usize) {
        let estado = if self.completada { "[X]" } else { "[ ]" };
        println!("{} {}: {}", estado, id, self.descripcion);
    }
}

fn main() {
    println!("Bienvenido al gestor de tareas");
    let nombre_archivo = "hola.json";
    let mut tareas = cargar_tareas(nombre_archivo).unwrap();

    loop {
        println!(
            "\ningresa un comando('agregar <descripcion>', 'completar <id>', 'listar','salir')"
        );
        let mut entrada = String::new();
        io::stdin()
            .read_line(&mut entrada)
            .expect("Error al leer la entrada");
        let entrada = entrada.trim();

        match entrada {
            "salir" => {
                println!("\nSaliendo del gestor de tareas");
                guardar_tareas(tareas, nombre_archivo);
                break;
            }
            "listar" => {
                listar_tareas(&tareas);
            }
            _ if entrada.starts_with("agregar ") => {
                let descripcion = entrada[8..].trim();
                if !descripcion.is_empty() {
                    tareas.push(Tarea {
                        descripcion: descripcion.to_string(),
                        completada: false,
                    });
                    println!("\nTarea agregada: {descripcion}");
                } else {
                    println!("\nLa descripción de la tarea no puede estar vacía.");
                }
            }
            _ if entrada.starts_with("completar ") => {
                let id: usize = match entrada[10..].trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("\nID inválido. Debe ser un número.");
                        continue;
                    }
                };
                if id > 0 && id <= tareas.len() {
                    tareas[id - 1].completada = true;
                    println!("\nTarea {id} marcada como completada.");
                } else {
                    println!("\nID de tarea no válido.");
                }
            }

            _ => println!("\nComando no reconocido. Intenta de nuevo."),
        }
    }
}

fn listar_tareas(lista_de_tareas: &[Tarea]) {
    println!("\nLista de Tareas:");

    for (i, tarea) in lista_de_tareas.iter().enumerate() {
        tarea.mostrar(i + 1);
    }
}

fn cargar_tareas<P: AsRef<Path> + Copy>(
    direccion: P,
) -> Result<Vec<Tarea>, Box<dyn std::error::Error>> {
    let archivo = File::open(direccion);
    let mut tareas: Vec<Tarea> = vec![];
    match archivo {
        // Si el archivo no existe lo creo y si existe devuelvo las tareas
        Ok(archivo) => {
            let reader = BufReader::new(archivo);
            tareas = serde_json::from_reader(reader)?;
        }
        Err(_) => {
            File::create(direccion)?;
        }
    }
    Ok(tareas)
}

fn guardar_tareas<P: AsRef<Path>>(lista_tareas: Vec<Tarea>, direccion: P) {
    let mut archivo = File::create(direccion).unwrap();
    let serialized = serde_json::to_string_pretty(&lista_tareas).unwrap();
    archivo.write_all(serialized.as_bytes()).unwrap();
}

/*
    Desafio uno:
        Refactorizar el codigo con un match en vez de if, else if, else (DONE)

    Desafio dos:
        Guardar las tareas en un archivo serializado en formato JSON con el crate serde (DONE)

    Desafio tres:
        Cargar las tareas desde un archivo al iniciar el programa deserializado en formato JSON con el crate serde (DONE)

    Desafio cuatro:
        emitir reportes (TODO)

    Desafio cinco:
        prioridades (TODO)

    Desafio seis:
        etiquetas (TODO)

    Desafio siete:
        subtareas (TODO)
*/
