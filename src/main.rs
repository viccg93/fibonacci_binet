use std::io;
const PHI: f64 = 1.618033988749895;
fn main() {
    println!("========");
    println!("Programa que calcula la serie de fibonacci usando la formula de Binet");
    println!("Jacques Binet fue un matematico y astronomo frances, nacio en 1786 y murio en 1856.");
    println!("su formula utiliza el numero aureo cuyo valor es {PHI}");
    println!("========");
    println!("=== Ingresa el numero de elementos a generar de la serie fibonacci ===");
    let num_elementos =  obtener_numero_usr_input();
    if num_elementos > 0 {
        println!("El numero de elementos de la serie a generar es {num_elementos}");
        imprimir_serie_fibonacci(num_elementos);
    } else {
        println!("Introduce un valor valido, numerico mayor a 0.");
    }

}

//devuelve un numero de la entrada del usr, en caso de Err resuelve 0.
fn obtener_numero_usr_input() -> u32{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer entrada de texto");
    
    //Recordemos que el valor retornado por read_line es una cadena y necesitamos un u32
    //para realizar la convercion usaremos parse()
    //parse() devuelve un enum y el valor wrapped, por lo que tenemos que sacar el valor
    //En esta ocasion lo haremos con match sobre el enum.
    let input: u32 = match input.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("El valor introducido no es un numero, Introduce un numero!");
            0
        },
    };
    input
}

//usa formula de Binet con PHI
fn imprimir_serie_fibonacci_PHI(max_iteracion: u32){
    //la formula de Binet requiere n=0 como inicial.
    //el rango no toma el limite superior
    for iteracion in 0..max_iteracion {
        // se requiere que n o la iteracion sea f64 por concordancia con powf
        let iteracion = iteracion as f64;
        let result = (PHI.powf(iteracion) - (1.0 - PHI).powf(iteracion)) / f64::sqrt(5.0);
        println!("[{}]", result as u64);
    }
}

//usa formula de Binet simplificada
fn imprimir_serie_fibonacci(max_iteracion: u32){
    //la formula de Binet requiere n=0 como inicial.
    //el rango no toma el limite superior
    let raiz_de_5 = f64:: sqrt(5.0);
    for iteracion in 0..max_iteracion {
        // se requiere que n o la iteracion sea f64 por concordancia con powf
        let iteracion = iteracion as f64;
        let result = (f64::powf(1.0+raiz_de_5,iteracion) - f64::powf(1.0-raiz_de_5,iteracion)) / (f64::powf(2.0,iteracion) * raiz_de_5);
        println!("[{}]", result as u64);
    }
}

//posterior implementacion con Kahan-Babuska en las operaciones con f64.