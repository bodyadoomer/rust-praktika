fn generate_permutations(arr: &mut Vec<u8>, start: usize, results: &mut Vec<Vec<u8>>) {
    if start == arr.len() {
        results.push(arr.clone());
        return;
    }
    for i in start..arr.len() {
        arr.swap(start, i);
        generate_permutations(arr, start + 1, results);
        arr.swap(start, i);
    }
}

fn main() {
    let mut digits: Vec<u8> = (1..=8).collect();
    let mut permutations = Vec::new();
    generate_permutations(&mut digits, 0, &mut permutations);
    let mut solutions = 0;

    for perm in permutations {
        let m = perm[0] as u32;
        let u = perm[1] as u32;
        let x = perm[2] as u32;
        let a = perm[3] as u32;
        let s = perm[4] as u32;
        let l = perm[5] as u32;
        let o = perm[6] as u32;
        let n = perm[7] as u32;

        let muxa = m * 1000 + u * 100 + x * 10 + a;
        let slon = s * 1000 + l * 100 + o * 10 + n;
        
        if muxa * a == slon {
            solutions += 1;
            println!("{} x {} = {}", muxa, a, slon);
        }
    }

    println!("Total solutions: {}", solutions);
}
