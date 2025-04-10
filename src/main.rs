use std::fs;
use std::io;

mod entradas;

fn main() -> io::Result<()> {
    let texto: String = fs::read_to_string("texto.txt")?;

    // Imprime el contenido tal como se vería en la consola:
    println!("Contenido de texto.txt:");
    println!("{:?}", texto);

    let texto_limpio: String = texto.replace("\n", " ").replace("\r", " ");

    // Imprime el contenido limpio:)
    println!("Contenido limpio:");
    println!("{:}", texto_limpio);

    // Tokeniza el contenido modificado según las reglas definidas
    let tokens: Vec<String> = entradas::tokenize(&texto_limpio);

    // Imprime el vector resultante de tokens {:?} pa ver los /n etc
    println!("vector tokenizado:");
    println!("{:?}", tokens);

    Ok(())
}
