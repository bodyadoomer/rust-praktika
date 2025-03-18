// src/homework/hw04.rs
pub fn draw_diamond() {
    const WIDTH: usize = 11;  // Максимальна ширина ромба
    const HEIGHT: usize = 11; // Висота ромба
    
    let mut diamond = String::new();
    
    for i in 0..HEIGHT {
        // Кількість зірочок у рядку
        let stars = if i <= HEIGHT / 2 {
            1 + 2 * i  
        } else {
            1 + 2 * (HEIGHT - 1 - i)  
        };
        
        // Додаємо пробіли перед зірочками
        for _ in 0..(WIDTH - stars) / 2 {
            diamond.push(' ');
        }
        
        // Додаємо зірочки
        for _ in 0..stars {
            diamond.push('*');
        }
        
        // Додаємо новий рядок
        diamond.push('\n');
    }
    
    print!("{}", diamond);
    println!(); // Завершуємо виведення
}

// Додаємо main для тестування в онлайн-компіляторі
fn main() {
    draw_diamond();
}
