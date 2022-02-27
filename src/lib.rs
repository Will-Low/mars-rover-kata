use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Debug, PartialEq)]
enum Orientation {
    North,
    South,
    East,
    West,
}

impl Orientation {
    fn new() -> Self {
        Orientation::North
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl From<&Orientation> for String {
    fn from(orientation: &Orientation) -> Self {
        use Orientation::*;
        match orientation {
            North => "N".to_string(),
            South => "S".to_string(),
            East => "E".to_string(),
            West => "W".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Position {
    x: u64,
    y: u64,
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0 }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct RoverData {
    position: Position,
    orientation: Orientation,
}

impl RoverData {
    fn new() -> Self {
        RoverData {
            position: Position::new(),
            orientation: Orientation::new(),
        }
    }

    fn rotate_right(&self) -> RoverData {
        let position = self.position.clone();
        let orientation = {
            use Orientation::*;
            match self.orientation {
                North => East,
                East => South,
                South => West,
                West => North,
            }
        };

        RoverData {
            position,
            orientation,
        }
    }

    fn rotate_left(&self) -> RoverData {
        let position = self.position.clone();
        let orientation = {
            use Orientation::*;
            match self.orientation {
                North => West,
                West => South,
                South => East,
                East => North,
            }
        };

        RoverData {
            position,
            orientation,
        }
    }

    fn forward(&self) -> RoverData {
        let position = {
            use Orientation::*;
            match self.orientation {
                North => Position {
                    x: self.position.x,
                    y: self.position.y + 1,
                },
                East => Position {
                    x: self.position.x + 1,
                    y: self.position.y,
                },
                South => Position {
                    x: self.position.x,
                    y: self.position.y - 1,
                },
                _ => todo!(),
            }
        };
        let orientation = self.orientation.clone();

        RoverData {
            position,
            orientation,
        }
    }
}

fn plot(location: RoverData, commands: &[char]) -> RoverData {
    commands
        .iter()
        .fold(location, |new_location, c| handle_command(new_location, c))
}

fn handle_command(location: RoverData, command: &char) -> RoverData {
    match command {
        'R' => location.rotate_right(),
        'L' => location.rotate_left(),
        'M' => location.forward(),
        _ => todo!(),
    }
}

impl From<RoverData> for String {
    fn from(location: RoverData) -> Self {
        let x = location.position.x;
        let y = location.position.y;
        let orientation = location.orientation;
        format!("{x}:{y}:{orientation}")
    }
}

pub fn execute(command_str: &str) -> String {
    let commands: Vec<char> = command_str.chars().collect();
    let starting_location = RoverData::new();
    let final_location = plot(starting_location, &commands);
    String::from(final_location)
}
