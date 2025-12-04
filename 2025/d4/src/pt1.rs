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

#[allow(dead_code)]
pub fn solve(txt: &str){
    let map = parse(txt);
    let mut res = 0;
    for (x,y) in map.keys() {
        if count_neighbors(*x, *y, &map) < 4{
            res += 1;
        }
    }
    println!("pt1: {res}");
}
