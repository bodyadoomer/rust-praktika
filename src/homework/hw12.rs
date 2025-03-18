fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;
    
    if total % n != 0 {
        return None; // Неможливо розподілити рівномірно
    }
    
    let avg = total / n;
    let mut moves = 0;
    let mut balance = 0;
    
    for &ship in shipments {
        balance += (ship as i32 - avg as i32);
        moves += balance.abs() as usize;
    }
    
    Some(moves)
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let avg = 10; // Наприклад, усі кораблі будуть мати 10 вантажу
    vec![avg; n]
}

fn main() {
    let example1 = vec![8, 2, 2, 4, 4];
    let example2 = vec![9, 3, 7, 2, 9];
    
    match count_permutation(&example1) {
        Some(moves) => println!("Переносів: {}", moves),
        None => println!("Неможливо вирівняти вантажі")
    }
    
    match count_permutation(&example2) {
        Some(moves) => println!("Переносів: {}", moves),
        None => println!("Неможливо вирівняти вантажі")
    }
    
    let generated = gen_shipments(5);
    println!("Згенеровані вантажі: {:?}", generated);
}
