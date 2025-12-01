
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

#[allow(dead_code)]
pub fn solve(txt: &str){
    let (mut left, mut right) = parse(txt);
    left.sort_unstable();
    right.sort_unstable();
    let res = left.iter().zip(right).fold(0, |acc, (a,b)|acc + a.abs_diff(b));
    println!("{res}");
}
