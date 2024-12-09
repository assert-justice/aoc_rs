use std::collections::HashMap;

struct Puzzle{
    data: HashMap<usize, usize>,
}

struct Block{
    id: usize,
    start: usize,
    len: usize,
}

struct Gap{
    start: usize,
    len: usize,
}

impl Puzzle {
    pub fn new(text: &str) -> Self{
        let chars = text.bytes().collect::<Vec<_>>();
        let mut block_id = 0;
        let mut pointer = 0;
        let mut puzzle = Puzzle{
            data: HashMap::new(),
        };
        let mut blocks = Vec::new();
        let mut gaps = Vec::new();
        for idx in 0..chars.len(){
            let is_file = idx % 2 == 0;
            let size = chars[idx] - 48;
            if is_file{
                blocks.push(Block{
                    id: block_id,
                    start: pointer,
                    len: usize::from(size),
                });
                block_id += 1;
            }
            else{
                gaps.push(Gap{
                    start: pointer,
                    len: usize::from(size),
                });
            }
            pointer += usize::from(size);
        }
        while let Some(mut block) = blocks.pop() {
            for gap in &mut gaps[..]{
                if gap.start > block.start{break;}
                if gap.len < block.len{continue;}
                block.start = gap.start;
                gap.start += block.len;
                gap.len -= block.len;
            }
            for i in 0..block.len{
                puzzle.data.insert(i + block.start, block.id);
            }
        }
        puzzle
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
        let puzzle = Puzzle::new(line);
        println!("{}", puzzle.checksum());
        break;
    }
}