/*
*Nombre:     Yair Briones de la Rosa
*Matricula:  170113
*Carrera:    ITI
*Descripción: Programación expresiones regulares
*Escrito:       22/10/2020
*Ultima actualización:  23/10/2020
*% Rust
*/

extern crate regex;     //dependencia 
use regex::Regex;       //MODULOS

fn main() {
    #![allow(non_snake_case)]
    let source_code="int acumulador = 9;".split_whitespace();     //sentencia de entrada
    let mut vector: Vec<String> = Vec::new();       //declaracion de arreglo de cadenas
    let mut aux: String = "".to_string();       

    //DEFINICION DE EXPRESIONES REGULARES
    let minus = Regex::new(r"[a-z]").unwrap(); 
    let mayus = Regex::new(r"[A-Z]").unwrap();
    let numbers = Regex::new(r"[0-9]").unwrap();
    let operador= Regex::new(r"(\+|\-|\*|/|\&|=)").unwrap();
    
    //RECORRE CADA PALABRA DE LA SENTENCIA DE ENTRADA
    for word in source_code {
        if word=="int" || word=="str" || word=="bool" {     //TIPO DE DATO
            aux = ("DATATYPE: ".to_owned()+word);
            vector.insert(0,aux);           //INSERTA EN EL VECTOR
        }else if minus.is_match(word) || mayus.is_match(word){  
            aux = ("IDENTIFIER: ".to_owned()+word);         //IDENTIFICADOR
            vector.insert(1,aux);           //INSERTA EN EL VECTOR
        }else if operador.is_match(word) {                  //OPERADOR
            aux = ("OPERAND: ".to_owned()+word);
            vector.insert(2,aux);           //INSERTA EN EL VECTOR
        }else if numbers.is_match(word){                    //VALOR Y FIN DE SENTENCIA
            let mut x=word.len();
            if word[x-1..x].to_string()==";"{
                aux = word[0..x-1].to_string();     //ASIGNA EL VALOR NUMERICO
                let cad="INTEGER: ";         //CADENA AUXILIAR
                aux=cad.to_owned() + &aux;                //CONCATENA LAS CADENAS
                vector.insert(3,aux);       //INSERTA EN EL VECTOR
                aux = "END_STATEMENT: ;".to_string();
                vector.insert(4,aux);       //INSERTA EN EL VECTOR
            }else{
                aux = ("INTEGER: ".to_owned()+word);
                vector.insert(3,aux);
            }
        }
    } 
    for x in vector {           //IMPRIME EL VECTOR
        print!("[{:?}], ", x);
    }
}

fn variablePROLOG(w : String) -> i32{
    let mut w: String = "".to_string(); 
    let mut aux: Vec<char> = Vec::new();    
    for cars in w.chars()   //CONVIERTE LA CADENA EN UN ARREGLO DE CARACTERES
    {
        aux.push(cars);
    }
    let aux2: Vec<_> =aux[0].to_uppercase().collect();  //SE DECLARA UN VECTOR QUE PERMITE RECIBIR LOS ELEMENTOS DEL ITERADOR TO_UPPER
    if aux[0].is_alphabetic() && aux[0]==(aux2[0] as char) ||  aux[0]=='_' {
        //w = w[1:] #Se quita el primer caracter
        aux.reverse();
        aux.pop();
        aux.reverse();
        // while w and (w[0].isalnum() or w[0]=='_'):
        //Mientras queden caracteres en "w" y el primer caracter actual sea un alfanumerico o un subrayado, todo esta bien
        while !aux.is_empty() && aux[0].is_alphanumeric() || aux[0]=='_'{
            //w = w[1:] #Quitar el primer caracter
            aux.reverse();
            aux.pop();
            aux.reverse();
        }
        //if w=='':
        if aux.is_empty(){
            return 0;
        }
    }
    return 1;  
}