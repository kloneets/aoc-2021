fn main() {
    let initial_values = "1,1,1,1,2,1,1,4,1,4,3,1,1,1,1,1,1,1,1,4,1,3,1,1,1,5,1,3,1,4,1,2,1,1,5,1,1,1,1,1,1,1,1,1,1,3,4,1,5,1,1,1,1,1,1,1,1,1,3,1,4,1,1,1,1,3,5,1,1,2,1,1,1,1,4,4,1,1,1,4,1,1,4,2,4,4,5,1,1,1,1,2,3,1,1,4,1,5,1,1,1,3,1,1,1,1,5,5,1,2,2,2,2,1,1,2,1,1,1,1,1,3,1,1,1,2,3,1,5,1,1,1,2,2,1,1,1,1,1,3,2,1,1,1,4,3,1,1,4,1,5,4,1,4,1,1,1,1,1,1,1,1,1,1,2,2,4,5,1,1,1,1,5,4,1,3,1,1,1,1,4,3,3,3,1,2,3,1,1,1,1,1,1,1,1,2,1,1,1,5,1,3,1,4,3,1,3,1,5,1,1,1,1,3,1,5,1,2,4,1,1,4,1,4,4,2,1,2,1,3,3,1,4,4,1,1,3,4,1,1,1,2,5,2,5,1,1,1,4,1,1,1,1,1,1,3,1,5,1,2,1,1,1,1,1,4,4,1,1,1,5,1,1,5,1,2,1,5,1,1,1,1,1,1,1,1,1,1,1,1,3,2,4,1,1,2,1,1,3,2".to_owned();
    let fishes: Vec<u16> = initial_values
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    println!("Initial state: {:?}", fishes);
    println!(
        "After 80 days we have {} fishes (better)",
        count_fishes(fishes.clone(), 80)
    );
    println!(
        "After 256 days we have {} fishes (better)",
        count_fishes(fishes, 256)
    );
}

fn count_fishes(fishes: Vec<u16>, days: i32) -> i64 {
    // count of counts
    let mut type_count: Vec<i64> = vec![0; 9];
    for fish in fishes {
        type_count[usize::try_from(fish).unwrap()] += 1;
    }
    for _ in 0..days {
        let mut sixes = 0;
        for fish in 0..9 {
            let to_move = type_count[fish];
            type_count[fish] = 0;
            if fish == 0 {
                //move to 8
                sixes = to_move;
            } else {
                type_count[fish - 1] += to_move;
            }
        }
        type_count[6] += sixes;
        type_count[8] += sixes;
    }
    type_count.iter().sum::<i64>()
}
