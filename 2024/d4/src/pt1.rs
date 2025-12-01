use std::collections::HashMap;


fn parse(txt: &str) -> HashMap<(i32, i32), char>{
    let mut map = HashMap::new();
    for (row, line) in txt.lines().enumerate(){
        for (column, c) in line.chars().enumerate(){
            map.insert((column as i32, row as i32), c);
        }
    }
    map
}

fn match_char(c: char, coord: (i32, i32), map: &HashMap<(i32, i32), char>) -> bool{
    if let Some(spam) = map.get(&coord) {
        return *spam == c;
    }
    false
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let map = parse(txt);
    let mut dirs = Vec::new();
    for dx in -1..2 {
        for dy in -1..2{
            if dx == 0 && dy == 0 {
                continue;
            }
            dirs.push((dx, dy));
        }
    }
    let s = "XMAS";
    let mut res = 0;
    for coord in map.keys() {
        for (dx, dy) in &dirs{
            let (x, y) = coord;
            let mut x = *x;
            let mut y = *y;
            let mut found = true;
            for c in s.chars() {
                if match_char(c, (x,y), &map){
                    x += dx;
                    y += dy;
                }
                else{
                    found = false;
                    break;
                }
            }
            if found{
                res += 1;
            }
        }
    }
    println!("{res}");
}
