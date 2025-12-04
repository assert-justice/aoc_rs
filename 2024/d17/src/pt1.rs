
struct VM{
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
    program: Vec<usize>,
    out: Vec<usize>,
}

#[derive(Clone, Copy)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl VM {
    fn run(&mut self) -> String{
        let codes: [OpCode; 8] = [OpCode::Adv, OpCode::Bxl, OpCode::Bst, OpCode::Jnz, OpCode::Bxc, OpCode::Out, OpCode::Bdv, OpCode::Cdv];
        loop {
            if self.ip >= self.program.len(){break;}
            let op = codes[self.program[self.ip]];
            let literal = self.program[self.ip + 1];
            let combo = match literal {
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => literal,
            };
            // let ip = self.ip;
            self.ip += 2;
            match op {
                OpCode::Adv => {
                    self.a = self.a / 2_usize.pow(combo.try_into().unwrap());
                },
                OpCode::Bxl => {
                    self.b = self.b ^ literal;
                },
                OpCode::Bst => {
                    self.b = combo & 0b111;
                },
                OpCode::Jnz => {
                    if self.a > 0{
                        self.ip = literal;
                    }
                },
                OpCode::Bxc => {
                    self.b = self.b ^ self.c;
                },
                OpCode::Out => {
                    self.out.push(combo & 0b111);
                },
                OpCode::Bdv => {
                    self.b = self.a / 2_usize.pow(combo.try_into().unwrap());
                },
                OpCode::Cdv => {
                    self.c = self.a / 2_usize.pow(combo.try_into().unwrap());
                },
            }
        }
        self.out.iter().map(|c|c.to_string()).collect::<Vec<String>>().join(",")
    }
}

fn parse(txt: &str) -> VM{
    let mut lines = txt.lines();
    let a: usize = lines.next().unwrap().split(' ').last().unwrap().parse().unwrap();
    let program: Vec<usize> = lines.last().unwrap().split(' ').last().unwrap().split(',').map(|c|c.parse().unwrap()).collect();
    let vm = VM{
        a,
        b: 0,
        c: 0,
        ip: 0,
        program,
        out: Vec::new(),
    };
    vm
}

#[allow(dead_code)]
pub fn solve(txt: &str) -> String{
    let mut data = parse(txt);
    format!("{}", data.run())
}
