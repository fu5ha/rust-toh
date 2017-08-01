use std::cmp;
use std::time::Duration;
use std::thread;
use std::env;

fn main() {

    let mut args = env::args();
    args.next();
    let n: u16 = match args.next() {
        Some(arg)  => arg.parse()
            .expect("First argument should be a positive integer number"),
        None        => 5u16,
    };
    toh_solve(n);
}

fn toh_solve(num_slabs: u16) {

    let mut tower1: Vec<u16> = Vec::new();
    for i in (1..num_slabs+1).rev() {
        tower1.push(i);
    }
    let mut tower2: Vec<u16> = Vec::new();
    let mut tower3: Vec<u16> = Vec::new();
    let mut state = vec![&mut tower1, &mut tower2, &mut tower3];
    render(&mut state);
    toh(num_slabs,0,2,1,&mut state);
}

fn toh(slab: u16, src: u16, dest: u16, aux: u16, state: &mut Vec<&mut Vec<u16>>) {
    if slab == 1 {
        do_move(slab, src, dest, state);
        return;
    }
    toh(slab-1, src, aux, dest, state);
    do_move(slab, src, dest, state);
    toh(slab-1, aux,dest, src, state);
}

fn do_move(slab: u16, src: u16, dest: u16, state: &mut Vec<&mut Vec<u16>>) {
    state[src as usize].pop();
    state[dest as usize].push(slab);
    render(state);
}

fn render(state: &mut Vec<&mut Vec<u16>>) {
    print!("{}[2J", 27 as char);

    let height = state.iter().fold(0,
        |acc, tower| acc + tower.len());

    for layer in (0..height+1).rev() {
        let slabs: Vec<String> = state.iter().map(
            |tower| match tower.get(layer as usize) {
                Some(&slab) => {
                    let buf_string = (0..(((height*2) as u16 - slab*2) / 2))
                        .fold(String::new(), |acc, _| format!("{}{}",acc," "));
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
    let table = (0..width)
        .fold(String::from(""), |acc, _| format!("{}{}",acc,"#"));
    println!("{}", table);

    thread::sleep(Duration::from_millis(cmp::max(2000/height as u64, 20)));
}
