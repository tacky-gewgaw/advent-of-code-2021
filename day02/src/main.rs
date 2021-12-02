pub fn main() {
    let (depth, hor) = include_str!("input.txt").lines()
            .map(|s| s.split(" ").collect())
            .map(|v: Vec<&str>| (String::from(v[0]), v[1].parse::<i32>().unwrap()))
            .fold((0,0), |pos, com| process_command(pos, com));
    println!("{}", depth * hor);
}

fn process_command((depth,hor): (i32, i32), (direction, value): (String, i32)) -> (i32, i32) {
    let mut depth1 = depth;
    let mut hor1 = hor;

    if direction == "forward" {
        hor1 += value;
    } else if direction == "up" {
        depth1 -= value;
    } else if direction == "down" {
        depth1 += value;
    }

    return (depth1, hor1);
}