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
}