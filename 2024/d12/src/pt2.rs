use std::collections::{HashMap, HashSet};

type Point = (i64, i64);
#[derive(Debug)]
#[allow(dead_code)]
struct Group{
    id: char,
    members: HashSet<Point>,
    area: usize,
    corners: usize,
}
struct Puzzle{
    grid: HashMap<Point, char>,
    groups: Vec<Group>,
}

impl Puzzle {
    pub fn new(txt: &str) -> Self{
        let mut puzzle = Puzzle{
            grid: HashMap::new(),
            groups: Vec::new(),
        };
        let mut y = 0;
        for line in txt.lines(){
            let mut x = 0;
            for c in line.chars(){
                puzzle.grid.insert((x,y), c);
                x += 1;
            }
            y += 1;
        }
        puzzle.find_groups();
        puzzle
    }
    fn get_neighbors(&self, (x,y): Point, c: char) -> Vec<Point>{
        let directions = [(1,0),(0,-1),(-1,0),(0,1)];
        let mut out = Vec::new();
        for (dx, dy) in directions{
            let point = (x+dx,y+dy);
            if let Some(id) = self.grid.get(&point) {
                if *id == c{
                    out.push(point);
                }
            }
        }
        out
    }
    fn count_corners(&self, point: Point, c: char) -> usize{
        let mut corners = 0;
        let (x,y) = point;
        let dirs = [(1,0),(1,-1),(0,-1),(-1,-1),(-1,0),(-1,1),(0,1),(1,1),(1,0)];
        let mut mask = Vec::new();
        for (dx, dy) in dirs{
            let p = (x+dx, y+dy);
            if let Some(ch) = self.grid.get(&p){
                if *ch == c{
                    mask.push(true);
                    continue;
                }
            }
            mask.push(false);
        }
        println!("{:?}", mask);
        for i in (1..mask.len()).step_by(2){
            println!("{} {} {}", mask[i-1], mask[i], mask[i+1]);
            if !mask[i] && !mask[i-1] && !mask[i+1]{
                corners += 1;
                // println!("match!");
            }
            else if !mask[i] && mask[i-1] && mask[i+1]{
                // println!("match!");
                corners += 1
            }
            else if mask[i] && !mask[i-1] && !mask[i+1]{
                // println!("match!");
                corners += 1
            }
        }
        // println!("{corners}");
        corners
    }
    fn flood(&self, point: Point, c: char) -> Group{
        let mut members = HashSet::new();
        let mut open = Vec::new();
        // let mut perimeter = 0;
        open.push(point);
        let mut corners = 0;
        println!("{c}");
        while open.len() > 0 {
            while let Some(p) = open.pop() {
                members.insert(p);
                let neighbors = self.get_neighbors(p, c);
                let co= self.count_corners(p, c);
                println!("{:?} {}", p, co);
                corners += co;
                // perimeter += 4 - neighbors.len();
                // println!("{}", neighbors.len());
                for n in neighbors{
                    if open.contains(&n){continue;}
                    if members.contains(&n){continue;}
                    open.push(n);
                }
            }
        }
        let area = members.len();
        let g = Group{
            id: c,
            members,
            area,
            corners,
        };
        // println!("{:?}", g);
        g
    }
    pub fn find_groups(&mut self){
        let mut visited:HashSet<Point> = HashSet::new();
        for (point, c) in &self.grid{
            if visited.contains(&point){continue;}
            // println!("{} {:?}", c, point);
            let group = self.flood(*point, *c);
            for point in &group.members{
                visited.insert(*point);
            }
            self.groups.push(group)
            // perimeter: usize,;
        }
    }
    pub fn eval(&self){
        let mut total = 0;
        for g in &self.groups{
            // println!("id: {}, area: {}, perimeter: {}", g.id, g.area, g.perimeter);
            // println!("{:?}", g);
            total += g.area * g.corners;
        }
        total /= 2;
        println!("{total}");
    }
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let mut puzzle = Puzzle::new(txt);
    puzzle.find_groups();
    puzzle.eval();
}
