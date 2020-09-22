Programa en Rust para ilustrar las diferentes operaciones de conjuntos

    Los tres operadores lógicos básicos son O, Y y NO, representados en Rust por |, & y !, respectivamente.

Objetivo

    El objetivo del ejercicio es lograr las 4 combinaciones posibles en x,y para los valores de la lista [true,false].
    De esta forma se obtienen las siguientes combinaciones.

        x       y
        true    true
        true    false
        false   true
        false   false

    Ademas, para cada tabla se lleva a cabo una operacion logica especifica (or, and, not, xor).

Source Code

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

Problemas al programar 

    Para ejecutar una sentencia for en Python es muy sencillo, solo basta con poner el elemento a recorrer y el rango. 
    Sin embargo esta tarea en rust se le agrega un paso, pues al trabajar con arreglos se debe agregar el metodo iter()
    a fin de que el siguiente elemento del arreglo se recorra.
    
        for x in booleanos.iter(){ 
            println!("{},{}\t",x,!x);
        }

Ejecucion de la aplicacion

    Para la compilacion

        $ rustc main.rs

    Para la ejecución

        $ ./main # o main.exe en Windows
