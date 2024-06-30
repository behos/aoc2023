const TIMES: [u64; 4] = [61, 67, 75, 71];
const DISTANCES: [u64; 4] = [430, 1036, 1307, 1150];

const TIME: u64 = 61677571;
const DISTANCE: u64 = 430103613071150;

fn main() {
    let total_ways_to_win = TIMES
        .iter()
        .zip(DISTANCES.iter())
        .map(|(t, d)| ways_to_win(*t, *d))
        .product::<usize>();
    let ways_to_win_big_race = ways_to_win(TIME, DISTANCE);
    println!("part 1: {total_ways_to_win}");
    println!("part 2: {ways_to_win_big_race}");
}

fn ways_to_win(time: u64, distance: u64) -> usize {
    // meh, just brute force it man
    (0..time)
        .filter(|charge_time| (time - charge_time) * charge_time > distance)
        .count()
}
