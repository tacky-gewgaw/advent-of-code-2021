pub fn run(input: String) -> (i32, i32) {
    let integers = parse_as_integers(input);

    let increases = count_increases(integers.to_vec());

    let window_increases = count_increases(group_as_windows(integers.to_vec()));

    return (increases, window_increases);
}

fn parse_as_integers(i: String) -> Vec<i32> {
    return i.lines().map(|x| String::from(x).parse::<i32>()).collect::<Result<_, _>>().unwrap();
}

fn count_increases(is: Vec<i32>) -> i32 {
    let mut out = 0;

    let mut previous = is[0];

    for i in is {
        if i > previous {
            out += 1;
        }

        previous = i;
    }

    return out;
}

fn group_as_windows(is: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    for n in 0..is.len()-2 {
        result.push(is[n] + is[n+1] + is[n+2]);
    }

    return result;
}