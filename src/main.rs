use std::cmp;
use std::time::Duration;
use std::thread;
use std::env;
use std::io;

struct Tower(Vec<u8>);

impl Tower {
    fn new() -> Tower {
        Tower(Vec::new())
    }

    fn place(&mut self, slab: u8) {
        let &mut Tower(ref mut vec) = self;
        vec.push(slab);
    }

    fn remove(&mut self) {
        let &mut Tower(ref mut vec) = self;
        vec.pop();
    }

    fn len(&self) -> usize {
        let &Tower(ref vec) = self;
        vec.len()
    }
    
    fn get(&self, index: usize) -> Option<&u8> {
        let &Tower(ref vec) = self;
        vec.get(index)
    }
}

struct GameState {
    towers: Vec<Tower>,
    wait: bool,
}

impl GameState {

    fn do_move(&mut self, slab: u8, src: u8, dest: u8) {
        self.towers[src as usize].remove();
        self.towers[dest as usize].place(slab);
        self.render();
    }

    fn render(&mut self) {
        let wait = self.wait;
        print!("{}[2J", 27 as char);

        let height = self.towers.iter().fold(0,
            |acc, tower| acc + tower.len());

        for layer in (0..height+1).rev() {
            let slabs: Vec<String> = self.towers.iter().map(
                |tower| match tower.get(layer) {
                    Some(&slab) => {
                        let buf_string = (0..(((height*2) as u8 - slab*2) / 2))
                            .fold(String::new(), |acc, _| acc + " ");
                        let mut slab_string = format!("{}{}",buf_string,"(");
                        let fill_str = if slab < 10 {slab.to_string()} else {String::from("$")};
                        for _ in 0..(slab*2)-1 {
                            slab_string = format!("{}{}",slab_string,fill_str);
                        }
                        format!("{}{}{}",slab_string,")",buf_string)
                    },
                    None => {
                        let buf_string = (0..height)
                            .fold(String::new(), |acc, _| format!("{}{}",acc," "));
                        format!("{}{}{}",buf_string,"|",buf_string)
                    },
                }).collect();

            println!("{}   {}   {}", slabs[0], slabs[1], slabs[2]);
        }
        let width = (height*2+1)*3 + 6;
        let barrier = (0..width)
            .fold(String::from(""), |acc, _| acc + "#");
        println!("{}", barrier);

        if wait {
            println!("Press Enter to execute next move");
            let mut string_in = String::new();
            io::stdin().read_line(&mut string_in).expect("failed to read line");
        } else {
            thread::sleep(Duration::from_millis(cmp::max(2000/height as u64, 30)));
        }
    }

}

fn main() {

    let mut args = env::args();
    args.next();

    let n: u8 = match args.next() {
        Some(arg)   => arg.parse()
            .expect("First argument should be a positive integer number"),
        None        => 5u8,
    };

    let wait: bool = match args.next() {
        Some(arg)   => arg == "--wait",
        None        => false,
    };

    let mut tower1 = Tower::new();
    for i in (1..n+1).rev() {
        tower1.place(i);
    }

    let mut state = GameState {
        towers: vec![tower1, Tower::new(), Tower::new()],
        wait,
    };

    state.render();
    toh(n,0,2,1,&mut state);
}

fn toh(slab: u8, src: u8, dest: u8, aux: u8, state: &mut GameState) {
    if slab == 1 {
        state.do_move(slab, src, dest);
        return;
    }
    toh(slab-1, src, aux, dest, state);
    state.do_move(slab, src, dest);
    toh(slab-1, aux,dest, src, state);
}


