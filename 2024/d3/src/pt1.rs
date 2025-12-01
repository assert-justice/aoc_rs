
#[derive(Debug)]
enum Token {
    Mul(u32, u32),
}

struct Parser{
    cp: usize,
    chars: Vec<char>,
    tokens: Vec<Token>,
}

impl Parser{
    fn new(txt: &str) -> Self{
        Self { 
            cp: 0, 
            chars: txt.chars().collect(),
            tokens: Vec::new(),
        }
    }
    fn peek(&self) -> char{
        self.chars[self.cp]
    }
    fn advance(&mut self){
        self.cp += 1;
    }
    fn at_eof(&self) -> bool{
        self.cp >= self.chars.len()
    }
    fn match_str(&mut self, s: &str) -> bool{
        let start = self.cp;
        for c in s.chars() {
            if self.at_eof() || c != self.peek(){
                self.cp = start;
                return false;
            }
            self.advance();
        }
        true
    }
    fn match_num(&mut self) -> Option<u32>{
        let mut cs: Vec<char> = Vec::new();
        while !self.at_eof() {
            if !self.peek().is_numeric(){
                break;
            }
            cs.push(self.peek());
            self.advance();
        }
        if cs.len() == 0{
            return None;
        }
        let cs: String = cs.iter().collect();
        Some(cs.parse().unwrap())
    }
    fn parse(txt: &str) -> Vec<Token>{
        let mut parser = Parser::new(txt);
        while !parser.at_eof() {
            if !parser.match_str("mul("){
                parser.advance();
                continue;
            }
            let a = match parser.match_num() {
                None => {
                    continue;
                },
                Some(n) => n
            };
            if !parser.match_str(","){
                continue;
            }
            let b = match parser.match_num() {
                None => {
                    continue;
                },
                Some(n) => n
            };
            if !parser.match_str(")"){
                continue;
            }
            parser.tokens.push(Token::Mul(a, b));
        }
        parser.tokens
    }
}

#[allow(dead_code)]
pub fn solve(txt: &str){
    let tokens = Parser::parse(txt);
    let mut res = 0;
    for t in tokens {
        match t {
            Token::Mul(a, b)=>{res += a * b;}
        }
    }
    println!("{res}");
}
