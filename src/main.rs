fn main() {
    let btr = BackTrackResult::Invalid;
    println!("{:?}", btr);
}

struct DS {
    num_exactly_once: u32,
    num_at_least_twice: u32,
    counter: [u32; 1024],
}

fn init_datastructure(x: &[u32], ds: &mut DS) -> () {
    ()
}

fn is_valid(ds: &DS) -> bool {
    true
}

fn is_done(ds: &DS) -> bool {
    true
}

enum BackTrackResult {
    Exhausted,
    Invalid,
    Found([i32; 1024]),
}

fn search(ds: &mut DS, level: u32, x: &mut [u32], to_search: &[u32]) -> BackTrackResult {
    let b = BackTrackResult::Exhausted;
    b
}
