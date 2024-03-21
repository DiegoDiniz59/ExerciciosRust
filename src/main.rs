fn main() {

/***************************************************************************************************
Exercício 1: Encontrar o Máximo
Enunciado: Escreva um programa em Rust que encontre o valor máximo em um array de inteiros.
***************************************************************************************************/
    
    let arr: [u8; 10] = [2, 8, 4, 6, 10, 1, 9, 3, 7, 5];
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
}