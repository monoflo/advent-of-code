use std::collections::HashSet;
use std::io::Read;
use std::str::FromStr;

fn split_at_mid(input: &str) -> (&str, &str) {
    let mid = input.len() / 2;
    input.split_at(mid)
}

type Item = char;
type ItemSet = HashSet<Item>;

/// Computes the priority for the given `Item`.
fn priority(item: Item) -> anyhow::Result<usize> {
    static ASCII_A_LOWER: usize = 'a' as usize;
    static ASCII_A_UPPER: usize = 'A' as usize;
    let ascii = item as usize;
    match item {
        'a'..='z' => Ok(ascii - ASCII_A_LOWER + 1),
        'A'..='Z' => Ok(ascii - ASCII_A_UPPER + 27),
        _ => anyhow::bail!("Invalid item encountered"),
    }
}

/// Represents a rucksack containing supplies.
struct Rucksack {
    /// An array of sets representing rucksack compartments; contains each unique item in the compartment.
    compartment: [ItemSet; 2],
}

impl Rucksack {
    /// Retrieves the common item between the two compartments, of which there is assumed to be
    /// only one.
    fn common_item(&self) -> anyhow::Result<Item> {
        // Retrieve an iterator of the items common to both compartments.
        let mut iter = self.compartment[0].intersection(&self.compartment[1]);

        // Expect the first item in the iterator.
        let item = iter.next().unwrap();

        // Check that there are no other common items between the rucksacks.
        assert_eq!(0, iter.count());

        Ok(*item)
    }
}

impl FromStr for Rucksack {
    type Err = &'static str;

    /// Attempt to parse `Rucksack` from a string.
    fn from_str(contents: &str) -> Result<Self, Self::Err> {
        let (mut c1, mut c2) = (ItemSet::new(), ItemSet::new());
        let (lhs, rhs) = split_at_mid(contents);
        lhs.chars().for_each(|chr| {
            c1.insert(chr);
        });
        rhs.chars().for_each(|chr| {
            c2.insert(chr);
        });
        Ok(Rucksack {
            compartment: [c1, c2],
        })
    }
}

/// Read, return data from the input file.
fn read_input() -> anyhow::Result<Vec<Rucksack>> {
    let path = std::path::PathBuf::from("input");
    let mut file = std::fs::File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let rucksacks: Vec<Rucksack> = data
        .lines()
        .map(|line| Rucksack::from_str(line).unwrap())
        .collect();
    Ok(rucksacks)
}

/// Compute the sum of priorities of all the items common to both compartments of each rucksack.
fn part_one(data: &[Rucksack]) -> anyhow::Result<usize> {
    Ok(data
        .iter()
        .map(|sack| sack.common_item().unwrap())
        .map(|item| priority(item).unwrap())
        .reduce(|sum, prio| sum + prio)
        .unwrap())
}

fn main() -> anyhow::Result<()> {
    let data = read_input()?;
    println!(
        "Part One - Sum of priorities of common items: {}",
        part_one(&data)?
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_at_mid_valid() {
        let input = String::from("Hello World!");
        let (lhs, rhs) = split_at_mid(&input);
        assert_eq!(lhs, "Hello ");
        assert_eq!(rhs, "World!");
    }

    #[test]
    fn priority_valid() -> anyhow::Result<()> {
        assert_eq!(1, priority('a')?);
        assert_eq!(4, priority('d')?);
        assert_eq!(26, priority('z')?);
        assert_eq!(27, priority('A')?);
        assert_eq!(30, priority('D')?);
        assert_eq!(52, priority('Z')?);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn priority_invalid() {
        priority('?').unwrap();
    }
}
