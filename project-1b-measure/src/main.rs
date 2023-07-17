use std::fmt::Display;

use anyhow::anyhow;
use clap::Parser;

#[derive(Parser, Clone, Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = format!("{},{}", self.x, self.y);
        s.fmt(f)
    }
}

impl std::str::FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((l, r)) = s.split_once(',') else {
            anyhow::bail!("Point has a number, then a comma, then another number")
        };
        let x = l
            .parse()
            .map_err(|e| anyhow!("Could not parse a number: {e}"))?;
        let y = r.parse()?;
        Ok(Point { x, y })
    }
}

/// Simple program to measure a line
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct MeasureApp {
    /// First point
    #[arg(long)]
    start: Point,

    /// Second point
    #[arg(long)]
    end: Point,
}

fn main() {
    let args = MeasureApp::parse();

    println!("Start: {}", args.start);
    println!("End: {}", args.end);

    let distance = distance_between_points(args.start, args.end);

    println!("Distance between points: {}", distance);
}

/// Calculate the distance between two points
#[rustfmt::skip]
fn distance_between_points(
    Point { x: x1, y: y1 }: Point, 
    Point { x: x2, y: y2 }: Point,
) -> f32 {
    let x_squared = (x1 - x2).powi(2);
    let y_squared = (y1 - y2).powi(2);
    let sum = x_squared + y_squared;
    let distance = sum.sqrt();
    distance
}
