use std::collections::HashMap;


struct Grid{
    data: HashMap<(i64,i64), char>,
    rx: i64,
    ry: i64,
    width: i64,
    height: i64,
}

impl Grid {
    pub fn new() -> Self{
        Grid{
            data: HashMap::new(),
            rx: 0,
            ry: 0,
            width: 0,
            height: 0,
        }
    }
    pub fn add_tile(&mut self, x: i64, y: i64, c: char){
        if x >= self.width{
            self.width = x + 1;
        }
        if y >= self.height{
            self.height = y + 1;
        }
        if c == '@'{
            self.rx = x; self.ry = y;
        }
        else{
            self.data.insert((x,y), c);
        }
    }
    pub fn print(&self){
        for y in 0..self.height{
            for x in 0..self.width{
                if x == self.rx && y == self.ry{
                    print!("@");
                }
                else if let Some(c) = self.data.get(&(x,y)){
                    print!("{}", *c);
                }
                else{
                    print!(".");
                }
            }
            print!("\n");
        }
    }
    pub fn step(&mut self, c: char){
        let dir = match c {
            '>' => (1,0),
            '^' => (0,-1),
            '<' => (-1,0),
            'v' => (0,1),
            _ => panic!("Oops"),
        };
        let (dx, dy) = dir;
        let x = self.rx + dx;
        let y = self.ry + dy;
        if self.push(x, y, dx, dy){
            self.rx = x;
            self.ry = y;
        }
    }
    fn push(&mut self, x: i64, y: i64, dx: i64, dy: i64) -> bool{
        if let Some(c) = self.data.get(&(x,y)) {
            match c {
                '#' => {return false;},
                'O' => {
                    let nx = x + dx;
                    let ny = y + dy;
                    if self.push(nx, ny, dx, dy){
                        self.data.remove(&(x,y));
                        self.data.insert((nx,ny), 'O');
                        return true;
                    }
                    return false;
                }
                _ => {panic!("Oops!");}
            }
        }
        else{
            return true;
        }
    }
    pub fn gps(&self){
        let mut total = 0;
        for ((x,y), c) in &self.data{
            if *c != 'O'{continue;}
            total += y * 100 + x;
        }
        println!("{total}");
    }
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let mut in_grid = true;
    let mut y = 0;
    let mut grid = Grid::new();
    let mut commands = String::new();
    for line in txt.lines(){
        if line == ""{
            in_grid = false;
            continue;
        }
        if in_grid{
            let mut x = 0;
            for c in line.chars(){
                if c != '.'{grid.add_tile(x, y, c);}
                x += 1;
            }
            y+=1;
        }
        else{
            commands.push_str(line);
        }
    }
    grid.print();
    // println!("{commands}");
    for c in commands.chars(){
        grid.step(c);
    }
    grid.print();
    grid.gps();
}
