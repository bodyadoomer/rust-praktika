// hw03.rs
pub fn draw_envelope() {
    const WIDTH: usize = 30;  // Ширина конверта
    const HEIGHT: usize = 15; // Висота конверта
    
    let mut envelope = String::new();
    
    // Верхня межа
    for _ in 0..WIDTH {
        envelope.push('*');
    }
    envelope.push('\n');
    
    // Середня частина
    for i in 0..HEIGHT - 2 {
        // Ліва межа
        envelope.push('*');
        
        // Внутрішня частина з діагоналями
        for j in 0..WIDTH - 2 {
            // Перша діагональ: від верхнього лівого до нижнього правого
            // Друга діагональ: від верхнього правого до нижнього лівого
            if j == i * (WIDTH - 2) / (HEIGHT - 2) ||              // ліва-верх -> права-низ
               j == (WIDTH - 2) - (i * (WIDTH - 2) / (HEIGHT - 2)) // права-верх -> ліва-низ
            {
                envelope.push('*');
            } else {
                envelope.push(' ');
            }
        }
        
        // Права межа
        envelope.push('*');
        envelope.push('\n');
    }
    
    // Нижня межа
    for _ in 0..WIDTH {
        envelope.push('*');
    }
    
    print!("{}", envelope);
    println!(); // Додаємо новий рядок в кінці
}

// Для тестування
fn main() {
    draw_envelope();
}
