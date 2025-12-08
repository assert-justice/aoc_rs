use std::{collections::HashSet};


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
}

fn find_distances(boxes: Vec<Vec3I>) -> Vec<(i64, Vec3I, Vec3I)>{
    let mut res = Vec::new();
    for i in 0..(boxes.len()-1) {
        for f in (i+1)..boxes.len() {
            let a = boxes[i];
            let b = boxes[f];
            let d = a.distance_squared(&b);
            res.push((d, a, b));
        }
    }
    res.sort_unstable_by(|a, b|{
        let (ad, _, _) = a;
        let (bd, _, _) = b;
        bd.cmp(ad)
    });
    res
}

fn parse(txt: &str) -> Vec<Vec3I>{
    txt.lines().map(|s|{
        let coords: Vec<i64> = s.split(',').map(|n|n.parse().unwrap()).collect();
        Vec3I { x: coords[0], y: coords[1], z: coords[2] }
    }).collect()
}

fn add_circuit(a: Vec3I, b: Vec3I, mut circuits: Vec<HashSet<Vec3I>>) -> Vec<HashSet<Vec3I>>{
    let mut a_idx = None;
    let mut b_idx = None;
    for (i, c) in circuits.iter().enumerate() {
        if c.contains(&a){
            a_idx = Some(i);
        }
        else if c.contains(&b) {
            b_idx = Some(i);
        }
    }
    if a_idx.is_none() && b_idx.is_none(){
        let mut s = HashSet::new();
        s.insert(a);
        s.insert(b);
        circuits.push(s);
    }
    else if a_idx.is_some() && b_idx.is_some(){
        let a_idx = a_idx.unwrap();
        let b_idx = b_idx.unwrap();
        // Merge. Fucking hell.
        let min = if a_idx < b_idx {a_idx} else{b_idx};
        let max = if a_idx < b_idx {b_idx} else{a_idx};
        let mut ca = circuits.remove(max as usize);
        let cb = circuits.remove(min as usize);
        for c in cb {
            ca.insert(c);
        }
        circuits.push(ca);
    }
    else{
        if a_idx.is_some(){
            circuits[a_idx.unwrap()].insert(b);
        }
        else {
            circuits[b_idx.unwrap()].insert(a);
        }
    }
    circuits
}

#[allow(dead_code)]
pub fn solve(txt: &str) -> String{
    let data = parse(txt);
    let mut disconnected = HashSet::new();
    for idx in 0..data.len() {
        disconnected.insert(data[idx].clone());
    }
    let mut distances = find_distances(data);
    let mut circuits = Vec::new();
    loop {
        let (_, a, b) = distances.pop().unwrap();
        circuits = add_circuit(a, b, circuits);
        disconnected.remove(&a);
        disconnected.remove(&b);
        if disconnected.len() > 0{continue;}
        return format!("{}", a.x * b.x);
    }
}
