use std::io::Read;

type Range = (usize, usize);

fn read_input() -> anyhow::Result<String> {
    let path = std::path::PathBuf::from("input");
    let mut file = std::fs::File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

fn parse_input(input: String) -> anyhow::Result<Vec<(Range, Range)>> {
    Ok(input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(elf1, elf2)| (split_range(elf1), split_range(elf2)))
        .collect())
}

fn split_range(input: &str) -> Range {
    input
        .split_once('-')
        .map(|(start, stop)| {
            (
                start.parse::<usize>().unwrap(),
                stop.parse::<usize>().unwrap(),
            )
        })
        .unwrap()
}

fn part1(data: &Vec<(Range, Range)>) -> usize {
    let mut total: usize = 0;
    for (elf1, elf2) in data {
        total += ((elf1.0 >= elf2.0 && elf1.1 <= elf2.1) || (elf2.0 >= elf1.0 && elf2.1 <= elf1.1))
            as usize;
    }
    total
}

fn part2(data: &Vec<(Range, Range)>) -> usize {
    let mut total: usize = 0;
    for (elf1, elf2) in data {
        total += (elf1.0.max(elf2.0) <= elf1.1.min(elf2.1)) as usize;
    }
    total
}

fn main() -> anyhow::Result<()> {
    let data = parse_input(read_input()?)?;
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
    Ok(())
}
