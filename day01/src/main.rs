use itertools::Itertools;

pub fn main() {

    let integers: Vec<i32> = include_str!("input.txt").lines().map(|i| i.parse::<i32>().unwrap()).collect();
    
    println!("{}", count_increases(integers.to_owned()));

    println!("{:?}", count_increases(group_as_windows(integers.to_owned())));
}

fn count_increases(is: Vec<i32>) -> i32 {
    return is.iter().tuple_windows().filter(|(x,y)| y > x).count() as i32

    // let mut result = 0;

    // for i in 0..is.len()-1 {
    //     if is[i] < is[i+1] {
    //         result += 1;
    //     }
    // }

    // return result;
}

fn group_as_windows(is: Vec<i32>) -> Vec<i32> {
    return is.iter().tuple_windows().map(|(x,y,z)| x+y+z).collect();
    // println!("{:?}", a);
    // let mut result = Vec::new();

    // for n in 0..is.len()-2 {
    //     result.push(is[n] + is[n+1] + is[n+2]);
    // }

    // return result;
}
