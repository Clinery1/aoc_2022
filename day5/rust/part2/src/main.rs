use std::fs::read_to_string;


// [N]             [R]             [C]
// [T] [J]         [S] [J]         [N]
// [B] [Z]     [H] [M] [Z]         [D]
// [S] [P]     [G] [L] [H] [Z]     [T]
// [Q] [D]     [F] [D] [V] [L] [S] [M]
// [H] [F] [V] [J] [C] [W] [P] [W] [L]
// [G] [S] [H] [Z] [Z] [T] [F] [V] [H]
// [R] [H] [Z] [M] [T] [M] [T] [Q] [W]
//  1   2   3   4   5   6   7   8   9 

const STARTING_CRATES:[&[char];9]=[
    &['R','G','H','Q','S','B','T','N'],
    &['H','S','F','D','P','Z','J'],
    &['Z','H','V'],
    &['M','Z','J','F','G','H'],
    &['T','Z','C','D','L','M','S','R'],
    &['M','T','W','V','H','Z','J'],
    &['T','F','P','L','Z'],
    &['Q','V','W','S'],
    &['W','H','L','M','T','D','N','C'],
];


fn main() {
    let input=read_to_string("../../input").unwrap();
    let mut crates=Vec::new();
    for row in STARTING_CRATES {
        crates.push(row.to_vec());
    }
    for line in input.lines() {
        let sections=line.split(' ').collect::<Vec<_>>();
        let count=sections[1].parse::<usize>().unwrap();
        let from=sections[3].parse::<usize>().unwrap()-1;
        let to=sections[5].parse::<usize>().unwrap()-1;
        let mut tmp=Vec::new();
        for _ in 0..count {
            tmp.push(crates[from].pop().unwrap());
        }
        while let Some(c)=tmp.pop() {
            crates[to].push(c);
        }
    }
    let mut msg=String::new();
    for mut stack in crates {
        msg.push(stack.pop().unwrap());
    }
    println!("The top crates are: {}",msg);
}
