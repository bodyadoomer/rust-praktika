use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> Option<(usize, i32, i32)> {
    if data.len() < 2 {
        return None;
    }
    let (mut min_idx, mut min_sum) = (0, data[0] + data[1]);
    for i in 1..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_idx = i;
            min_sum = sum;
        }
    }
    Some((min_idx, data[min_idx], data[min_idx + 1]))
}

fn print_result(data: &[i32], min_info: Option<(usize, i32, i32)>) {
    let indexes: Vec<String> = (0..data.len()).map(|i| format!("{:>2}.", i)).collect();
    let data_str: Vec<String> = data.iter().map(|v| format!("{:>2}", v)).collect();
    
    println!("indexes: {}", indexes.join("  "));
    println!("data:    [{}]", data_str.join(", "));
    
    if let Some((idx, v1, v2)) = min_info {
        let mut indicator = vec![" ".to_string(); data.len() * 4 - 1];
        indicator[idx * 4 + 2] = '\\'.to_string();
        indicator[idx * 4 + 3] = "__".to_string();
        indicator[idx * 4 + 4] = "__".to_string();
        indicator[idx * 4 + 5] = "/".to_string();
        
        println!("indexes: {}", indicator.join(""));
        println!("min adjacent sum={}+{}={} at indexes:{},{}", v1, v2, v1 + v2, idx, idx + 1);
    }
}

fn main() {
    let data = gen_random_vector(20);
    let min_info = min_adjacent_sum(&data);
    print_result(&data, min_info);
}
