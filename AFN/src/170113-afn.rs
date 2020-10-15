/*
*Nombre:     Yair Briones de la Rosa
*Matricula:  170113
*Carrera:    ITI
*Descripción: Programa que realiza un AFD
*Escrito:       14/10/2020
*Ultima actualización:  14/10/2020
*% Rust
*/
extern crate regex;     //dependencia 
//modulos
use std::process;
use regex::Regex;
use std::io;

//funcion principal
fn main() {
    #![allow(non_snake_case)]
    let mut estado=0;
    //5=Error
    //0 Estado inicial q0
    //1 Estado final q1
    let tabla= vec![vec! [0,1,5],vec! [1,5,5]];   //matriz de transiciones
    
    println!("\t    +-------------------------------------+
            |    Ingrese una cadena a evaluar:    |
            +-------------------------------------+");
    let mut cadena = String::new();         //cadena leida
    io::stdin().read_line(&mut cadena);
    let cadena2=cadena.trim();
    cuerpo();
    encabezado();

    for car in cadena2.chars(){ //recorrido de la cadena caracter a caracter
        let sig=estado;
        let charcaracter=caracter(car); //llamamos al metodo para saber si es un caracter valido y el valor retornado se guarda en charcaracter
        estado=tabla[estado as usize][charcaracter as usize];   


        if estado==5{       //si la cadena no es valida
            println!("|     {}       |      {}    |      {}        |",sig,car,estado);
            cuerpo();
            println!("|              Cadena NO Valida           |");
            println!("+-----------------------------------------+");
            process::exit(0x0100);  
        }
        contenido(sig,car,estado);
    }
    if estado!=1{    //si la cadena no es valida
        println!("|              Cadena NO Valida           |");
        cuerpo();
        println!("+-----------------------------------------+"); 
    }
    if estado==1{   //si la cadena es valida
        println!("|     {}       |       |Fin Cadena|        |",estado);
        cuerpo();
        println!("|              Cadena Valida              |");
        println!("+-----------------------------------------+");     
    }
}

fn caracter(car:char) -> i32{
    #![allow(non_snake_case)]
    let letronix_a = Regex::new(r"a").unwrap(); //Expresion regular para a*
    let letronix_b = Regex::new(r"b").unwrap(); //Expresion regular para b
    let mut simbolo = String::from("");
	simbolo.push(car);
	let caracterAux: &str = &simbolo[..];

    if letronix_a.is_match(caracterAux){    //comparamos si es digito o operador
        return 0;
    }else{
        if letronix_b.is_match(caracterAux){
            return 1;
        }
        println!("Error, el caracter: {}, no es valido",car);   //si no es ni un digito ni un operador entonces es un caracter no valido
        process::exit(0x0100);     
    }              
}

fn cuerpo(){
    println!("+--------------+---------+----------------+")
}

fn encabezado(){    //definimos al la funcion  encabezado
    println!("| Edo. Actual | Caracter  | Edo. Siguiente|");
    cuerpo();
}

fn contenido(sig: i32, car: char, estado: i32){
    println!("|     {}       |      {}    |     {}         |",sig,car,estado);
    cuerpo();
}


