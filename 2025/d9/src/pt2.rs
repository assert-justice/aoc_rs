// use std::{collections::HashMap, i64};


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Vec2I{
    x: i64,
    y: i64,
}

#[allow(dead_code)]
impl Vec2I {
    fn new(x: i64, y: i64) -> Self{
        Self { x, y }
    }
    fn parse(txt: &str) -> Self{
        let args: Vec<i64> = txt.split(',').map(|s|s.parse().unwrap()).collect();
        Vec2I::new(args[0], args[1])
    }
    fn add(&self, v: Vec2I) -> Vec2I{
        Vec2I { x: self.x+v.x, y: self.y+v.y }
    }
    fn sub(&self, v: Vec2I) -> Vec2I{
        Vec2I { x: self.x-v.x, y: self.y-v.y }
    }
    fn mul(&self, scale: i64) -> Vec2I{
        Vec2I::new(self.x * scale, self.y * scale)
    }
    fn sign(&self) -> Vec2I{
        Vec2I { x: self.x.signum(), y: self.y.signum() }
    }
    const ZERO: Vec2I = Vec2I { x: 0, y: 0 };
    const ONE: Vec2I = Vec2I { x: 1, y: 1 };
}

// #[derive(PartialEq, Eq, Clone, Copy, Debug)]
// enum CellType{
//     None,
//     Red,
//     Green,
// }

// struct Grid{
//     data: HashMap<Vec2I, CellType>,
//     min: Vec2I,
//     max: Vec2I,
// }

// impl Grid {
//     fn new() -> Self{
//         Self { data: HashMap::new(), min: Vec2I::ZERO, max: Vec2I::ONE.mul(i64::MIN) }
//     }
//     fn add(&mut self, v: Vec2I, cell_type: CellType){
//         self.data.insert(v, cell_type);
//         if v.x < self.min.x{self.min.x = v.x;}
//         if v.x > self.max.x{self.max.x = v.x;}
//         if v.y < self.min.y{self.min.y = v.y;}
//         if v.y > self.max.y{self.max.y = v.y;}
//     }
//     fn get(&self, v: Vec2I) -> CellType{
//         if let Some(cell_type) = self.data.get(&v) {
//             *cell_type
//         }
//         else {
//             CellType::None
//         }
//     }
//     fn to_string(&self) -> String{
//         let mut res = String::new();
//         if self.data.len() == 0{ return "".to_string();}
//         for row in self.min.y..(self.max.y+2){
//             for col in self.min.x..(self.max.x+2) {
//                 let c = match self.get(Vec2I::new(col, row)) {
//                     CellType::None => '.',
//                     CellType::Red => '#',
//                     CellType::Green => 'X',
//                 };
//                 res.push(c);
//             }
//             res.push('\n');
//         }
//         res
//     }
//     fn add_points(&mut self, a: Vec2I, b: Vec2I){
//         debug_assert!(a.x == b.x || a.y == b.y);
//         let delta = b.sub(a);
//         let dir = delta.sign();
//         let mut v = a;
//         let len = if dir.x == 0 {delta.y.abs()} else {delta.x.abs()};
//         for _ in 0..(len-1) {
//             v = v.add(dir);
//             self.add(v, CellType::Green);
//         }
//     }
//     fn fill(&mut self){
//         // find left midpoint
//         let mid_y = (self.max.y - self.min.y) / 2;
//         let dir = Vec2I::new(1, 0);
//         let mut v = Vec2I::new(self.min.x, mid_y);
//         let mut found = false;
//         loop {
//             let t = self.get(v);
//             if !found && t != CellType::None{
//                 found = true;
//             }
//             if found && t == CellType::None{
//                 break;
//             }
//             v = v.add(dir);
//         }
//         // at this point, v is inside the shape.
//         println!("found inside! {:?}", v);
//         self.add(v, CellType::Green);
//     }
// }


