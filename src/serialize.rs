use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
  x: f32,
  y: f32
}

#[derive(Serialize, Deserialize, Debug)]
struct Line {
  points: Vec<Point>,
  valid: bool,
  length: f32,
  desc: String
}

fn main() {
    let point1: Point = Point {x:1.0, y:2.0};
    let point2: Point = Point {x:3.0, y:4.0};
    let point1s = bincode::serialize(&point1).unwrap();
    let point2s = bincode::serialize(&point2).unwrap();
    println!("struct Point serializes into byte array {:?}", point1s);
    println!("struct Point serializes into byte array {:?}", point2s);

    let length = ((point1.x - point2.x) * (point1.x - point2.x) + (point1.y - point2.y) * (point1.y - point2.y)).sqrt();
    let valid = if length == 0.0 { false } else { true };
    let line = Line { points: vec![point1, point2], valid: valid, length: length, desc: "a thin line".to_string() };
    let lines = bincode::serialize(&line).unwrap();
    println!("struct Line serializes into byte array {:?}", lines);

    let lined: Line = bincode::deserialize(&lines).unwrap();
    assert_eq!(lined.desc, "a thin line");
    assert_eq!(lined.points[1].x, 3.0);
}