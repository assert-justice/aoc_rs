use std::collections::HashMap;

struct Puzzle{
    data: HashMap<usize, usize>,
    size: usize,
    start: usize,
}

// impl Display for Puzzle {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, )
//     }
// }

impl Puzzle {
    pub fn new(text: &str) -> Self{
        let chars = text.bytes().collect::<Vec<_>>();
        let mut block_id = 0;
        let mut pointer = 0;
        let mut puzzle = Puzzle{
            data: HashMap::new(),
            size: 0,
            start: 0,
        };
        for idx in 0..chars.len(){
            let is_file = idx % 2 == 0;
            let size = chars[idx] - 48;
            if is_file{
                for _ in 0..size{
                    puzzle.data.insert(pointer, block_id);
                    pointer += 1;
                }
                block_id += 1;
            }
            else{
                pointer += usize::from(size);
            }
        }
        puzzle.size = pointer;
        puzzle
    }
    fn find_free(&mut self) -> usize{
        while let Some(_) = self.data.get(&self.start) {
            self.start += 1;
        }
        self.start
    }
    fn shrink(&mut self) -> usize{
        while let None = self.data.get(&self.size) {
            self.size -= 1;
        }
        // self.size += 1;
        self.size
    }
    pub fn compact(&mut self){
        while self.find_free() < self.shrink(){
            // get value at size
            let val = self.data.remove(&self.size).unwrap();
            self.data.insert(self.start, val);
        }
        self.size+=1;
    }
    pub fn print(&self){
        for idx in 0..self.size{
            if let Some(v) = self.data.get(&idx) {
                println!("{v}");
            }
        }
    }
    pub fn checksum(&self) -> usize{
        let mut total = 0;
        for (idx, id) in self.data.clone(){
            total += idx * id;
        }
        total
    }
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    for line in txt.lines(){
        let mut puzzle = Puzzle::new(line);
        println!("{}", puzzle.size);
        puzzle.compact();
        println!("{}", puzzle.size);
        // println!("{}", "0099811188827773336446555566".len());
        puzzle.print();
        println!("{}", puzzle.checksum());
        break;
    }
}