# Programa en Rust de un autómata finito no determinista

    
            +-------------------------------------+
            |    Ingrese una cadena a evaluar:    |
            +-------------------------------------+
            aaba
            +--------------+---------+----------------+
            | Edo. Actual | Caracter  | Edo. Siguiente|
            +--------------+---------+----------------+
            |     0       |      a    |     0         |
            +--------------+---------+----------------+
            |     0       |      a    |     0         |
            +--------------+---------+----------------+
            |     0       |      b    |     1         |
            +--------------+---------+----------------+
            |     1       |      a    |     1         |
            +--------------+---------+----------------+
            |     1       |       |Fin Cadena|        |
            +--------------+---------+----------------+
            |              Cadena Valida              |
            +-----------------------------------------+



# Objetivo

    El objetivo del ejercicio es simular un automata no finito determinista definido 
    por la siguiente expresión regular: a*ba*
        primer digito: a* = {"", a, aa, aaa, aaa}
        segundo digito: b= {b}
        tercer digito: a* = {"", a, aa, aaa, aaa}


# Problemas al programar

    El paso de parametros en la funciones en Python resulta muy sencillo, sin embargo en rust hay que especificar el tipo del dato ademas
    de indicar si a un valor se le esta haciendo referencia
    
        fn contenido(sig: i32, car: char, simbolo: &str, estado: i32){
             println!("|     {}       |      {}     |     {}  |      {}        |",sig,car,simbolo,estado);
            cuerpo();
        }

# Ejecucion de la aplicacion

    Para la compilacion

        $ cargo build

    Para la ejecución

        $ cargo run
