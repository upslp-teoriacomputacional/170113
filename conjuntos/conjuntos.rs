/*
*Nombre:     Yair Briones de la Rosa
*Matricula:  170113
*Carrera:    ITI
 
*Descripción: Programa que inicializa 3 conjuntos, y 
            lleva a cabo las operaciones basicas de estos
            (añadir, borrar, union, intersección, etc.)
 
*Escrito:       10/09/2020
*Ultima actualización:  14/09/2020
 
*% Rust
*/

use std::collections::HashSet;

fn main() {
    //# define three sets
    #![allow(non_snake_case)]
    let  A: HashSet<i32>=vec![1,2,3,4,5].into_iter().collect();   //Conjunto A
    let  B: HashSet<i32>=vec![3,4,5,6,7].into_iter().collect();   //Conjunto B
    let  C: HashSet<i32>=vec![].into_iter().collect();               //Conjunto C 

    //impresion de los 3 conjuntos
    println!("A: {:?}", A);
    println!("B: {:?}", B);
    println!("C: {:?}\n\n", C);

    pertenencia();
    transformarConj();
    quitar();
    clearSet();
    copiar();
    agregar();
    union();
    interseccion();
    diferencia();
    simetrica();
    subconjunto();
    superconjunto();
}

fn pertenencia(){
    #![allow(non_snake_case)]
    println!("Pertenencia");
    let set: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();
    println!("1 is in A: {}",set.contains(&1));
    println!("1 is not in A: {}",!set.contains(&1));
    println!("10 is in A: {}",set.contains(&10));
    println!("10 is not in A: {}",!set.contains(&10));
}

//Convertir a un conjunto
fn transformarConj(){
    #![allow(non_snake_case)]
    println!("Convertir a un conjunto");
    let A = [1,2,3,4,5];    //arreglo
    let set: HashSet<i32> = A.iter().cloned().collect();
    println!("A: {:?}", set);
    let  B: HashSet<i32>=vec![1,2,3,4,5].into_iter().collect();   //vector
    println!("B: {:?}", B);
    let cadena="Hola Mundo";    //string
    let mut C: HashSet<char> = HashSet::new();
    for car in cadena.chars(){
        C.insert(car);
    } 
    println!("C: {:?}", C);

}

//Remove an item from the set
fn quitar(){
    #![allow(non_snake_case)]
    println!("Quitar elemento");
    let mut A: HashSet<i32>=vec![1,2,3,4,5].into_iter().collect();
    assert!(A.remove(&2));
    println!("The new set A: {:?}", A);
}

//Remove all items from the set
fn clearSet(){
    #![allow(non_snake_case)]
    println!("Vaciar Conjunto");
    let mut a = HashSet::new();
    a.insert(1);
    a.insert(2);
    a.insert(3);
    a.insert(4);
    a.insert(5);
    a.clear();
    println!("The new set A: {:?}", a);
}

fn copiar(){
    #![allow(non_snake_case)]
    println!("Copiar");
    let A: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();
    let B: HashSet<_> = A.clone();
    println!("A: {:?}", A);
    println!("B: {:?}", B);

}

//Add an item
fn agregar(){
    #![allow(non_snake_case)]
    println!("Agregar");
    let mut B: HashSet<i32>=vec![3,4,5,6,7].into_iter().collect(); 
    assert!(B.insert(987));
    println!("The new set B: {:?}", B);
}

/*
"""
Set Operations
"""
*/

//Union
fn union(){
    #![allow(non_snake_case)]
    let  A: HashSet<i32>=vec![1,2,3,4,5].into_iter().collect();
    let  B: HashSet<i32>=vec![3,4,5,6,7].into_iter().collect();
    println!("Union: {:?}", A.union(&B).collect::<Vec<&i32>>());
}

//Intersection
fn interseccion(){
    #![allow(non_snake_case)]
    let  A: HashSet<i32>=vec![1,2,3,4,5].into_iter().collect();
    let  B: HashSet<i32>=vec![3,4,5,6,7].into_iter().collect(); 
    println!("Intersection: {:?}", A.intersection(&B).collect::<Vec<&i32>>());
}

//Diference
fn diferencia(){
    #![allow(non_snake_case)]
    let  A: HashSet<i32>=vec![1,2,3,4,5].into_iter().collect();
    let  B: HashSet<i32>=vec![3,4,5,6,7].into_iter().collect();
    println!("Difference: {:?}", A.difference(&B).collect::<Vec<&i32>>());
}

//Symmetric difference
fn simetrica(){
    #![allow(non_snake_case)]
    let  A: HashSet<i32>=vec![1,2,3,4,5].into_iter().collect();
    let  B: HashSet<i32>=vec![3,4,5,6,7].into_iter().collect();
    let  C: HashSet<i32>=vec![].into_iter().collect();
    println!("Symmetric Difference: {:?}",A.symmetric_difference(&B).collect::<Vec<&i32>>());
    println!("Symmetric Difference: {:?}",B.symmetric_difference(&A).collect::<Vec<&i32>>());
    println!("Symmetric Difference: {:?}",A.symmetric_difference(&C).collect::<Vec<&i32>>());
    println!("Symmetric Difference: {:?}",B.symmetric_difference(&C).collect::<Vec<&i32>>());    
}

//Subset
fn subconjunto(){
    #![allow(non_snake_case)]
    let  A: HashSet<i32>=vec![1,2,3,4,5].into_iter().collect();
    let  B: HashSet<i32>=vec![3,4,5,6,7].into_iter().collect();
    println!("Subset: {:?}",A.is_subset(&B));
    println!("Subset: {:?}",B.is_subset(&A));
}

//Superset
fn superconjunto(){
    #![allow(non_snake_case)]
    let  A: HashSet<i32>=vec![1,2,3,4,5].into_iter().collect();
    let  B: HashSet<i32>=vec![3,4,5,6,7].into_iter().collect();
    println!("Subset: {:?}",A.is_superset(&B));
    println!("Subset: {:?}",B.is_superset(&A));
}