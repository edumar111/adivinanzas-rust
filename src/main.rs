extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivina el numero!");
    
    let  numero_secreto = rand::thread_rng().gen_range(1,21);

   // println!("el numero secreto es:{}", numero_secreto);

    loop{
    	println!("Por favor introduce tu corazonada del 1-20.");

	    let mut corazonada = String::new();

	    io::stdin().read_line(&mut corazonada)
	        .ok()
	        .expect("Fallo al leer linea");
	    
	    let corazonada: u32 = corazonada.trim().parse()
	    .ok()
	    .expect("Por favor introduce un numero!");

	    println!("Tu corazonada fue: {}", corazonada);

	    match corazonada.cmp(&numero_secreto) {
	    	Ordering::Less    => println!("Muy pequeño!"),
	    	Ordering::Greater => println!("Muy grande!"),
	    	Ordering::Equal   => {
	    		println!("Haz ganado!");
	    		break;
	    	}
	    }
    }
    
}
