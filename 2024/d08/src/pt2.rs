use std::collections::{HashMap, HashSet};

struct Puzzle{
    width: i32,
    height: i32,
    nodes: HashMap<char, Vec<(i32,i32)>>,
}

impl Puzzle {
    pub fn new(width: i32, height: i32) -> Self{
        Self{
            width,
            height,
            nodes: HashMap::new(),
        }
    }
    pub fn add_node(&mut self, c: char, x: i32, y: i32){
        match self.nodes.get(&c) {
            Some(_) => {
                let v: &mut Vec<(i32,i32)> = &mut self.nodes.get_mut(&c).unwrap();
                v.push((x,y));
            },
            None => {
                let mut v = Vec::new();
                v.push((x,y));
                self.nodes.insert(c, v);
            }
        };
    }
    fn is_oob(&self, x: i32, y: i32) -> bool{
        if x < 0 || x >= self.height{return true;}
        if y < 0 || y >= self.width{return true;}
        return false;
    }
    pub fn solve(&self){
        // let mut total = 0;
        let nodes = self.nodes.clone();
        let mut anti_nodes = HashSet::new();
        for (_,v) in nodes{
            // for each two nodes in v
            // find anti nodes
            // if anti node is on map add to total
            for i in 0..v.len(){
                for f in 0..v.len(){
                    if i == f {continue;}
                    let (xi,yi) = v[i];
                    let (xf, yf) = v[f];
                    let dx = xi - xf;
                    let dy = yi - yf;
                    let mut x = xi;
                    let mut y = yi;
                    while !self.is_oob(x, y){
                        // println!("x: {x} y: {y}");
                        // total += 1;
                        anti_nodes.insert((x,y));
                        x += dx;
                        y += dy;
                    }
                }
            }
            // println!("{c} {total}");
        }
        println!("{}", anti_nodes.len());
    }
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let lines: Vec<&str> = txt.lines().collect();
    let height = txt.lines().count();
    let width = lines[0].chars().count();
    let mut puzzle = Puzzle::new(width.try_into().unwrap(), height.try_into().unwrap());
    let mut y = 0;
    for line in txt.lines(){
        let mut x = 0;
        for c in line.chars(){
            // nodes.insert((x,y), c);
            if c != '.' {puzzle.add_node(c, x, y)};
            x += 1;
        }
        y += 1;
    }
    puzzle.solve();
}
