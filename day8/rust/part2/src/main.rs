use std::{
    ops::{
        Index,
        IndexMut,
    },
    fs::read_to_string,
};
use Direction::*;


// Visible inner trees in test input:
// - 1,1
// - 2,1
// - 1,2
// - 3,2
// - 2,3


#[derive(Copy,Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}


#[derive(Copy,Clone,Debug)]
struct Cursor {
    x:usize,
    y:usize,
}
impl  Cursor {
    /// Returns true if the cursor cannot move any farther
    fn inc(&mut self,dir:Direction,max:usize)->bool {
        match dir {
            Up=>{
                if self.y==0 {return true}
                self.y-=1;
            },
            Down=>{
                if self.y==max-1 {return true}
                self.y+=1;
            },
            Left=>{
                if self.x==0 {return true}
                self.x-=1;
            },
            Right=>{
                if self.x==max-1 {return true}
                self.x+=1;
            },
        }
        return false;
    }
}
impl<T> Index<Cursor> for Vec<Vec<T>> {
    type Output=T;
    fn index(&self,c:Cursor)->&T {
        self.index(c.y).index(c.x)
    }
}
impl<T> IndexMut<Cursor> for Vec<Vec<T>> {
    fn index_mut(&mut self,c:Cursor)->&mut T {
        self.index_mut(c.y).index_mut(c.x)
    }
}


fn main() {
    let raw_input=read_to_string("../../input").unwrap();
//     let raw_input="30373
// 25512
// 65332
// 33549
// 35390";
    let input=raw_input
        .lines()
        .map(|line|line
            .chars()
            .map(|c|format!("{}",c).parse::<u8>().unwrap())
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    let index_max=input.len();
    let mut scenic_scores=input
        .iter()
        .map(|row|row
            .iter()
            .map(|tree|*tree as u64)
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    for y in 0..input.len() {
        for x in 0..input.len() {
            let cursor=Cursor{x,y};
            let current=input[cursor];
            let scenic_score=[Up,Down,Left,Right]
                .into_iter()
                .map(|dir|{
                    let mut alt_cursor=cursor.clone();
                    let mut tree_count=0;
                    loop {
                        if alt_cursor.inc(dir,index_max) {
                            break;
                        }
                        tree_count+=1;
                        if input[alt_cursor]>=current {
                            break;
                        }
                    }
                    return tree_count;
                })
                .reduce(|a,b|a*b).unwrap();
            scenic_scores[cursor]=scenic_score;
        }
    }
    println!("Highest scenic score: {}",scenic_scores
        .iter()
        .map(|row|row
            .iter()
            .max()
            .unwrap()
        )
        .max()
        .unwrap()
    );
}
