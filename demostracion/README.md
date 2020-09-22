#Programa en Rust para ilustrar las diferentes operaciones de conjuntos

    Los tres operadores lógicos básicos son O, Y y NO, representados en Rust por |, & y !, respectivamente.

#Objetivo

    El objetivo del ejercicio es lograr las 4 combinaciones posibles en x,y para los valores de la lista [true,false].
    De esta forma se obtienen las siguientes combinaciones.

        x       y
        true    true
        true    false
        false   true
        false   false

    Ademas, para cada tabla se lleva a cabo una operacion logica especifica (or, and, not, xor).

#Problemas al programar 

    Para ejecutar una sentencia for en Python es muy sencillo, solo basta con poner el elemento a recorrer y el rango. 
    Sin embargo esta tarea en rust se le agrega un paso, pues al trabajar con arreglos se debe agregar el metodo iter()
    a fin de que el siguiente elemento del arreglo se recorra.
    
        for x in booleanos.iter(){ 
            println!("{},{}\t",x,!x);
        }

#Ejecucion de la aplicacion

    Para la compilacion

        $ rustc main.rs

    Para la ejecución

        $ ./main # o main.exe en Windows
