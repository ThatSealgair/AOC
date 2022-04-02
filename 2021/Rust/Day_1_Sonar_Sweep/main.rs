// You're minding your own business on a ship at sea when the overboard alarm goes off! 
//You rush to see if you can help. Apparently, one of the Elves tripped and accidentally sent the sleigh keys flying into the ocean!
// Before you know it, you're inside a submarine the Elves keep ready for situations like this. 
// It's covered in Christmas lights (because of course it is), and it even has an experimental antenna that should be able to track the keys if you can boost its signal strength high enough; there's a little meter that indicates the antenna's signal strength by displaying 0-50 stars.
// Your instincts tell you that in order to save Christmas, you'll need to get all fifty stars by December 25th.
// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!
// As the submarine drops below the surface of the ocean, it automatically performs a sonar sweep of the nearby sea floor. On a small screen, the sonar sweep report (your puzzle input) appears: each line is a measurement of the sea floor depth as the sweep looks further and further away from the submarine.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Read;
use std::iter::Sum;


fn read_data() -> Vec<i64> {
    let mut file = File::open("input.txt").expect("File not found");
    let br = BufReader::new(file);
    let mut vect = Vec::new();

    for line in br.lines() {
        let line = line.unwrap();
        vect.push(line.parse::<i64>().unwrap());
    }
    vect
}


fn positive_changes(depth: &Vec<i64>) -> i64 {
    let mut count = 0;
    let mut previous = depth[0];
    for depth in depth.iter() {
        if previous < *depth {
            count += 1;
        }
        previous = *depth;
    }
    count
}


fn  sliding_window(depth: &Vec<i64>) -> i64 {
        let mut count: i64 = 0;
        let mut prev: Option<i64> = None;
        let mut curr: i64;
        for i in depth.windows(3) {
            curr =  i.iter().sum::<i64>();
            if let Some(p) = prev {
                if curr > p {
                    count += 1;
                }
            }
            prev = Some(curr);
        }
        count
}



fn main() {
    println!("{}", positive_changes(&read_data()));
    println!("{}", sliding_window(&read_data()));
}
