use std::collections::HashSet;

fn main() {
    let datastream = include_str!("input.txt");

    dbg!(datastream.find_marker_of_len(4));
    dbg!(datastream.find_marker_of_len(14));
}

trait ElfDevice {
    fn find_marker_of_len(&self, length: usize) -> Option<usize>;
}

impl ElfDevice for &str {
    fn find_marker_of_len(&self, length: usize) -> Option<usize> {
        self
        .as_bytes()
        .windows(length)
        .position(|window| window.iter().collect::<HashSet<_>>().len() == length)
        .map(|pos| pos + length)
    }
}

#[cfg(test)]
mod tests {
    use crate::ElfDevice;

    #[test]
    fn test_find_packet() {
        assert_eq!(Some(7), "mjqjpqmgbljsphdztnvjfqwrcgsmlb".find_marker_of_len(4));
    }
}