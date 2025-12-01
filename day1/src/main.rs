fn main() {
    // read file of rotations
    let file_name = "input.txt";
    let lines = std::fs::read_to_string(file_name).expect("Failed to read file");
    let mut rotations: Vec<Rotation> = Vec::new();
    for line in lines.lines() {
        let rotation: Rotation = line.parse().expect("Failed to parse rotation");
        rotations.push(rotation);
    }
    let mut dial = Dial(50);

    let mut zero_positions = 0;
    for rotation in &rotations {
        dial.rotate(rotation);
        if dial.0 == 0 {
            zero_positions += 1;
        }
    }
    println!("zeros: {zero_positions}")
}

#[derive(Debug)]
struct Dial(i64);

impl Dial {
    fn rotate(&mut self, rotation: &Rotation) {
        self.0 = (self.0 + rotation.0).rem_euclid(100);
    }
}

#[derive(Debug)]
struct Rotation(i64);

impl std::str::FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = s[1..].parse::<i64>().map_err(|e| e.to_string())?;
        value = value % 100;

        if s.starts_with('L') {
            Ok(Rotation(-value))
        } else if s.starts_with('R') {
            Ok(Rotation(value))
        } else {
            Err("Expected string to start with 'L' or 'R'".to_string())
        }
    }
}
