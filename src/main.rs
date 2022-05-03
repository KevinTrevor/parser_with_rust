use std::io::{BufRead, BufReader, Write};
use std::fs::File;
use std::collections::HashMap;

/* 
    AQUÍ IRIA EL EBNF (SI LO TUVIERA EN DIGITAL)
*/
fn main() {
    leer_archivo();  
}

fn separar_string(oracion: &str) -> Vec<&str>{
    // Tomamos la cadena que nos pasan como argumento, y la dividimos con la función split()
    let vector: Vec<&str> = oracion.split(" ").collect();
    return vector;
}

fn es_numerico(cadena: &String) -> bool {
    /*
        Separamos una cadena en caracteres para saber si posee algun
        caracter numérico.
    */
    for caracter in cadena.chars(){
        if caracter.is_numeric() {
            return true;
        }
    }
    return false;
}

fn tiene_puntuacion(cadena: &String) -> bool {
    /*
        Separamos una cadena en caracteres para saber si posee algun
        símbolo de puntuación (sin incluir el espacio en blanco).
    */
    for caracter in cadena.chars(){
        if caracter.is_ascii_punctuation() && caracter != ' '{
            return true;
        }
    }
    return false;
}

fn leer_archivo(){
    // Aqui llamamos a las funciones para obtener los archivos de lectura y escritura
    let archivo: File = abrir_archivo_lectura();
    let mut nuevo: File = crear_archivo_escritura(); 

    // Instanciamos el lector del archivo con un BufReader
    let reader: BufReader<File> = BufReader::new(archivo);

    // Instanciamos el HashMap que contiene nuestros simbolos terminales
    let terminales = HashMap::from([
        ("nombre", vec!["rosa", "maria", "carlota", "lucia", "juan", "diego", "luis", "jesus"]),
        ("articulo", vec!["la", "las", "el", "los", "un", "una", "unos", "unas"]),
        ("sustantivo", vec!["fruta", "perro", "perra", "gato", "gata", "niño", "niña", "arbol", 
            "pelota", "perros", "perras", "gatos", "gatas", "pelotas", "niños", "niñas", "arboles"]),
        ("verbo", vec!["juega", "juegan", "come", "comen", "quiere", "quieren", "es", "son", 
            "corre", "corren", "llora", "lloran"]),
        ("preposicion", vec!["a", "con", "como", "por"]),
        ("adverbio", vec!["poco", "poca", "mucho", "mucha", "muy"]),
        ("adjetivo", vec!["rapido", "rapidos", "grande", "grandes", "verde", "verdes", "roja", 
            "rojas", "pequeño", "pequeños"])
    ]);

    for (numero, linea) in reader.lines().enumerate(){
        // La variable linea la usaremos para escribir en el nuevo archivo
        let linea: String = linea.unwrap(); 
        // La variable linea_lowercase la usaremos para separarla 
        let mut linea_lowercase = linea.to_lowercase();

        /* 
            Corroboramos que nuestra línea termine con 
        */
        if linea.ends_with('.') {
            // Primero quitamos el punto de nuestra linea_lowercase 
            linea_lowercase.pop();
            
            /* 
                Corroboramos que la cadena no sea numérica o no posea alguna puntuación demás;
                de ser así, la oración no pertenece al lenguaje.
            */
            if !es_numerico(&linea_lowercase) && !tiene_puntuacion(&linea_lowercase) {
                // Luego separamos la linea_lowercase y la usaremos para el analisis léxico
                let palabras = separar_string(&linea_lowercase);
                let tokens = analisis_lexico(&terminales, &palabras);

                /* 
                    Corroboramos que todas las palabras tengan un token asociado; sino
                    es así, quiere decir que la oración no pertenece al lenguaje.
                */
                if tokens.len() == palabras.len() {
                    // A partir de acá se puede proceder a realizar el analisis sintactico.
                    if analisis_sintactico(tokens) {
                        writeln!(&mut nuevo, "Oración {} Ok.", numero).unwrap();
                    } else {
                        writeln!(&mut nuevo, "Oración {} Error de sintaxis.", numero).unwrap();
                    }
                } else {
                    writeln!(&mut nuevo, "Oración {} Error de sintaxis.", numero).unwrap();
                }
            } else {
                writeln!(&mut nuevo, "Oración {} Error de sintaxis.", numero).unwrap();
            }
            
        } else {
            writeln!(&mut nuevo, "Oración {} Error de sintaxis.", numero).unwrap();
        }
    }
}

fn crear_archivo_escritura() -> File {
    // Crear un archivo en modo escritura
    // Esto crea el archivo si no existe y lo deja en blanco si ya existe
    let nuevo_archivo: File = File::create("src/análisis.txt").unwrap();
    return nuevo_archivo;
}

fn abrir_archivo_lectura() -> File {
    // Aqui abrimos el archivo, con el pathing src/oraciones.txt, en modo lectura
    let path: &str = "src/oraciones.txt";
    let archivo: File = File::open(path).unwrap();
    return archivo;
}

