pub struct Iota(pub i64);

impl Iterator for Iota {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        Some(self.0)
    }
}
