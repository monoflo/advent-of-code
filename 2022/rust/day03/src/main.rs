use std::collections::HashSet;
use std::io::Read;

type Item = char;
type ItemSet = HashSet<Item>;

/// Computes the priority for the given `Item`.
fn priority(item: &Item) -> anyhow::Result<usize> {
    static ASCII_A_LOWER: usize = 'a' as usize;
    static ASCII_A_UPPER: usize = 'A' as usize;
    let ascii = *item as usize;
    match item {
        'a'..='z' => Ok(ascii - ASCII_A_LOWER + 1),
        'A'..='Z' => Ok(ascii - ASCII_A_UPPER + 27),
        _ => anyhow::bail!("Invalid item encountered"),
    }
}

/// Read, return data from the input file.
fn read_input() -> anyhow::Result<String> {
    let path = std::path::PathBuf::from("input");
    let mut file = std::fs::File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

/// Compute the sum of priorities of all the items common to both compartments of each rucksack.
fn part_one(data: &str) -> anyhow::Result<usize> {
    let mut sum: usize = 0;

    for line in data.lines() {
        // create sets for both compartments
        let (mut lset, mut rset) = (ItemSet::new(), ItemSet::new());

        // split line in half at the midpoint
        let (lstr, rstr) = line.split_at(line.len() / 2);

        // insert items from each compartment into respective sets
        lstr.chars().for_each(|c| {
            lset.insert(c);
        });
        rstr.chars().for_each(|c| {
            rset.insert(c);
        });

        // determine items common to both compartments
        let mut itcommon = lset.intersection(&rset);

        // expect the first item in the iterator and no more
        let item = itcommon.next().unwrap();
        assert_eq!(0, itcommon.count());

        // determine priority of common item and add to sum
        sum += priority(item)?;
    }

    Ok(sum)
}

fn part_two(data: &str) -> anyhow::Result<usize> {
    const GROUP_SIZE: usize = 3;

    let mut sum: usize = 0;
    let mut elfidx: usize = 0;
    let mut elfgroup: [ItemSet; GROUP_SIZE] = Default::default();

    for sack in data.lines() {
        // clear previous sack to overwrite
        elfgroup[elfidx].clear();

        // insert items from each sack into the set for the correct group member
        sack.chars().for_each(|c| {
            elfgroup[elfidx].insert(c);
        });

        // circularly update the elf group index
        elfidx = (elfidx + 1) % GROUP_SIZE;

        if elfidx == 0 {
            // filter-out items not contained in other groups
            let (common, others) = elfgroup.split_at_mut(1);
            let common = &mut common[0];
            for other in others {
                common.retain(|c| other.contains(c));
            }

            // expect the first item in the iterator and no more
            let mut itcommon = common.iter();
            let item = itcommon.next().unwrap();
            assert_eq!(0, itcommon.count());

            // determine priority of common item and add to sum
            sum += priority(item)?;
        }
    }

    Ok(sum)
}

fn main() -> anyhow::Result<()> {
    let data = read_input()?;
    println!("Part One: {}", part_one(&data)?);
    println!("Part Two: {}", part_two(&data)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
