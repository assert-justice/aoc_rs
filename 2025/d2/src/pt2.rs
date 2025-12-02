
fn parse(txt: &str) -> Vec<(usize,usize)>{
    txt.trim().split(',').map(|line|{
        // println!("{line}");
        let line:Vec<usize> = line.split('-').map(|n|n.parse().unwrap()).collect();
        (line[0], line[1])
    }).collect()
}

fn factorize(x: usize) -> Vec<usize>{
    let mut res = vec![1];
    for f in 2..(x/2+1) {
        if x%f == 0{res.push(f);}
    }
    res
}

fn is_valid(x: usize) -> bool{
    let s = x.to_string();
    let factors = factorize(s.len());
    for f in factors {
        // println!("{f}");
        let p = &s[0..f];
        // println!("'{p}'");
        let mut idx = 0;
        let mut valid = false;
        while idx < s.len() {
            // println!("checking {}", &s[idx..p.len()]);
            if *p != s[idx..(idx + p.len())]{
                valid = true;
                break;
            }
            idx += p.len();
        }
        if !valid{return false;}
    }
    true
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    // println!("ans: {}", is_valid(123123));
    let ranges = parse(txt);
    let mut res = 0;
    for (start, end) in ranges {
        for x in start..(end+1) {
            if !is_valid(x){
                println!("{x}");
                res += x;
            }
        }
    }
    println!("pt 2: {res}");
}
