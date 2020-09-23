/*
*Nombre:     Yair Briones de la Rosa
*Matricula:  170113
*Carrera:    ITI
 
*Descripción: Programa que inicializa una lista de posibles 
              valores booleanos a fin de llevar a cabo las operaciones
              and, or, not y xor
 
*Escrito:       21/09/2020
*Ultima actualización:  21/09/2020
 
*% Rust
*/

fn main() {     //funcion principal

    let booleanos = vec![true,false];   //Lista de posibles valores booleanos

    //Tabla de verdad de OR
    println!("x\ty\tx or y");   //Encabezados de tabla
    
    for _x in 0..22{     //impresión de caracteres para la separación
        print!("-");
    }
    println!();
    for x in booleanos.iter(){      //Itera con cada valor booleano (true, false) e imprime su valor actual, ademas realiza la operacion or
        for y in booleanos.iter(){
            println!("{},{},{}\t",x,y,x|y); //itera nuevamente sobre cada valor, a fin de alcanzar todas las posibles combinaciones
        }
    }
    println!();

    //Tabla de verdad de AND
    println!("x\ty\tx and y");  //Encabezados de tabla

    for _x in 0..22{             //impresión de caracteres para la separación
        print!("-");
    }
    println!();

    for x in booleanos.iter(){  //Itera con cada valor booleano (true, false) e imprime su valor actual, ademas realiza la operacion and
        for y in booleanos.iter(){
            println!("{},{},{}\t",x,y,x&y); //itera nuevamente sobre cada valor, a fin de alcanzar todas las posibles combinaciones
        }
    }
    println!();

    //Tabla de verdad de NOT
    println!("x\tnot x"); //Encabezados de tabla
    for _x in 0..13{
        print!("-");    //impresión de caracteres para la separación
    }
    println!();
    for x in booleanos.iter(){ //Itera con cada valor booleano (true, false) e imprime su valor actual, ademas realiza la negacion de este
        println!("{},{}\t",x,!x);
    }
    println!();
    

    //Tabla de verdad de ^

    println!("x\ty\tx ^ y");   //Encabezados de tabla
    
    for _x in 0..22{     //impresión de caracteres para la separación
        print!("-");
    }
    println!();
    for x in booleanos.iter(){      //Itera con cada valor booleano (true, false) e imprime su valor actual, ademas realiza la operacion xor
        for y in booleanos.iter(){
            println!("{},{},{}\t",x,y,x^y); //itera nuevamente sobre cada valor, a fin de alcanzar todas las posibles combinaciones
        }
    }

}