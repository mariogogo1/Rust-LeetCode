pub fn gcd<T: std::cmp::PartialEq + std::ops::Rem<Output = T> + Default + Copy>(
    mut a: T,
    mut b: T,
) -> T {
    while b != T::default() {
        (a, b) = (b, a % b)
    }
    a
}
