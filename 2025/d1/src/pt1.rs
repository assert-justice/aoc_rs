
fn modulo(a: i32, b: i32) -> i32{
    ((a % b) + b) % b
}

fn parse(txt: &str) -> Vec<i32>{
    let mut res = Vec::new();
    for line in txt.lines() {
        let (x, xs) = line.split_at(1);
        let mut turn: i32 = xs.parse().unwrap();
        if x == "L"{turn *= -1;}
        res.push(turn);
    }
    res
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let turns = parse(txt);
    let mut dial = 50;
    let mut res = 0;
    for t in turns {
        dial = modulo(dial + t, 100);
        if dial == 0{res += 1;}
    }
    println!("pt 1: {res}");
}
