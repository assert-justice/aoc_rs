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
        let did_push;
        if dy == 0{
            did_push = self.push_h(x, y, dx);
        }
        else{
            did_push = self.push_v(x, y, dy);
        }
        if did_push{
            self.rx = x;
            self.ry = y;
        }
    }
    fn can_push_v(&mut self, x: i64, y: i64, dy: i64) -> bool{
        if let Some(c) = self.data.get(&(x,y)) {
            match c {
                '#' => {return false;},
                '[' => {
                    // DON'T try pushing partner, just check if that space can be open
                    let nx = x;
                    let ny = y + dy;
                    if self.can_push_v(nx, ny, dy) && self.can_push_v(nx+1, ny, dy){
                        return true;
                    }
                    return false;
                }
                ']' => {
                    // DON'T try pushing partner, just check if that space can be open
                    let nx = x;
                    let ny = y + dy;
                    if self.can_push_v(nx, ny, dy) && self.can_push_v(nx-1, ny, dy){
                        return true;
                    }
                    return false;
                }
                _ => {panic!("Oops! Char: {}", c);}
            }
        }
        else{
            return true;
        }
    }
    fn push_v(&mut self, x: i64, y: i64, dy: i64) -> bool{
        if !self.can_push_v(x, y, dy){
            return false;
        }
        if let Some(c) = self.data.get(&(x,y)) {
            match c {
                '[' => {
                    let nx = x;
                    let ny = y + dy;
                    self.push_v(nx, ny, dy);
                    self.push_v(nx+1, ny, dy);
                    self.data.remove(&(x,y));
                    self.data.remove(&(x+1,y));
                    self.data.insert((nx,ny), '[');
                    self.data.insert((nx+1,ny), ']');
                }
                ']' => {
                    let nx = x;
                    let ny = y + dy;
                    self.push_v(nx, ny, dy);
                    self.push_v(nx-1, ny, dy);
                    self.data.remove(&(x,y));
                    self.data.remove(&(x-1,y));
                    self.data.insert((nx,ny), ']');
                    self.data.insert((nx-1,ny), '[');
                }
                _ => {panic!("Oops!");}
            }
        }
        true
    }
    fn push_h(&mut self, x: i64, y: i64, dx: i64) -> bool{
        if let Some(c) = self.data.get(&(x,y)) {
            match c {
                '#' => {return false;},
                '[' => {
                    let nx = x + dx;
                    let ny = y;
                    if self.push_h(nx, ny, dx){
                        self.data.remove(&(x,y));
                        self.data.insert((nx,ny), '[');
                        return true;
                    }
                    return false;
                }
                ']' => {
                    let nx = x + dx;
                    let ny = y;
                    if self.push_h(nx, ny, dx){
                        self.data.remove(&(x,y));
                        self.data.insert((nx,ny), ']');
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
            if *c != '['{continue;}
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
                match c {
                    '#' => {grid.add_tile(x, y, c);grid.add_tile(x+1, y, c);}
                    'O' => {grid.add_tile(x, y, '[');grid.add_tile(x+1, y, ']');}
                    '@' => {grid.rx = x; grid.ry = y;}
                    _ => {},
                }
                x += 2;
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
        // grid.print();
    }
    grid.print();
    grid.gps();
}
