# Programación expresiones regulares
    El objetivo del programa es evaluar una sentencia de codigo fuente y separla a fin de conocer todas sus partes utilizando expresiones regulares 
# Source Code
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
    Cuando se ejecuta este fragmento de código, la salida debe ser la siguiente:
    ['DATATYPE', 'int'], ['IDENTIFIER', 'acumulador'], ['OPERATOR', '='], ['INTEGER', '9'], ['END_STATEMENT', ';']

# Problemas al programar
    ->Pyhton es un lenguaje que facilita el uso de la listas, sin embargo en Rust se debe recurrir al uso de arreglos o de HashSet; para resolver este problema se utilizo un vector de cadenas
     let mut vector: Vec<String> = Vec::new();
    ->El manejo de variables de tipo string se dificultaba por que existian cadenas de tipo &str
     Utilizar el metodo .to_string() 
  
# Ejecucion de la aplicacion

    Para la compilacion

        $ cargo build

    Para la ejecución

        $ cargo run

