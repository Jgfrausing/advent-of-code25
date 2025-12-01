fn main() {
    // read file of rotations
    let file_name = "input.txt";
    let lines = std::fs::read_to_string(file_name).expect("Failed to read file");
    let mut rotations: Vec<Rotation> = Vec::new();
    for line in lines.lines() {
        let rotation: Rotation = line.parse().expect("Failed to parse rotation");
        rotations.push(rotation);
    }
    let mut dial = Dial::new(50);

    for rotation in rotations {
        dial.rotate(rotation);
    }
    println!("dial: {dial:?}")
}

#[derive(Debug)]
struct Dial {
    position: i64,
    times_at_position_zero: u64,
    times_ending_at_position_zero: u64,
}

impl Dial {
    fn new(position: i64) -> Dial {
        Dial {
            position,
            times_at_position_zero: 0,
            times_ending_at_position_zero: 0,
        }
    }
    fn rotate(&mut self, rotation: Rotation) {
        let full_turns = (rotation.0 / 100).abs() as u64;
        self.times_at_position_zero += full_turns;

        let rotation = rotation.reduce();
        if rotation.0 != 0 {
            let indicator = self.position + rotation.0;
            if indicator <= 0 || indicator >= 100 {
                self.times_at_position_zero += 1;
            }
        }

        self.position = (self.position + rotation.0).rem_euclid(100);
        if self.position == 0 {
            self.times_ending_at_position_zero += 1;
        }
    }
}

#[derive(Debug)]
struct Rotation(i64);

impl Rotation {
    fn reduce(self) -> Rotation {
        if self.0 > 0 {
            Rotation(self.0.rem_euclid(100))
        } else {
            let positive_equivalent = -self.0;
            Rotation(-positive_equivalent.rem_euclid(100))
        }
    }
}

impl std::str::FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s[1..].parse::<i64>().map_err(|e| e.to_string())?;

        if s.starts_with('L') {
            Ok(Rotation(-value))
        } else if s.starts_with('R') {
            Ok(Rotation(value))
        } else {
            Err("Expected string to start with 'L' or 'R'".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduce() {
        let rotation = Rotation(250);
        let reduced = rotation.reduce();
        assert_eq!(reduced.0, 50);

        let rotation = Rotation(-250);
        let reduced = rotation.reduce();
        assert_eq!(reduced.0, -50);

        let rotation = Rotation(75);
        let reduced = rotation.reduce();
        assert_eq!(reduced.0, 75);

        let rotation = Rotation(-75);
        let reduced = rotation.reduce();
        assert_eq!(reduced.0, -75);
    }
    #[test]
    fn rotate() {
        let mut dial = Dial::new(50);
        dial.rotate(Rotation(100));

        assert_eq!(dial.position, 50);
        assert_eq!(dial.times_at_position_zero, 1);
        assert_eq!(dial.times_ending_at_position_zero, 0);

        dial.rotate(Rotation(-100));
        assert_eq!(dial.position, 50);
        assert_eq!(dial.times_at_position_zero, 2);
        assert_eq!(dial.times_ending_at_position_zero, 0);

        dial.rotate(Rotation(-50));
        assert_eq!(dial.position, 0);
        assert_eq!(dial.times_at_position_zero, 3);
        assert_eq!(dial.times_ending_at_position_zero, 1);

        dial.rotate(Rotation(1));
        assert_eq!(dial.position, 1);
        assert_eq!(dial.times_at_position_zero, 3);
        assert_eq!(dial.times_ending_at_position_zero, 1);

        dial.rotate(Rotation(-2));
        assert_eq!(dial.position, 99);
        assert_eq!(dial.times_at_position_zero, 4);
        assert_eq!(dial.times_ending_at_position_zero, 1);

        dial.rotate(Rotation(-99));
        assert_eq!(dial.position, 0);
        assert_eq!(dial.times_at_position_zero, 5);
        assert_eq!(dial.times_ending_at_position_zero, 2);

        dial.rotate(Rotation(50));
        assert_eq!(dial.position, 50);
        assert_eq!(dial.times_at_position_zero, 5);
        assert_eq!(dial.times_ending_at_position_zero, 2);

        dial.rotate(Rotation(10000));
        assert_eq!(dial.position, 50);
        assert_eq!(dial.times_at_position_zero, 105);
        assert_eq!(dial.times_ending_at_position_zero, 2);

        dial.rotate(Rotation(50));
        assert_eq!(dial.position, 0);
        assert_eq!(dial.times_at_position_zero, 106);

        dial.rotate(Rotation(-10000));
        assert_eq!(dial.position, 0);
        assert_eq!(dial.times_at_position_zero, 206);
    }
}
