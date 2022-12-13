use std::fs::read_to_string;


fn main() {
    let raw_input=read_to_string("../../input").unwrap();
    let raw_input="30373
25512
65332
33549
35390";
    let input=raw_input
        .lines()
        .map(|line|line
            .chars()
            .map(|c|c as u8-('0' as u8))
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    let mut visible_count=(input.len()*2)+((input[0].len()-2)*2);
    for y in 1..input.len()-1 {
        for x in 1..input[y].len()-1 {
            let max=input[y][x];
            let mut visible=false;
            'dirs:for dir in 0..4 {
                let mut alt_x=x;
                let mut alt_y=y;
                let mut max_tree=0;
                while alt_x<input[alt_y].len()-1&&alt_x>0&&y<input.len()-1&&y>0 {
                    if input[alt_y][alt_x]>max_tree {
                        if alt_x!=x&&alt_y!=y {
                            max_tree=input[alt_y][alt_x];
                        }
                    }
                    match dir {
                        0=>alt_x-=1,
                        1=>alt_x+=1,
                        2=>alt_y-=1,
                        3=>alt_y+=1,
                        _=>{},
                    }
                }
                if max_tree<max {
                    visible=true;
                    break 'dirs;
                }
            }
            if visible {
                visible_count+=1;
            }
        }
    }
    println!("Amount of trees visible from the edge: {}",visible_count);
}
