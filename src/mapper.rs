pub struct Mapper {
    accept_start: u64,
    provide_start: u64,
    range_length: u64,
}

impl Mapper {
    pub fn new(accept_start: u64, provide_start: u64, range_length: u64) -> Mapper {
        assert!(accept_start + range_length <= u64::MAX);
        return Mapper {
            accept_start,
            provide_start,
            range_length
        }
    }
    pub fn accept(&self, value: u64) -> bool {
        let end = (self.accept_start + self.range_length) - 1;
        match value {
            x if self.accept_start <= x && x <= end => true,
            _ => false
        }
    }

    pub fn map_value(&self, value: u64) -> u64 {
        let offset = value - self.accept_start;
        return self.provide_start + offset;
    }

    pub fn start(&self) -> u64 { self.accept_start }
}