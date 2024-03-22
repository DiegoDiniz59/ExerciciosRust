/***************************************************************************************************
*                                                                                                  *
*                                        ARRAYS                                                    *
*                                                                                                  *
***************************************************************************************************/
fn main() {

/***************************************************************************************************
Exercício 1: Encontrar o Máximo
Enunciado: Escreva um programa em Rust que encontre o valor máximo em um array de inteiros.
***************************************************************************************************/
    
    let arr = [2, 8, 4, 6, 10, 1, 9, 3, 7, 5];
    let mut encontra_maximo;
    let mut i:usize = 0;
    
    encontra_maximo=arr[0];
    while i<arr.len()
    {
         if encontra_maximo < arr[i] {
             encontra_maximo = arr[i]
         }
         i = i + 1;
    }
    println!("O maior elemento é: {}", encontra_maximo);

/***************************************************************************************************
Exercício 2: Reverter um Array
Enunciado: Escreva um programa em Rust que reverta um array de inteiros sem usar métodos prontos da
linguagem para isso.
***************************************************************************************************/

pub fn reverter_array(array: &mut [u8]) {
    let mut start = 0;
    let mut end = array.len() - 1;

    while start < end {
        array.swap(start, end);
        start += 1;
        end -= 1;
    }
}
    let mut array = [2, 8, 4, 6, 10, 1, 9, 3, 7, 5];
    println!("Array original: {:?}", array);
    
    reverter_array(&mut array);
    
    println!("Array revertido: {:?}", array);

/***************************************************************************************************
Exercício 3: Calcular a Média
Enunciado: Escreva um programa em Rust que calcule a média dos valores armazenados em um
array de números de ponto flutuante.
***************************************************************************************************/

    let array = [2.5, 8.7, 4.3, 6.5, 10.1, 1.10, 9.6, 3.4, 7.8, 5.6];

    let mut all_sum_values: f64 = 0.0;
    let mut count = 0;

    for &num in &array {
        all_sum_values += num;
        count += 1;
    }

    let mean = all_sum_values / count as f64;

    println!("A média é: {}", mean);

/***************************************************************************************************
Exercício 4: Contar Elementos Negativos
Enunciado: Escreva um programa em Rust que conte quantos números negativos existem em um
array de inteiros.
***************************************************************************************************/

let arr = [2, 8, 4, 6, 10, -1, -9, -3, -7, -5];

    let mut negative_number = 0;

    for &num in arr.iter() {
        if num < 0 {
            negative_number += 1;
        }
    }

    println!("A quantidade de números negativos é: {}", negative_number);

/***************************************************************************************************
Exercício 5: Verificar Presença de Elemento
Enunciado: Escreva um programa em Rust que verifique se um determinado inteiro está
presente em um array.
***************************************************************************************************/

    let array = [2, 8, 4, 6, 10, 1, 9, 3, 7, 5];

    if array.contains(&5) {
        println!("Sim, o elemento específico está presente no Array");
    }
    else {
        println!("Não, o elemento especificado não está presente no Array");
    }

/***************************************************************************************************
*                                                                                                  *
*                                        VETORES                                                   *
*                                                                                                  *
***************************************************************************************************/

/***************************************************************************************************
Exercício 1: Adicionar Elementos a um Vetor
Enunciado: Escreva um programa em Rust que crie um vetor vazio e adicione a ele os
números de 1 a 5, um de cada vez, usando um loop.
***************************************************************************************************/

    let mut vetor: Vec<i32> = Vec::new();
    let mut count = 0;

    loop {
        count += 1;
        vetor.push(count);

        if count == 5 {
            println!("O vetor agora tem os seguintes números nele: {:?}", vetor);
            break;
        }
}

/***************************************************************************************************
Exercício 2: Remover Elemento Específico
Enunciado: Escreva um programa em Rust que remova o primeiro elemento de
valor 3 de um vetor de inteiros.
***************************************************************************************************/

    let mut vetor = vec![1, 3, 5, 3, 7];
    let mut snipe = false;

    for (indice, &valor) in vetor.iter().enumerate() {
        if valor == 3 {
            vetor.remove(indice);
            snipe = true;
            break

        }
    }

    if snipe {
    println!("O primeiro valor 3 foi removido com sucesso! {:?}", vetor);
    }
    else {
    println!("Não foi encontrado um valor 3 pra ser removido. {:?}", vetor);
    }

/***************************************************************************************************
Exercício 3: Calcular a Soma de Elementos
Enunciado: Escreva um programa em Rust que calcule a soma de todos os elementos
em um vetor de números inteiros.
***************************************************************************************************/

    let vetor = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let sum: i32 = vetor.iter().sum();

    println!("O valor total desse vetor é: {}", sum);

/***************************************************************************************************
Exercício 4: Encontrar o Menor Elemento
Enunciado: Escreva um programa em Rust que encontre o menor elemento em um vetor de números
inteiros.
***************************************************************************************************/

    let vetor = vec![2, 8, 4, 6, 10, 1, 9, 3, 7, 5];
    let min_value = vetor.iter().min();

    match min_value {
        Some(min) => println!("O valor mínimo é: {}", min),
        None => println!("O vetor está vazio")
    }

/***************************************************************************************************
Exercício 5: Filtrar Elementos Pares e Criar um Novo Vetor
Enunciado: Escreva um programa em Rust que, dado um vetor de números inteiros, crie
um novo vetor contendo apenas os elementos pares do vetor original.
***************************************************************************************************/

    let vetor = vec![2, 8, 4, 6, 10, 1, 9, 3, 7, 5];
    let mut vetor_par = Vec::new();

    for &numero in &vetor {
        if numero % 2 == 0 {
            vetor_par.push(numero);
        }
    }

    assert_eq!(vetor, vec![2, 8, 4, 6, 10, 1, 9, 3, 7, 5]);
    assert_eq!(vetor_par, vec![2, 8, 4, 6, 10]);

    println!("Aqui está o vetor novo com apenas os elementos pares do original: {:?}", vetor_par);

}