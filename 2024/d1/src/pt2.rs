use std::collections::HashMap;


fn parse(txt: &str) -> (Vec<u32>, Vec<u32>){
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in txt.lines(){
        let line: Vec<u32> = line.split("   ").map(|s| s.parse::<u32>().unwrap()).collect();
        left.push(line[0]);
        right.push(line[1]);
    }
    (left, right)
}

fn get_freq(xs: Vec<u32>) -> HashMap<u32, u32>{
    let mut freq = HashMap::new();
    for x in xs {
        let f = freq.get(&x).unwrap_or(&0) + 1;
        freq.insert(x, f);
    }
    freq
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let (left, right) = parse(txt);
    let freq = get_freq(right);
    let res = left.iter().fold(0, |acc, x| acc + x * freq.get(x).unwrap_or(&0));
    println!("{res}");
}
