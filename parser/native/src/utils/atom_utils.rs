#[inline]
pub fn get_version(num: u32) -> u32 {
    (num & 0xffffff00) >> 8
}

#[cfg(test)]
mod tests {
    use super::get_version;

    #[test]
    fn test_get_version() {
        assert_eq!(get_version(3), 0);
    }
}
