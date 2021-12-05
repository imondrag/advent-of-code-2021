const WIDTH: usize = 12;
fn main() {
    let nums: Vec<_> = include_str!("../input.txt")
        .lines()
        .map(|s| usize::from_str_radix(s, 2).unwrap())
        .collect();

    let gamma: u32 = nums
        .iter()
        .fold(vec![0; WIDTH], |counts, n| {
            let bitstream = (0..WIDTH).map(|i| (n & 1 << i) >> i);
            counts
                .into_iter()
                .zip(bitstream)
                .map(|(lhs, rhs)| lhs + rhs)
                .collect()
        })
        .into_iter()
        .enumerate()
        .map(|(i, count)| ((count >= nums.len() / 2) as u32) << i)
        .sum();

    let epsilon = !gamma & 0xFFF;

    println!("Day 3.1: {}", gamma * epsilon);

    let mut oxy_nums = nums.clone();
    let mut co_nums = nums.clone();
    for i in (0..WIDTH).rev() {
        let get_bit = |n| (n & 1 << i) >> i;
        let get_common_bit =
            |arr: &[usize]| arr.iter().filter(|&&n| (get_bit(n)) > 0).count() >= arr.len() / 2;

        let do_common = oxy_nums.len() != 1;
        let do_least_common = co_nums.len() != 1;

        if do_common {
            let common_bit = get_common_bit(&oxy_nums);
            oxy_nums.retain(|&n| get_bit(n) == common_bit as usize);
        }

        if do_least_common {
            let common_bit = get_common_bit(&co_nums);
            co_nums.retain(|&n| get_bit(n) != common_bit as usize);
        }
    }

    // dbg!(&oxy_nums);
    // dbg!(&co_nums);
    let oxygen = oxy_nums[0];
    let co2 = co_nums[0];

    println!("Day 3.2: {}", oxygen * co2);
}
