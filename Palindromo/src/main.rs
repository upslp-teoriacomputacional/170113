/*
*Nombre:     Yair Briones de la Rosa
*Matricula:  170113
*Carrera:    ITI
*Descripción: Programación Palindromo
*Escrito:       05/11/2020
*Ultima actualización:  06/11/2020
*% Rust
*/

use std::io;

/*Función Principal*/
fn main() {
    /*Palindromo*/
    println!("\t    +-------------------------------------+
            |    Ingrese una cadena a evaluar:    |
            +-------------------------------------+");
    /*cadena leida*/
    let mut cadena = String::new();         
    io::stdin().read_line(&mut cadena);
    /*Elimina el salto de linea de la cadena*/
    let palindrome=cadena.trim();    
    /*Se eliminan los espacios de la cadena*/
    let aux=palindrome.replace(" ","");
    /*Arreglo para almacenar la cadena y comparar*/
    let mut vector: Vec<String> = Vec::new();
    let mut compara: Vec<String> = Vec::new();
    /*Variables de control*/
    let mut i=0;
    let mut j=0;
    /*Variable para conocer la mitad del palindromo*/
    let mut tam=(aux.len())/2;
    /*Almacena la cadena en dos arreglos*/
    for x in aux.chars(){
        print!("{:?}",x);
        if i>=tam{
            compara.insert(j,x.to_string());
            j=j+1;
        }if i<=tam{
            vector.insert(i,x.to_string());
            i=i+1;
        }
    }
    println!();
    /*Revierte el orden del arreglo comparador*/ 
    compara.reverse();
    /*Validacion del palindromo*/
    if estado(&vector,&compara)==true{
        println!("El palindromo: {}, es valido",palindrome);
    }else{
        println!("El palindromo: {}, es invalido",palindrome);
    }
    
}

/*Funcion que evalua los dos arreglos y compara si son iguales*/
fn estado(vector: &[String], compara: &[String]) -> bool{
    /*Si los arreglos tienen el mismo valor, en la misma posicion*/    
    if vector==compara{
        /*Si son iguales*/
        return true;
    }else {
        /*Si no son iguales*/
        return false;
    }
    
}