fn analisis_lexico<'a>(term: &'a HashMap<&'a str, Vec<&'a str>>, pal: &'a Vec<&'a str>) -> Vec<(&'a &'a str, &'a &'a str)> {
    /*
        Creamos un vector vacío, donde almacenaremos una tupla que almacenara
        (tipo, valor) de nuestros tokens pertenencientes a nuestros simbolos terminales
    */
    let mut tokens = Vec::new();

    for palabra in pal {
        /* 
            Para cada palabra en el vector de palabras se hará una comparación 
        */
        for (tipo, valor) in term.iter() {
            /* 
                La comparación se hace con cada uno de los simbolos terminales de
                cada categoría y, si esa categoría contiene la palabra, entonces
                se creará un token y se almacenará en nuestro vector tokens 
            */
            if valor.contains(&palabra){
                let token = (tipo, palabra);
                tokens.push(token);
            }
        }
    }
    return tokens;
}

fn generar_producciones<'a>() -> HashMap<&'a str, Vec<Vec<&'a str>>> {
    /* 
        Instanciamos las producciones en CNF basado en nuestras producciones EBNF
    */
    let producciones = HashMap::from([
        ("oracion", vec![vec!["sujeto", "predicado"]]),
        ("sujeto", vec![vec!["articulo", "sustantivo"], vec!["rosa"], vec!["maria"],  
            vec!["carlota"], vec!["lucia"], vec!["juan"], vec!["diego"], vec!["luis"], vec!["jesus"]]),
        ("predicado", vec![vec!["verbo", "sujeto"], vec!["verbo", "sujeto_adjetivo"], 
            vec!["verbo", "adjetivo"], vec!["verbo", "preposicion_sujeto"], 
            vec!["verbo", "adverbio_preposicion_sujeto"], vec!["verbo", "adverbio_sustantivo"],
            vec!["verbo", "adverbio_adjetivo"], vec!["juega"], vec!["juegan"], vec!["come"], 
            vec!["comen"], vec!["quiere"], vec!["quieren"], vec!["es"], vec!["son"], vec!["corre"],
            vec!["corren"], vec!["llora"], vec!["lloran"]]),
        ("verbo", vec![vec!["juega"], vec!["juegan"], vec!["come"], vec!["comen"], vec!["quiere"],
            vec!["quieren"], vec!["es"], vec!["son"], vec!["corre"], vec!["corren"], vec!["llora"], vec!["lloran"]]),
        ("sujeto_adjetivo", vec![vec!["sujeto", "adjetivo"]]),
        ("preposicion_sujeto", vec![vec!["preposicion", "sujeto"]]),
        ("adverbio_preposicion_sujeto", vec![vec!["adverbio", "preposicion_sujeto"]]),
        ("adverbio_sustantivo", vec![vec!["adverbio", "sustantivo"]]),
        ("adverbio_adjetivo", vec![vec!["adverbio", "adjetivo"]]),
        ("articulo", vec![vec!["la"], vec!["las"], vec!["el"], vec!["los"], vec!["un"], vec!["una"],
        vec!["unos"], vec!["unas"]]),
        ("sustantivo", vec![vec!["fruta"], vec!["perro"], vec!["perra"], vec!["gato"], vec!["gata"],
            vec!["pelota"], vec!["niño"], vec!["niña"], vec!["arbol"], vec!["frutas"], vec!["perros"],
            vec!["perras"], vec!["gatos"], vec!["gatas"], vec!["pelotas"], vec!["niños"], vec!["niñas"], vec!["arboles"]]),
        ("preposicion", vec![vec!["a"], vec!["con"], vec!["como"], vec!["por"]]), 
        ("adjetivo", vec![vec!["rapido"], vec!["rapidos"], vec!["grande"], vec!["grandes"], vec!["verde"],
            vec!["verdes"], vec!["roja"], vec!["rojas"], vec!["pequeño"], vec!["pequeños"]]),
        ("adverbio", vec![vec!["poco"], vec!["poca"], vec!["mucho"], vec!["mucha"], vec!["muy"]])
    ]);
    return producciones;
}

fn analisis_sintactico<'a>(tokens: Vec<(&'a &'a str, &'a &'a str)>) -> bool {
    /*
        Instanciamos nuestras producciones en CNF, el tamaño de nuestros tokens
        y la matriz solución que usaremos para el analisis sintactico
    */
    let prod = generar_producciones();
    let n = tokens.len();
    let mut solucion: Vec<Vec<String>> = vec![vec![String::from(""); n]; n]; 

    /* 
        Utilizamos el algoritmo CYK para realizar el analisis sintáctico;
        primero ingresamos los valores en la diagonal de nuestra matriz solucion
    */
    for i in 0..n {
        for (clave, valor) in prod.iter() {
            if valor.contains(&vec![&tokens[i].1]) {
                solucion[i][i].push_str(&clave);
            }
        }
    }

    /*
        Luego iteramos a través de la matriz, tomando en cuenta las producciones.
        Se van ingresando las producciones a la inversa, hasta llegar al axioma.
    */
    for l in 1..n {
        for r in 0..n-l {
            for t in 0..l {
                for (clave, valores) in prod.iter() {
                    for valor in valores {
                        if valor.len() == 2 {
                             if solucion[r][r + t].contains(&valor[0]) && 
                                solucion[r + t + 1][r + l].contains(&valor[1]) {
                                solucion[r][r + l].push_str(&clave);
                            }
                        }
                    }
                }
            }
        }
    }

    /* 
        Si la matriz tiene el axioma en la posición fila 0 y columna n-1, 
        quiere decir que la cadena pertenece al lenguaje. 

        Sino, entonces la cadena no pertenece al lenguaje.
    */
    if solucion[0][n-1] == "oracion" {
        return true;
    }
    return false;
}