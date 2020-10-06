extern crate regex;     //dependencia 
//modulos
use std::process;
use regex::Regex;
use std::io;

//funcion principal
fn main() {
    #![allow(non_snake_case)]
    let mut estado=0;
    //5=E
    //8=A
    let tabla= vec![vec! [1,5,5],vec! [5,2,5],vec! [3,5,5],vec! [5,5,8]];   //matriz de transiciones
    
    println!("\t    +-------------------------------------+
            |    Ingrese una cadena a evaluar:    |
            +-------------------------------------+");
    let mut simbolo: String = "".to_string();
    let mut cadena = String::new();         //cadena leida
    io::stdin().read_line(&mut cadena);
    let cadena2=cadena.trim();
    cuerpo();
    encabezado();

    for car in cadena2.chars(){ //recorrido de la cadena caracter a caracter
        let mut sig=estado;
        let mut charcaracter=caracter(car); //llamamos al metodo para saber si es un caracter valido y el valor retornado se guarda en charcaracter
        estado=tabla[estado as usize][charcaracter as usize];   
        if charcaracter==0{ //valor del simbolo
      	    simbolo="Dig".to_string();
      	}
      	else if charcaracter==1{ //valor del simbolo
      	    simbolo="Op ".to_string();
      	}
        if estado==5{       //si la cadena no es valida
            println!("|     {}        |      {}      |       {}      |      {}      |",sig,car,simbolo,estado);
            cuerpo();
            println!("|              Cadena No Valida :(                   |
                +----------------------------------------------------+");
            process::exit(0x0100);  
        }
        contenido(sig,car,&simbolo,estado);
    }
    if estado!=3{    //si la cadena no es valida
        println!("|              Cadena No Valida :(                   |
            +----------------------------------------------------+");
    }
    if estado==3{   //si la cadena es valida
        println!("|     {}      |       |Fin Cadena|            |",estado);
        cuerpo();
        println!("|              Cadena Valida                   |
            +----------------------------------------------------+");
    }
}

fn caracter(car:char) -> i32{
    #![allow(non_snake_case)]
    let mut Fin = "";
    let digito = Regex::new(r"[0-9]").unwrap(); 
    let operador= Regex::new(r"(\+|\-|\*|/)").unwrap();

    let mut simbolo = String::from("");
	simbolo.push(car);
	let caracterAux: &str = &simbolo[..];

    if digito.is_match(caracterAux){    //comparamos si es digito o operador
        return 0;
    }else{
        if operador.is_match(caracterAux){
            return 1;
        }else{
            if caracterAux==Fin{
                return 2;
            }   
        }
        println!("Error, el caracter: {}, no es valido",car);   //si no es ni un digito ni un operador entonces es un caracter no valido
        process::exit(0x0100);     
    }              
}

fn cuerpo(){
    println!("+--------------+---------+-----------+---------------+")
}

fn encabezado(){    //definimos al la funcion  encabezado
    println!("| Edo. Actual | Caracter   | Simbolo  |Edo. Siguiente |");
    cuerpo();
}

fn contenido(sig: i32, car: char, simbolo: &str, estado: i32){
    println!("|     {}       |      {}     |     {}  |      {}        |",sig,car,simbolo,estado);
    cuerpo();
}


