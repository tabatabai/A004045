fn main() {
    let n = 6;
    let goal = 19;
    let mut x = vec![0, 1]; // wlog
    let level = 2; // two choices already made
    let mut ds = DS {
        num_exactly_once: 0,
        num_at_least_twice: 0,
        counter: [0; 64],
    };
    let mut iterations = 0;
    let mut to_search = [0; 64];
    for i in 0..64 {
        to_search[i] = i;
    }
    init_datastructure(&x, &mut ds, n);
    let result = search(
        &mut ds,
        n,
        level,
        &mut x,
        &to_search[2..],
        goal,
        &mut iterations,
    );
    match result {
        BackTrackResult::Found(x) => {
            for vec in x {
                print!(" {} ", vec);
            }
        }
        BackTrackResult::Exhausted => println!("Exhausted."),
        BackTrackResult::Invalid => println!("Invalid."),
    }
}

struct DS {
    num_exactly_once: usize,
    num_at_least_twice: usize,
    counter: [usize; 64],
}

fn init_datastructure(x: &Vec<usize>, ds: &mut DS, n: usize) -> () {
    for i in x {
        update(ds, *i, n);
    }
}

fn is_valid(ds: &DS, n: usize, level: usize, to_search: &[usize], goal: usize) -> bool {
    if to_search.len() + level < goal {
        return false;
    }
    let missing = 2 * 2usize.pow(n as u32) - 2 * ds.num_at_least_twice - ds.num_exactly_once;
    if missing > (goal - level) * (n + 1) {
        return false;
    }
    return true;
}

enum BackTrackResult {
    Exhausted,
    Invalid,
    Found(Vec<usize>),
}

fn update(ds: &mut DS, vec: usize, n: usize) {
    ds.counter[vec] += 1;
    if ds.counter[vec] == 2 {
        ds.num_at_least_twice += 1;
        ds.num_exactly_once -= 1;
    } else if ds.counter[vec] == 1 {
        ds.num_exactly_once += 1;
    }
    let mut modifier = 1;
    for _ in 0..n {
        let new = vec ^ modifier;
        ds.counter[new] += 1;
        if ds.counter[new] == 2 {
            ds.num_at_least_twice += 1;
            ds.num_exactly_once -= 1;
        } else if ds.counter[new] == 1 {
            ds.num_exactly_once += 1;
        }
        modifier = modifier << 1;
    }
}

fn undo(ds: &mut DS, vec: usize, n: usize) {
    ds.counter[vec] -= 1;
    if ds.counter[vec] == 1 {
        ds.num_exactly_once += 1;
        ds.num_at_least_twice -= 1;
    } else if ds.counter[vec] == 0 {
        ds.num_exactly_once -= 1;
    }
    let mut modifier = 1;
    for _ in 0..n {
        let new = vec ^ modifier;
        ds.counter[new] -= 1;
        if ds.counter[new] == 1 {
            ds.num_exactly_once += 1;
            ds.num_at_least_twice -= 1;
        } else if ds.counter[new] == 0 {
            ds.num_exactly_once -= 1;
        }
        modifier = modifier << 1;
    }
}

fn search(
    ds: &mut DS,
    n: usize,
    level: usize,
    x: &mut Vec<usize>,
    to_search: &[usize],
    goal: usize,
    iterations: &mut u128,
) -> BackTrackResult {
    *iterations += 1;
    if *iterations % 134217728 == 0 {
        println!("{}M iterations.", *iterations / 1_000_000);
        println!("Current solution: {:?}. Depth is {}.", x, level);
    }
    if is_valid(ds, n, level, to_search, goal) {
        if level == goal {
            return BackTrackResult::Found(x.to_vec());
        }
    } else {
        return BackTrackResult::Invalid;
    }

    for (i, vec) in to_search.iter().enumerate() {
        x.push(*vec);
        update(ds, *vec, n);
        let result = search(ds, n, level + 1, x, &to_search[i + 1..], goal, iterations);
        match result {
            BackTrackResult::Found(x) => return BackTrackResult::Found(x),
            _ => {}
        }
        undo(ds, *vec, n);
        x.pop();
    }
    BackTrackResult::Exhausted
}
