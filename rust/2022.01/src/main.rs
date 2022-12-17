use std::collections::BinaryHeap;
use std::fs::File;
use std::path::PathBuf;

fn read_input(mut file: impl std::io::Read) -> anyhow::Result<String> {
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

fn main() -> anyhow::Result<()> {
    // A priority queue implemented with a binary heap
    let mut heap: BinaryHeap<usize> = BinaryHeap::new();
    let mut calories: usize = 0;

    let file = File::open(PathBuf::from("input"))?;
    for line in read_input(file)?.lines() {
        match line.parse::<usize>() {
            // lines valid numerical data
            Ok(snack_calories) => {
                calories += snack_calories;
            }
            // empty lines, expected
            Err(_) => {
                // add sum of calories to the heap
                heap.push(calories);

                // reset sum of calories for elf
                calories = 0;
            }
        }
    }

    // add the final Elf's entry, as an empty line does not exist at the end
    heap.push(calories);

    // Retrieve the most calories carried by popping-off the priority queue
    let most: [usize; 3] = [
        heap.pop().unwrap_or(0),
        heap.pop().unwrap_or(0),
        heap.pop().unwrap_or(0),
    ];

    println!(
        "The top three most calories carried by Elves: {}, {}, {}",
        most[0], most[1], most[2]
    );
    println!(
        "Total calories carried by these Elves: {}",
        most.into_iter().sum::<usize>()
    );
    Ok(())
}
