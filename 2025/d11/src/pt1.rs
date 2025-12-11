use std::collections::HashMap;


struct Device{
    name: String,
    outputs: Vec<String>,
    num_routes: Option<usize>,
}

impl Device {
    fn parse(line: &str) -> Self{
        let line: Vec<&str> = line.split(": ").collect();
        let name = line[0].to_string();
        let outputs = line[1].split(' ').map(|s|s.to_string()).collect();
        Self { name, outputs, num_routes: None }
    }
}

struct Server {
    devices: HashMap<String, Device>,
}

impl Server{
    fn new() -> Self{
        Self { devices: HashMap::new() }
    }
    fn add_device(&mut self, line: &str){
        let d = Device::parse(line);
        self.devices.insert(d.name.clone(), d);
    }
    fn count_routes(&self, start: &str) -> usize{
        let d = self.devices.get_mut(start).unwrap();
        if let Some(num_routes) = d.num_routes {
            return num_routes;
        }
        let mut rs = Vec::new();
        for o in &d.outputs {
            rs.push(self.count_routes(&o));
        }
        let res = rs.iter().min();
        // let rs = d.outputs.iter().map(|o|self.count_routes(&o)).min();
        // if let Some(res) = rs {
        //     d.num_routes = Some(res);
        //     return res;
        // }
        return 0;
    }
}

fn parse(txt: &str) -> Server{
    let mut server = Server::new();
    for line in txt.lines() {
        server.add_device(line);
    }
    server
}

#[allow(dead_code)]
pub fn solve(txt: &str) -> String{
    let data = parse(txt);
    format!("{}", data)
}
