pub fn main() {
    let (depth, hor) = include_str!("input.txt").lines()
            .map(|s| s.split(" ").collect())
            .map(|v: Vec<&str>| (String::from(v[0]), v[1].parse::<i32>().unwrap()))
            .fold((0,0), |pos, com| process_command(pos, com));
    println!("{}", depth * hor);

    let (depth, hor, _) = include_str!("input.txt").lines()
            .map(|s| s.split(" ").collect())
            .map(|v: Vec<&str>| (String::from(v[0]), v[1].parse::<i32>().unwrap()))
            .fold((0,0,0), |pos, com| process_command_using_aim(pos, com));
    println!("{}", depth * hor);
}

fn process_command((depth,hor): (i32, i32), (direction, value): (String, i32)) -> (i32, i32) {
    
    if direction == "forward" {
        return (depth, hor + value);
    }
    
    if direction == "up" {
        return (depth - value, hor);
    }
    
    if direction == "down" {
        return (depth + value, hor);
    }

    return (depth, hor);
}

fn process_command_using_aim((depth, hor, aim): (i32, i32, i32), (direction, value): (String, i32)) -> (i32, i32, i32) {

    if direction == "forward" {
        return (depth + (aim*value), hor + value, aim);
    }
    if direction == "up" {
        return (depth, hor, aim - value);
    }
    if direction == "down" {
        return (depth, hor, aim + value)
    }

    return (depth, hor, aim);
}