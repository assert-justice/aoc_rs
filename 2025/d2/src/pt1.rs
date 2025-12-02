
fn parse(txt: &str) -> Vec<(usize,usize)>{
    txt.trim().split(',').map(|line|{
        // println!("{line}");
        let line:Vec<usize> = line.split('-').map(|n|n.parse().unwrap()).collect();
        (line[0], line[1])
    }).collect()
}

fn is_valid(x: usize) -> bool{
    let s = x.to_string();
    if s.len() % 2 == 1{return true;}
    let (a,b) = s.split_at(s.len()/2);
    a != b
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let ranges = parse(txt);
    let mut res = 0;
    for (start, end) in ranges {
        for x in start..(end+1) {
            if !is_valid(x){res += x;}
        }
    }
    println!("pt 1: {res}");
}
