
fn parse(txt: &str) -> Vec<Vec<i32>>{
    let mut res = Vec::new();
    for line in txt.lines(){
        res.push(line.split(' ').map(|x|x.parse::<i32>().unwrap()).collect());
    }
    res
}

fn get_diffs(xs: Vec<i32>) -> Vec<i32>{
    let mut res = Vec::new();
    for idx in 0..(xs.len() - 1) {
        res.push(xs[idx] - xs[idx+1]);
    }
    res
}

fn is_safe(xs: Vec<i32>) -> bool{
    let xs = get_diffs(xs);
    let mut lz = false;
    let mut gz = false;
    for x in xs {
        if x < 0{
            lz = true;
        }
        if x > 0 {
            gz = true;
        }
        if lz && gz {
            return false;
        }
        let ax = x.abs();
        if ax > 3 || ax < 1{
            return false;
        }
    }
    true
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let res = parse(txt).iter().filter(|v|is_safe(v.to_vec())).count();
    println!("{res}");
}
