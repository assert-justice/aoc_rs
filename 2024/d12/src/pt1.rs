use std::collections::{HashMap, HashSet};

type Point = (i64, i64);
#[derive(Debug)]
#[allow(dead_code)]
struct Group{
    id: char,
    members: HashSet<Point>,
    area: usize,
    perimeter: usize,
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
    fn flood(&self, point: Point, c: char) -> Group{
        let mut members = HashSet::new();
        let mut open = Vec::new();
        let mut perimeter = 0;
        open.push(point);
        // println!("{c}");
        while open.len() > 0 {
            while let Some(p) = open.pop() {
                members.insert(p);
                let neighbors = self.get_neighbors(p, c);
                perimeter += 4 - neighbors.len();
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
            perimeter,
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
            self.groups.push(group);
        }
    }
    pub fn eval(&self){
        let mut total = 0;
        for g in &self.groups{
            // println!("id: {}, area: {}, perimeter: {}", g.id, g.area, g.perimeter);
            total += g.area * g.perimeter;
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
