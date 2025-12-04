
fn parse(txt: &str) -> usize{
    txt.len()
}

#[allow(dead_code)]
pub fn solve(txt: &str) -> String{
    let data = parse(txt);
    format!("{}", data)
}
