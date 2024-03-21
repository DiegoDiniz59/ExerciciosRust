pub fn reverter_array(array: &mut [i32]) {
    let mut start = 0;
    let mut end = array.len() - 1;

    while start < end {
        // Troca os elementos nas posições start e end
        array.swap(start, end);
        // Move as posições de start e end para o próximo par de elementos
        start += 1;
        end -= 1;
    }
}

fn main() {
    let mut array = [2, 8, 4, 6, 10, 1, 9, 3, 7, 5];
    println!("Array original: {:?}", array);
    
    reverter_array(&mut array);
    
    println!("Array revertido: {:?}", array);
}
