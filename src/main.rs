/*
(normal)
for i in array:
    i = {
        if random_inclusve(0, 99) < n:
            ON
        else:
            OFF
    }

(new)
i = 0
while i in 0..array.len():
    array[i] = 1
    i += random_inclusive(0, (100/n).ceil())
    
*/

// 2.33333 = (100 - 30) / 30

// ! I do not believe I can make the new_al() work due to rounding errors from converting it to a usize. Unsure.

use rand::Rng;

const OUTPUT_LENGTH: usize = 1_000_000;
const CHANCE_TO_SET: u32   = 35;

fn main() {
    println!("\nNAIVE ALGORITHM: ");
    print_output(&naive_al());

    println!("\nNEW ALGORITHM: ");
    print_output(&new_al());
}

fn print_output(out: &Vec<bool>) {
    let count = out.len();
    let num_of_trues = out.iter().filter(|b| **b).count();
    let num_of_false = count - num_of_trues;
    let percent_of_trues = (num_of_trues as f64) / (count as f64) * 100.;
    let average_spaces = count_average_spaces(&out);

    // Print the vector if OUTPUT_LENGTH isn't too high.
    if OUTPUT_LENGTH <= 100 {
        println!("\n-----\n{:?}", out);
    }

    println!(
        "
        Number of Trues: {}
        -----
        Number of Falses: {}
        -----
        Percent of True: {}%
        -----
        Avg Number of Groups of False: {}
        ",
        num_of_trues,
        num_of_false,
        percent_of_trues,
        average_spaces,
    );
}

fn count_average_spaces(to_count: &Vec<bool>) -> f64 {
    let mut counts: Vec<usize> = Vec::new();
    let mut current_count: usize = 0;
    
    // When it encounters false, it counts up, when it encounters true, it ends
    // the current count and pushes it to a vector, to be averaged out.
    for element in to_count {
        if *element {
            counts.push(current_count);
            current_count = 0;
        } else {
            current_count += 1;
        }
    }

    // Calculate Average
    {
        let sum: f64 = counts.iter().sum::<usize>() as f64;
        let count: f64 = counts.len() as f64;
        
        sum / count
    }
}

fn naive_al() -> Vec<bool> {
    let mut out: Vec<bool> = vec![false; OUTPUT_LENGTH];
    let mut rng = rand::thread_rng();
    for i in out.iter_mut() {
        if rng.gen_range(0..100) < CHANCE_TO_SET {
            *i = true;
        }
    }

    out
}

fn new_al() -> Vec<bool> {
    const CHANCE_TO_SET_F64: f64 = CHANCE_TO_SET as f64;
    const EXPECTED_AVERAGE_GROUPS_OF_FALSE: f64 = (100. - CHANCE_TO_SET_F64) / CHANCE_TO_SET_F64;

    let mut out: Vec<bool> = vec![false; OUTPUT_LENGTH];
    let mut rng = rand::thread_rng();

    let mut i: usize = 0;
    while i < OUTPUT_LENGTH {
        out[i] = true;
        i += rng.gen_range(
            0..
            (
                ((EXPECTED_AVERAGE_GROUPS_OF_FALSE) * 200.0) as usize
            )
        );
    }

    out
}