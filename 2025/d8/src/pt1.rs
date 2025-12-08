use std::{collections::{HashMap, HashSet}};


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Vec3I{
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3I {
    fn distance_squared(&self, v: &Vec3I) -> i64{
        let dx = self.x - v.x;
        let dy = self.y - v.y;
        let dz = self.z - v.z;
        dx * dx + dy * dy + dz * dz
    }

    fn get_neighbors(&self) -> Vec<Vec3I>{
        let mut res = Vec::new();
        for dx in -1..2 {
            for dy in -1..2 {
                for dz in -1..2 {
                    if dx == 0 && dy == 0 && dz == 0{continue;}
                    res.push(Vec3I { x: self.x+dx, y: self.y+dy, z: self.z+dz });
                }
            }
        }
        res
    }
}

struct Cell{
    boxes: Vec<Vec3I>,
}

struct Grid{
    cells: HashMap<Vec3I, Cell>,
    // circuits: Vec<HashSet<Vec3I>>, 
    cell_width: i64,
}

impl Grid {
    fn new(cell_width: i64, boxes: Vec<Vec3I>) -> Self{
        let mut grid = Self { cells: HashMap::new(), cell_width };
        for b in boxes {
            grid.add_box(b);
        }
        grid
    }
    fn add_box(&mut self, j_box: Vec3I){
        let cx = j_box.x / self.cell_width;
        let cy = j_box.y / self.cell_width;
        let cz = j_box.z / self.cell_width;
        let cv = Vec3I{x: cx, y: cy, z: cz};
        if let Some(cell) = self.cells.get_mut(&cv) {
            cell.boxes.push(j_box);
        }
        else{
            let cell = Cell{boxes: vec![j_box]};
            self.cells.insert(cv, cell);
        }
    }
    fn find_distances(&self) -> Vec<(i64, Vec3I, Vec3I)>{
        let mut res = Vec::new();
        for (pos, cell) in &self.cells {
            let neighbors = pos.get_neighbors();
            let mut boxes = cell.boxes.clone();
            for n in neighbors {
                if let Some(p) = self.cells.get(&n) {
                    boxes.extend_from_slice(&p.boxes);
                }
            }
            for i in 0..(boxes.len()-1) {
                for f in (i+1)..boxes.len() {
                    let a = boxes[i];
                    let b = boxes[f];
                    let d = a.distance_squared(&b);
                    res.push((d, a, b));
                }
            }
        }
        res.sort_unstable_by(|a, b|{
            let (ad, _, _) = a;
            let (bd, _, _) = b;
            ad.cmp(bd)
        });
        res
    }
}

fn parse(txt: &str) -> Vec<Vec3I>{
    txt.lines().map(|s|{
        let coords: Vec<i64> = s.split(',').map(|n|n.parse().unwrap()).collect();
        Vec3I { x: coords[0], y: coords[1], z: coords[2] }
    }).collect()
}

fn add_circuit(a: Vec3I, b: Vec3I, mut circuits: Vec<HashSet<Vec3I>>) -> Vec<HashSet<Vec3I>>{
    let mut idx = None;
    for (i, c) in circuits.iter().enumerate() {
        if c.contains(&a){
            idx = Some(i);
            break;
        }
        else if c.contains(&b) {
            idx = Some(i);
        }
    }
    if let Some(idx) = idx {
        circuits[idx].insert(a);
        circuits[idx].insert(b);
    }
    else {
        let mut s = HashSet::new();
        s.insert(a);
        s.insert(b);
        // println!("new circuit");
        circuits.push(s);
    }
    circuits
}

#[allow(dead_code)]
pub fn solve(txt: &str) -> String{
    let data = parse(txt);
    let grid = Grid::new(10000, data);
    let distances = grid.find_distances();
    let mut circuits = Vec::new();
    for idx in 0..10 {
        let (_, a, b) = distances[idx];
        circuits = add_circuit(a, b, circuits);
        let mut res: Vec<usize> = circuits.iter().map(|c|c.len()).collect();
        res.sort();
        res.reverse();
        println!("{idx}: {:?}", res);
    }
    let mut res: Vec<usize> = circuits.iter().map(|c|c.len()).collect();
    res.sort();
    res.reverse();
    format!("{}", res[0] * res[1] * res[2])
}
