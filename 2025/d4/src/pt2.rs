use std::collections::HashMap;

fn parse(txt: &str) -> HashMap<(i64, i64), char>{
    let mut res = HashMap::new();
    for (y,line) in txt.lines().enumerate() {
        for (x, c) in line.char_indices() {
            if c == '@'{
                res.insert((x.try_into().unwrap(),y.try_into().unwrap()), c);
            }
        }
    }
    res
}

fn count_neighbors(x: i64, y: i64, map: &HashMap<(i64, i64), char>) -> usize{
    let mut res = 0;
    for dx in -1..2 {
        for dy in -1..2 {
            if dx == 0 && dy == 0{continue;}
            if map.contains_key(&(x+dx, y+dy)){
                res += 1;
            }
        }
    }
    res
}

fn prune(map: &HashMap<(i64, i64), char>) -> Vec<(i64,i64)>{
    let mut remove = Vec::new();
    for (x,y) in map.keys() {
        if count_neighbors(*x, *y, &map) < 4{
            remove.push((*x, *y));
        }
    }
    remove
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let mut map = parse(txt);
    let mut res = 0;
    loop {
        let remove = prune(&map);
        if remove.len() == 0{
            break;
        }
        res += remove.len();
        for coord in remove {
            map.remove(&coord);
        }
    }
    println!("pt 2: {res}");
}
