
fn parse(txt: &str) -> Vec<Vec<char>>{
    let mut res = Vec::new();
    for line in txt.lines() {
        res.push(line.chars().collect());
    }
    res
}

fn get_max_charge(xs: &Vec<char>) -> usize{
    let mut max = 0;
    for i in 0..xs.len() {
        for f in (i+1)..xs.len() {
            if i == f{continue;}
            let mut s = String::new();
            s.push(xs[i]);
            s.push(xs[f]);
            let x: usize = s.parse().unwrap();
            if x > max {max = x;}
        }
    }
    max
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let batteries = parse(txt);
    // let res: usize = batteries.iter().map(|xs|get_max_charge(xs)).sum();
    let mut res = 0;
    for b in batteries {
        let x = get_max_charge(&b);
        // println!("{x}");
        res += x;
    }
    println!("pt 1: {res}");
}
