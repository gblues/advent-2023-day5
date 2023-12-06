mod mapper;
mod seed;

fn main() {
    let sections: Vec<_> = include_str!("input/5.txt").split("\n\n").collect();

    let seed_to_soil:Vec<_>  = parse_section(sections[1]);
    let soil_to_fertilizer:Vec<_> = parse_section(sections[2]);
    let fertilizer_to_water: Vec<_> = parse_section(sections[3]);
    let water_to_light: Vec<_> = parse_section(sections[4]);
    let light_to_temp: Vec<_> = parse_section(sections[5]);
    let temp_to_humidity: Vec<_> = parse_section(sections[6]);
    let humidity_to_location: Vec<_> = parse_section(sections[7]);

    let seeds: Vec<_> = sections[0].split_whitespace().filter_map(|s| s.parse::<u64>().ok()).collect();

    let mut seed_locations: Vec<_> = seeds.iter()
        .map(|seed|
            seed::SeedLocationBuilder::new(*seed)
                .soil(&seed_to_soil)
                .fertilizer(&soil_to_fertilizer)
                .water(&fertilizer_to_water)
                .light(&water_to_light)
                .temp(&light_to_temp)
                .humidity(&temp_to_humidity)
                .location(&humidity_to_location)
                .build()
        )
        .collect();

    seed_locations.sort_by(|o1, o2| o2.location.cmp(&o1.location));
    println!("{}", seed_locations.last().unwrap().location);
}

fn parse_section(section_txt: &str) -> Vec<mapper::Mapper> {
    section_txt.split("\n").filter(|f| !f.ends_with(":")).map(|r| {
        let parts:Vec<_> = r.split_whitespace().map(|n| n.parse::<u64>().unwrap()).collect();
        match parts.len() {
            3 => mapper::Mapper::new(parts[1], parts[0],parts[2]),
            _ => panic!("Expected 3 parts, got {}", parts.len())
        }
    }).collect()
}

