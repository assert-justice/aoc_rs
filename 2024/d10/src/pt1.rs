use std::collections::{HashMap, HashSet};

struct Grid{
    data: HashMap<(i32, i32), u8>
}

impl Grid {
    fn score(&self){
        let mut total = 0;
        for (point,h) in self.data.clone(){
            if h == 0{
                // let (x,y) = point;
                // println!("{x}, {y}");
                total += self.pathfind(point);
            }
        }
        println!("{total}");
    }
    fn get_neighbors(&self, point: (i32, i32), height: u8) -> Vec<(i32,i32)>{
        let (cx, cy) = point;
        let dirs = [(1,0),(0,-1),(-1,0),(0,1)];
        let mut res = Vec::new();
        for (dx, dy) in dirs{
            let x = cx + dx;
            let y = cy + dy;
            if let Some(h) = self.data.get(&(x,y)) {
                if *h == height{
                    res.push((x,y));
                }
            }
        }
        res
    }
    fn pathfind(&self, point: (i32, i32)) -> usize{
        let mut live = vec![point];
        let mut h = 0;
        while h < 9 && live.len() > 0 {
            let mut next = HashSet::new();
            while let Some(p) = live.pop() {
                for n in self.get_neighbors(p, h+1){
                    next.insert(n);
                }
            }
            for n in next{
                live.push(n);
            }
            h += 1;
        }
        return live.len();
    }
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let mut y = 0;
    let mut grid = Grid{data: HashMap::new()};
    for line in txt.lines(){
        let mut x = 0;
        for c in line.bytes(){
            grid.data.insert((x,y), c-48);
            x += 1;
        }
        y += 1;
    }
    grid.score();
}