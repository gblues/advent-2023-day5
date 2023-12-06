use crate::mapper;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct SeedLocation {
    pub seed: u64,
    pub location: u64
}

impl SeedLocation {
    fn builder() -> SeedLocationBuilder { SeedLocationBuilder::default() }
}

#[derive(Default)]
pub struct SeedLocationBuilder {
    seed: u64,
    soil: Option<u64>,
    fertilizer: Option<u64>,
    water: Option<u64>,
    light: Option<u64>,
    temp: Option<u64>,
    humidity: Option<u64>,
    location: Option<u64>
}

impl SeedLocationBuilder {
    pub fn new(seed: u64) -> SeedLocationBuilder {
        SeedLocationBuilder {
            seed,
            soil: None,
            fertilizer: None,
            water: None,
            light: None,
            temp: None,
            humidity: None,
            location: None
        }
    }
    pub fn soil(mut self, mappers: &Vec<mapper::Mapper>) -> SeedLocationBuilder {
        self.soil = Some(SeedLocationBuilder::apply_map(mappers, self.seed));
        self
    }

    pub fn fertilizer(mut self, mappers: &Vec<mapper::Mapper>) -> SeedLocationBuilder {
        assert_ne!(self.soil, None);
        self.fertilizer = Some(SeedLocationBuilder::apply_map(mappers, self.soil.unwrap()));
        self
    }

    pub fn water(mut self, mappers: &Vec<mapper::Mapper>) -> SeedLocationBuilder {
        assert_ne!(self.fertilizer, None);
        self.water = Some(SeedLocationBuilder::apply_map(mappers, self.fertilizer.unwrap()));
        self
    }

    pub fn light(mut self, mappers: &Vec<mapper::Mapper>) -> SeedLocationBuilder {
        assert_ne!(self.water, None);
        self.light = Some(SeedLocationBuilder::apply_map(mappers, self.water.unwrap()));
        self
    }

    pub fn temp(mut self, mappers: &Vec<mapper::Mapper>) -> SeedLocationBuilder {
        assert_ne!(self.light, None);
        self.temp = Some(SeedLocationBuilder::apply_map(mappers, self.light.unwrap()));
        self
    }

    pub fn humidity(mut self, mappers: &Vec<mapper::Mapper>) -> SeedLocationBuilder {
        assert_ne!(self.temp, None);
        self.humidity = Some(SeedLocationBuilder::apply_map(mappers, self.temp.unwrap()));
        self
    }

    pub fn location(mut self, mappers: &Vec<mapper::Mapper>) -> SeedLocationBuilder {
        assert_ne!(self.humidity, None);
        self.location = Some(SeedLocationBuilder::apply_map(mappers, self.humidity.unwrap()));
        self
    }

    pub fn build(&self) -> SeedLocation {
        assert_ne!(self.location, None);

        SeedLocation { seed: self.seed, location: self.location.unwrap() }
    }

    fn apply_map(mappers: &Vec<mapper::Mapper>, value: u64) -> u64 {
        let mapper: Vec<_> = mappers.iter().filter(|m| (*m).accept(value)).collect();
        match mapper.len() {
            0 => value,
            1 => mapper[0].map_value(value),
            _ => panic!("expected exactly one mapper to match but got {}", mapper.len())
        }
    }
}