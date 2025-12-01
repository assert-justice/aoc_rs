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
    let mut res = 0;
    for (coord, ch) in &map {
        if *ch != 'A' {continue;}
        let (cx, cy) = coord;
        let a = match_char('M', (cx-1, cy-1), &map) && match_char('S', (cx+1, cy+1), &map) || (match_char('S', (cx-1, cy-1), &map) && match_char('M', (cx+1, cy+1), &map));
        let b = match_char('M', (cx+1, cy-1), &map) && match_char('S', (cx-1, cy+1), &map) || (match_char('S', (cx+1, cy-1), &map) && match_char('M', (cx-1, cy+1), &map));
        if a && b{res += 1;}
    }
    println!("{res}");
}
