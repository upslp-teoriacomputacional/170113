# Programa en Rust de un autómata finito determinista

    
            +-------------------------------------+
            |    Ingrese una cadena a evaluar:    |
            +-------------------------------------+
            9+9
            +--------------+---------+-----------+---------------+
            | Edo. Actual | Caracter   | Simbolo  |Edo. Siguiente |
            +--------------+---------+-----------+---------------+
            |     0       |      9     |     Dig  |      1        |
            +--------------+---------+-----------+---------------+
            |     1       |      +     |     Op   |      2        |
            +--------------+---------+-----------+---------------+
            |     2       |      9     |     Dig  |      3        |
            +--------------+---------+-----------+---------------+
            |     3      |       |Fin Cadena|            |
            +--------------+---------+-----------+---------------+
            |              Cadena Valida                   |
            +----------------------------------------------------+


# Objetivo

    El objetivo del ejercicio es simular un automata finito determinista que acepta por valores de entrada
    cadenas que contenga 3 digitos:
        primer digito: [0-9]
        segundo digito: (+|-|*|/)
        tercer digito: [0-9]
    De esta forma el automata solo acepta cadenas que definan una operacion con digitos del 0 al 9
    La matriz de reglas es:
        Estado	Digitos	Operadores	Fin de cadena
        q0	    q1	    Error	    Error
        q1	    Error	q2	        Error
        q2	    qf	    Error	    Error
        qf	    error	Error	    Aceptacion

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
