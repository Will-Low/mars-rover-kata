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
struct Rover {
    position: Position,
    orientation: Orientation,
}

impl Rover {
    fn new() -> Self {
        Rover {
            position: Position::new(),
            orientation: Orientation::new(),
        }
    }

    fn plot(self, grid: &Grid, commands: &[char]) -> Rover {
        commands.iter().fold(self, |new_location, c| {
            handle_command(new_location, grid, c)
        })
    }

    fn rotate_right(&self) -> Rover {
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

        Rover {
            position,
            orientation,
        }
    }

    fn rotate_left(&self) -> Rover {
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

        Rover {
            position,
            orientation,
        }
    }

    fn forward(&self, grid: &Grid) -> Rover {
        let position = {
            use Orientation::*;
            match self.orientation {
                North => self.move_north(grid),
                East => self.move_east(grid),
                South => self.move_south(grid),
                West => self.move_west(grid),
            }
        };
        let orientation = self.orientation.clone();

        Rover {
            position,
            orientation,
        }
    }

    fn move_west(&self, grid: &Grid) -> Position {
        if self.position.x.checked_sub(1).is_none() {
            Position {
                x: grid.width - 1,
                y: self.position.y,
            }
        } else {
            Position {
                x: self.position.x - 1,
                y: self.position.y,
            }
        }
    }

    fn move_south(&self, grid: &Grid) -> Position {
        if self.position.y.checked_sub(1).is_none() {
            Position {
                x: self.position.x,
                y: grid.height - 1,
            }
        } else {
            Position {
                x: self.position.x,
                y: self.position.y - 1,
            }
        }
    }

    fn move_east(&self, grid: &Grid) -> Position {
        if grid.width == self.position.x + 1 {
            Position {
                x: 0,
                y: self.position.y,
            }
        } else {
            Position {
                x: self.position.x + 1,
                y: self.position.y,
            }
        }
    }

    fn move_north(&self, grid: &Grid) -> Position {
        if grid.height == self.position.y + 1 {
            Position {
                x: self.position.x,
                y: 0,
            }
        } else {
            Position {
                x: self.position.x,
                y: self.position.y + 1,
            }
        }
    }
}

fn handle_command(rover: Rover, grid: &Grid, command: &char) -> Rover {
    match command {
        'R' => rover.rotate_right(),
        'L' => rover.rotate_left(),
        'M' => rover.forward(grid),
        _ => unimplemented!(),
    }
}

impl From<Rover> for String {
    fn from(location: Rover) -> Self {
        let x = location.position.x;
        let y = location.position.y;
        let orientation = location.orientation;
        format!("{x}:{y}:{orientation}")
    }
}

pub struct Grid {
    width: u64,
    height: u64,
}

impl Grid {
    pub fn new(width: u64, height: u64) -> Self {
        assert!(width > 0, "Cannot have a grid with a width of 0");
        assert!(height > 0, "Cannot have a grid with a height of 0");
        Grid { width, height }
    }

    pub fn execute(&self, command_str: &str) -> String {
        let commands: Vec<char> = command_str.chars().collect();
        let rover = Rover::new();
        let final_location = rover.plot(self, &commands);
        String::from(final_location)
    }
}