// fn parse(txt: &str) -> Grid{
//     let data: Vec<Vec2I> = txt.lines().map(|l|Vec2I::parse(l)).collect();
//     // println!("{}", data.len());
//     let mut res = Grid::new();
//     for idx in 0..data.len() {
//         let v = data[idx];
//         let next = data[(idx+1)%data.len()];
//         res.add(v, CellType::Red);
//         res.add_points(v, next);
//     }
//     res.fill();
//     res
// }

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Range{
    start: i64,
    end: i64,
}
#[allow(dead_code)]
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum RangeOverlapStatus {
    None,
    Contains(Range),
    ContainedBy(Range),
    Overlaps(Range),
}

#[allow(dead_code)]
impl Range {
    fn new(start: i64, end: i64) -> Self{
        if start < end{
            Self { start, end }
        }
        else {
            Self { start: end, end: start }
        }
    }
    fn in_range(&self, num: i64) -> bool{
        num >= self.start && num <= self.end
    }
    fn get_overlap_status(&self, r: Range) -> RangeOverlapStatus{
        let a = r.in_range(self.start);
        let b = r.in_range(self.end);
        let c = self.in_range(r.start);
        let d = self.in_range(r.end);
        if a && b{return RangeOverlapStatus::ContainedBy(r)}
        if c && d{return RangeOverlapStatus::Contains(r)}
            if a || b || c || d{
            let min = if self.start < r.start{self.start} else{r.start};
            let max = if self.end > r.end{self.end} else{r.end};
            return RangeOverlapStatus::Overlaps(Range { start: min, end: max });
        }
        RangeOverlapStatus::None
    }
    fn has_overlap(&self, r: Range) -> Option<Range>{
        match self.get_overlap_status(r) {
            RangeOverlapStatus::None => None,
            RangeOverlapStatus::Overlaps(new_range) => Some(new_range),
            RangeOverlapStatus::ContainedBy(new_range) => Some(new_range),
            RangeOverlapStatus::Contains(_) => Some(*self),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Line{
    a: Vec2I,
    b: Vec2I,
    diff: Vec2I,
    is_vert: bool,
}

impl Line {
    fn new(a: Vec2I, b: Vec2I) -> Self {
        let diff = a.sub(b);
        let is_vert = diff.x == 0;
        let min_x = if a.x < b.x {a.x} else {b.x};
        let max_x = if a.x > b.x {a.x} else {b.x};
        let min_y = if a.y < b.y {a.y} else {b.y};
        let max_y = if a.y > b.y {a.y} else {b.y};
        Self { a: Vec2I::new(min_x, min_y), b: Vec2I::new(max_x, max_y), diff, is_vert}
    }
    fn get_range_x(&self) -> Range{
        Range::new(self.a.x, self.b.x)
    }
    fn get_range_y(&self) -> Range{
        Range::new(self.a.y, self.b.y)
    }
    fn is_line_crossing(&self, l: Line) -> bool{
        // if both lines are oriented the same way
        if self.is_vert == l.is_vert{
            if self.is_vert{
                if self.a.x != l.a.x{return false;}
                let a = Range::new(self.a.y, self.b.y);
                let b = Range::new(l.a.y, l.b.y);
                let status = a.get_overlap_status(b);
                // println!("{:?}", status);
                match status {
                    RangeOverlapStatus::None => {return false;}
                    RangeOverlapStatus::ContainedBy(_) => {return false;}
                    // RangeOverlapStatus::Contains(_) => {return false;}
                    _ => {return true;}
                }
            }
            else {
                if self.a.y != l.a.y{return false;}
                let a = Range::new(self.a.x, self.b.x);
                let b = Range::new(l.a.x, l.b.x);
                match a.get_overlap_status(b) {
                    RangeOverlapStatus::None => {return false;}
                    RangeOverlapStatus::ContainedBy(_) => {return false;}
                    // RangeOverlapStatus::Contains(_) => {return false;}
                    _ => {return true;}
                }
            }
        }
        // if the lines are oriented differently
        if self.is_vert{
            let v = self.get_range_y();
            let h = l.get_range_x();
            if v.in_range(l.a.y) && h.in_range(self.a.x){return true;}
        }
        else {
            let v = l.get_range_y();
            let h = self.get_range_x();
            if v.in_range(self.a.y) && h.in_range(l.a.x){return true;}
        }
        false
    }
}

struct Shape{
    lines: Vec<Line>,
}

impl Shape {
    fn new(points: &Vec<Vec2I>) -> Self{
        let mut lines = Vec::new();
        for idx in 0..points.len() {
            let a = points[idx];
            let b = points[(idx+1)%points.len()];
            lines.push(Line::new(a, b));
        }
        Self { lines }
    }
    fn does_box_fit(&self, my_box: &Box) -> bool{
        for l in my_box.lines {
            if !self.is_point_inside(l.a){
                println!("point failed. box: {:?}, {:?}, point: {:?}\n", my_box.a, my_box.b, l.a);
                return false;
            }
            if self.is_line_crossing(l){return false;}
        }
        true
    }
    fn is_point_inside(&self, v: Vec2I) -> bool{
        let end = Vec2I::new(i64::MAX, v.y);
        let line = Line::new(v, end);
        let num_crossings = self.lines.iter().filter(|l|l.is_line_crossing(line)).count();
        num_crossings % 2 == 1
    }
    fn is_line_crossing(&self, line: Line) -> bool{
        for l in &self.lines {
            if l.is_line_crossing(line){return true;}
        }
        false
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Box{
    a: Vec2I, 
    b: Vec2I,
    lines: [Line; 4],
}

impl Box {
    fn new(a: Vec2I, b: Vec2I) -> Self{
        let min_x = if a.x < b.x {a.x} else {b.x};
        let max_x = if a.x > b.x {a.x} else {b.x};
        let min_y = if a.y < b.y {a.y} else {b.y};
        let max_y = if a.y > b.y {a.y} else {b.y};
        let lines = [
            Line::new(Vec2I::new(min_x, min_y), Vec2I::new(min_x, max_y)),
            Line::new(Vec2I::new(min_x, max_y), Vec2I::new(max_x, max_y)),
            Line::new(Vec2I::new(max_x, max_y), Vec2I::new(max_x, min_y)),
            Line::new(Vec2I::new(max_x, min_y), Vec2I::new(min_x, min_y)),
        ];
        Self { a, b, lines }
    }
    fn area(&self) -> i64{
        let c = self.a.sub(self.b);
        ((c.x+1) * (c.y+1)).abs()
    }
}

fn tests() -> bool{
    let a = Line::new(Vec2I::ZERO, Vec2I::new(0, 12));
    let b = a;
    debug_assert!(!a.is_line_crossing(b));
    let a = Line::new(Vec2I::ZERO, Vec2I::new(12, 0));
    let b = a;
    debug_assert!(!a.is_line_crossing(b));
    let a = Line::new(Vec2I::ZERO, Vec2I::new(12, 0));
    let b = Line::new(Vec2I::ZERO, Vec2I::new(13, 0));
    debug_assert!(!a.is_line_crossing(b));
    let a = Line::new(Vec2I::ZERO, Vec2I::new(13, 0));
    let b = Line::new(Vec2I::ZERO, Vec2I::new(12, 0));
    debug_assert!(a.is_line_crossing(b));
    true
}

fn parse(txt: &str) -> Vec<Vec2I>{
    txt.lines().map(|l|Vec2I::parse(l)).collect()
}

#[allow(dead_code)]
pub fn solve(txt: &str) -> String{
    debug_assert!(tests());
    let data = parse(txt);
    let shape = Shape::new(&data);
    let mut area = 0;
    for i in 0..(data.len()-1) {
        for f in (i+1)..data.len() {
            // println!("i: {}, f: {}", i, f);
            let my_box = Box::new(data[i], data[f]);
            // println!("new box: {:?},{:?}", my_box.a, my_box.b);
            if !shape.does_box_fit(&my_box){continue;}
            let a = my_box.area();
            if a > area{area = a;}
        }
    }
    format!("{}", area)
}
