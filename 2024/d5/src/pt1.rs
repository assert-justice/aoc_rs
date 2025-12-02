use std::collections::{HashMap, HashSet};

fn parse(txt: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>){
    let mut deps: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut vec = Vec::new();
    let mut lines = txt.lines();
    loop {
        let line = lines.next().unwrap();
        if line.len() == 0{break;}
        let xs: Vec<u32> = line.split('|').map(|s|s.parse().unwrap()).collect();
        let a = xs[0];
        let b = xs[1];
        if let Some(v) = deps.get_mut(&b) {
            v.push(a);
        }
        else {
            deps.insert(b, vec![a]);
        }
    }
    while let Some(line) = lines.next() {
        let xs = line.split(',').map(|s|s.parse().unwrap()).collect();
        vec.push(xs);
    }
    (deps, vec)
}

fn vec_to_set(vec: &Vec<u32>) -> HashSet<u32>{
    vec.iter().fold(HashSet::new(), |mut set, x|{set.insert(*x); set})
}

fn is_correct(xs: &Vec<u32>, deps: &HashMap<u32, Vec<u32>>) ->bool{
    let set = vec_to_set(&xs);
    let mut seen = HashSet::new();
    for x in xs {
        if let Some(d) = deps.get(&x) {
            for n in d {
                if set.contains(n) && !seen.contains(n){return false;}
            }
        }
        seen.insert(x);
    }
    true
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let (deps, vec) = parse(txt);
    let mut res = 0;
    for xs in vec{
        if is_correct(&xs, &deps){
            let idx = xs.len() / 2;
            res += xs[idx];
        }
    }
    println!("pt 1: {res}");
}
