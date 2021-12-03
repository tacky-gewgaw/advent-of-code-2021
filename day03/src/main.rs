fn main() {
    let input = read_input(include_str!("input.txt"));
    
    let gamma_rate = gamma_rate_from(input.to_vec());

    let epsilon_rate = epsilon_rate_from(input.to_vec());

    let result = bin_to_dec(gamma_rate.to_vec()) * bin_to_dec(epsilon_rate.to_vec());

    println!("{}", result);

    let csr = find_co2_scrub_rating(input.to_vec());

    let ogr = find_ox_gen_rating(input.to_vec());

    let lsr = bin_to_dec(csr.to_vec()) * bin_to_dec(ogr.to_vec());

    println!("{:?}", lsr)
}

fn bin_to_dec(b: Vec<u8>) -> i32 {
    let s = b.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("");

    return isize::from_str_radix(&s, 2).unwrap() as i32;
}

fn read_input(file: &str) -> Vec<Vec<u8>> {
    return file.lines()
            .map(|s| s.split("")
                        .filter_map(|c| String::from(c).parse::<u8>().ok())
                        .collect())
            .collect()
}

fn gamma_rate_from(baa: Vec<Vec<u8>>) -> Vec<u8> {
    let threshold: i32 = (baa.len() as f32 / 2.0).floor() as i32;
    let mut result_vec: Vec<i32> = Vec::new();

    for _i in 0..baa[0].len() {
        result_vec.push(0)
    }

    baa.iter().for_each(|ba| ba.iter().enumerate().for_each(|(i, v)| result_vec[i] += *v as i32));

    let q = result_vec.iter().map(|i| (i > &threshold) as u8).collect::<Vec<u8>>();

    println!("gr: {}", q.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));

    return q
}

fn epsilon_rate_from(baa: Vec<Vec<u8>>) -> Vec<u8> {
    let threshold: i32 = (baa.len() as f32 / 2.0).floor() as i32;
    let mut result_vec: Vec<i32> = Vec::new();

    for _i in 0..baa[0].len() {
        result_vec.push(0)
    }

    baa.iter().for_each(|ba| ba.iter().enumerate().for_each(|(i, v)| result_vec[i] += *v as i32));

    return result_vec.iter().map(|i| !(i > &threshold) as u8).collect::<Vec<u8>>();
}

fn find_ox_gen_rating(baa: Vec<Vec<u8>>) -> Vec<u8> {
    let mut res: Vec<Vec<u8>> = baa.to_vec();

    let gr = gamma_rate_from(baa.to_vec()).to_vec();

    for i in 0..baa[0].len() {
        let res1: Vec<Vec<u8>> = res.iter().filter(
            |ba| ba[i] == gr[i]
        ).cloned().collect();

        if res1.len() == 1 {
            return res1[0].to_vec();
        }

        res = res1
    }

    return [].to_vec();
}

fn find_co2_scrub_rating(baa: Vec<Vec<u8>>) -> Vec<u8> {
    let mut res: Vec<Vec<u8>> = baa.to_vec();

    let gr = epsilon_rate_from(baa.to_vec()).to_vec();

    for i in 0..baa[0].len() {
        let res1: Vec<Vec<u8>> = res.iter().filter(
            |ba| ba[i] == gr[i]
        ).cloned().collect();

        if res1.len() == 1 {
            return res1[0].to_vec();
        }

        res = res1
    }

    return [].to_vec();
}