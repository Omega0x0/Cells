pub mod color;
pub mod cell;
pub mod world;
pub mod info;

pub fn limit(min: i64, max: i64, n: i64) -> i64 {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}