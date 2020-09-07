use crate::common::geom::Vec2;

pub struct Player {
    name: String,
    color: u32,
    inputs: InputState,
    ping: i32,
    // pos: MoveState,
}

pub struct MoveState {
    o: Vec2<f64>,
    v: Vec2<f64>,
}

pub struct InputState;

impl Player {
    fn new(name: &str, color: u32) -> Player {
        Player {
            name: filter_name(name),
            color: filter_color(color),
            inputs: InputState {},
            ping: -1,
            // pos: MoveState {},
        }
    }
}

fn filter_name(name: &str) -> String {
    return "unnamed".to_owned();
}

fn filter_color(c: u32) -> u32 {
    return c & 0xFFFFFF;
}
