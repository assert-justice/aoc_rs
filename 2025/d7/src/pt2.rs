use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Vec2I{
    x: i64,
    y: i64,
}

impl Vec2I {
    fn new(x: i64, y: i64) -> Self{
        Self { x, y }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum CellType{
    None,
    Splitter,
    // Beam,
}

struct Puzzle{
    data: HashMap<Vec2I, CellType>,
    start: Vec2I,
    height: u64,
    width: u64,
    mem: HashMap<Vec2I, usize>,
    // splits: usize,
    // dead_tachyons: usize,
}

impl Puzzle {
    fn parse(txt: &str) -> Self{
        let mut data = HashMap::new();
        // let mut tachyons = Vec::new();
        let mut start = Vec2I::new(0, 0);
        let mut height = 0;
        for (row, line) in txt.lines().enumerate() {
            height = line.chars().count();
            for (col, c) in line.char_indices(){
                match c {
                    'S' => {
                        start = Vec2I::new(col as i64, row as i64);
                    },
                    '^' => {
                        data.insert(Vec2I::new(col as i64, row as i64), CellType::Splitter);
                    },
                    _ => {},
                }
            }
        }
        let width = txt.lines().count();
        
        // Self { data, tachyons, width: width as u64, height: height as u64, splits: 0}
        Self { data, start, width: width as u64, height: height as u64, mem: HashMap::new()}
        // Self { data, tachyons, width: width as u64, height: height as u64, dead_tachyons: 0}
    }

    fn on_grid(& self, vec: &Vec2I) -> bool{
        if vec.x < 0 || vec.x as u64 >= self.width{
            return false;
        }
        if vec.y < 0 || vec.y as u64 >= self.height{
            return false;
        }
        true
    }

    fn get_cell(&self, vec: &Vec2I) -> CellType{
        if let Some(v) = self.data.get(vec) {
            *v
        }
        else {
            CellType::None
        }
    }

    // fn step(&mut self) -> bool{
    //     let mut ts = Vec::new();
    //     for t in &self.tachyons {
    //         if !self.on_grid(t){
    //             // self.dead_tachyons += 1;
    //             continue;
    //         }
    //         let ct = self.get_cell(t);
    //         match ct {
    //             CellType::Splitter => {
    //                 // self.splits += 1;
    //                 ts.push(Vec2I::new(t.x - 1, t.y));
    //                 ts.push(Vec2I::new(t.x + 1, t.y));
    //             },
    //             _ => {
    //                 self.data.insert(*t, CellType::Beam);
    //                 ts.push(Vec2I::new(t.x, t.y + 1));
    //             },
    //             // CellType::Beam => {},
    //         }
    //     }
    //     self.tachyons = ts;
    //     self.tachyons.len() > 0
    // }
    fn calc(&mut self, pos: Vec2I) -> usize{
        if !self.on_grid(&pos){
            return 1;
        }
        if let Some(v) = self.mem.get(&pos) {
            return *v
        }
        let res = match self.get_cell(&pos) {
            CellType::None => self.calc(Vec2I { x: pos.x, y: pos.y + 1 }),
            CellType::Splitter => self.calc(Vec2I { x: pos.x+1, y: pos.y }) + self.calc(Vec2I { x: pos.x-1, y: pos.y })
        };
        self.mem.insert(pos, res);
        res
    }

    fn solve(&mut self) -> usize{
        // while self.step() {}
        self.calc(self.start)
        // self.dead_tachyons
        // println!("{}", self.dead_tachyons);
        // (0..self.width).filter(|x|self.get_cell(&Vec2I { x: *x as i64, y: self.height as i64 -1 }) == CellType::Beam).count()
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String{
        let mut res = String::new();
        for row in 0..self.height {
            for col in 0..self.width {
                match self.get_cell(&Vec2I { x: col as i64, y: row as i64 }) {
                    CellType::None => {res.push('.');},
                    // CellType::Beam => {res.push('|');},
                    CellType::Splitter => {res.push('^');},
                }
            }
            res.push('\n');
        }
        res
    }
}


#[allow(dead_code)]
pub fn solve(txt: &str) -> String{
    let mut data = Puzzle::parse(txt);
    // println!("{}", data.to_string());
    let ans = data.solve();
    // println!("{}", data.to_string());
    format!("{}", ans)
}
