pub fn clamp<T>(val: T, min: T, max: T) -> T
where
    T: PartialOrd + Copy
{
    if val > max {
        max
    } else if val < min {
        min
    } else {
        val
    }
}