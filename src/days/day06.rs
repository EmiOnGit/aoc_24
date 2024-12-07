use arrayvec::ArrayVec;

use crate::{print, println};

pub fn part1(input: &str) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let mut buffer = input_to_buffer::<18000>(input);
    let mut position = buffer
        .iter()
        .position(|e| matches!(e, Entity::X(_)))
        .unwrap();
    let mut direction = Dir::North;
    loop {
        let Some(next_position) = maybe_next_position(position, direction, height, width) else {
            break;
        };
        match buffer[next_position] {
            Entity::DOT | Entity::X(_) => {
                position = next_position;
                // Direction only matters in the 2th part
                buffer[position] = Entity::X(direction);
            }
            Entity::HASH => {
                direction = direction.right();
            }
        }
    }
    let visited_points = buffer.iter().filter(|e| matches!(e, Entity::X(_))).count();
    println!("visited points ", visited_points);
}
fn input_to_buffer<const N: usize>(input: &str) -> ArrayVec<Entity, N> {
    let mut buffer = arrayvec::ArrayVec::<Entity, N>::new();
    for c in input.chars() {
        match c {
            '.' => buffer.push(Entity::DOT),
            '#' => buffer.push(Entity::HASH),
            '^' => buffer.push(Entity::X(Dir::North)),
            _ => continue,
        }
    }
    buffer
}
/// Returns Some(next_position) if next_position is valid,
/// None if next_position would be outside of the grid
fn maybe_next_position(
    current_position: usize,
    direction: Dir,
    height: usize,
    width: usize,
) -> Option<usize> {
    match direction {
        Dir::North => current_position.checked_sub(width),
        Dir::East => {
            if current_position % width < width - 1 {
                Some(current_position + 1)
            } else {
                None
            }
        }
        Dir::South => {
            if current_position + width < width * height {
                Some(current_position + width)
            } else {
                None
            }
        }
        Dir::West => {
            if current_position % width > 0 {
                Some(current_position - 1)
            } else {
                None
            }
        }
    }
}
#[derive(PartialEq)]
enum Entity {
    DOT,
    X(Dir),
    HASH,
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    East,
    South,
    West,
}
impl Dir {
    pub fn right(&self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }
}
pub fn part2(input: &str) {
    let height = input.lines().count() - 1;
    let width = input
        .lines()
        .next()
        .unwrap()
        .chars()
        // shouldn't be needed
        .filter(|c| *c == '.' || *c == '#')
        .count();
    let mut buffer = input_to_buffer::<18000>(input);
    println!("buffer:", buffer.len());
    println!("max:", height * width);
    println!("width:", width);
    let start_position = buffer
        .iter()
        .position(|e| matches!(e, Entity::X(_)))
        .unwrap();

    let mut position = start_position;
    let mut obstacle_places = 0;
    let mut direction = Dir::North;
    loop {
        let Some(next_position) = maybe_next_position(position, direction, height, width) else {
            break;
        };
        match buffer[next_position] {
            Entity::DOT => {
                position = next_position;
                buffer[position] = Entity::X(direction);
                if check_direction_cycle(position, direction.right(), &buffer, height, width) {
                    obstacle_places += 1;
                }
            }

            Entity::X(saved_dir) => {
                position = next_position;
                if check_direction_cycle(position, direction.right(), &buffer, height, width) {
                    obstacle_places += 1;
                }
                buffer[position] = Entity::X(direction);
            }
            Entity::HASH => {
                direction = direction.right();
            }
        }
    }
    let visited_points = buffer.iter().filter(|e| matches!(e, Entity::X(_))).count();
    println!("visited points ", visited_points);
    println!("obstacles ", obstacle_places);
}
/// returns true if a cycle is detected
fn check_direction_cycle<const N: usize>(
    mut position: usize,
    direction: Dir,
    buffer: &ArrayVec<Entity, N>,
    height: usize,
    width: usize,
) -> bool {
    while let Some(next_position) = maybe_next_position(position, direction, height, width) {
        if let Entity::X(registered_direction) = buffer[next_position] {
            if registered_direction == direction {
                return true;
            }
        } else if Entity::HASH == buffer[next_position] {
            return false;
        }
        position = next_position;
    }
    false
}
