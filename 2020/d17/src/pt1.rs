use std::collections::HashSet;

type Coord = (i64,i64,i64);

struct Puzzle{
    cubes: HashSet<Coord>,
}

impl Puzzle {
    fn count_active(&self) -> usize{
        self.cubes.len()
    }
    fn get_adj(coord: Coord) -> Vec<Coord>{
        let mut res = Vec::new();
        let (x, y, z) = coord;
        for dx in -1..2 {
            for dy in -1..2 {
                for dz in -1..2 {
                    if dx == 0 && dy == 0 && dz == 0{continue;}
                    res.push((x+dx,y+dy,z+dz));
                }
            }
        }
        res
    }
    fn count_living_neighbors(&self, coord: Coord) -> usize{
        Puzzle::get_adj(coord).iter().filter(|c|self.cubes.contains(c)).count()
    }
    fn simulate(&mut self){
        let mut dead_cells = HashSet::new();
        for coord in &self.cubes {
            for c in Puzzle::get_adj(*coord) {
                if !self.cubes.contains(&c){dead_cells.insert(c);}
            }
        }
        let mut remove = Vec::new();
        let mut add = Vec::new();
        for coord in self.cubes.clone() {
            let count = self.count_living_neighbors(coord);
            if count != 2 && count != 3{
                remove.push(coord);
            }
        }
        for coord in dead_cells {
            if self.count_living_neighbors(coord) == 3{add.push(coord);}
        }
        for coord in remove {
            self.cubes.remove(&coord);
        }
        for coord in add {
            self.cubes.insert(coord);
        }
    }
}

fn parse(txt: &str) -> Puzzle{
    let mut cubes = HashSet::new();
    for (y, line) in txt.lines().enumerate() {
        for (x, c) in line.char_indices() {
            if c == '#'{
                cubes.insert((x.try_into().unwrap(),y.try_into().unwrap(),0));
            }
        }
    }
    Puzzle { cubes }
}

#[allow(dead_code)]
pub fn solve(txt: &str) -> String{
    let mut data = parse(txt);
    for _ in 0..6{
        data.simulate();
    }
    format!("{}", data.count_active())
}
