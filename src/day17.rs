use std::ops::RangeInclusive;

use anyhow::{Context, Result};
use itertools::iproduct;

pub fn run() -> Result<()> {
    // let content = std::fs::read_to_string("inputs/day17.txt")?;
    let mut probe = Probe::new(Vec2::new(0, 0), Vec2::new(0, 0), 144..=178, -100..=-76);

    let mut max_y = i64::MIN;
    let mut count = 0;
    for (vx, vy) in iproduct!(1..200, -5000..20000) {
        // println!("V=({}, {})   ", vx, vy);
        probe.reset_with_velocity(Vec2::new(vx, vy));
        if probe.hits_target() {
            // println!("Target hit! V=({}, {}), max_y={}", vx, vy, probe.max_y);
            count += 1;
            if probe.max_y > max_y {
                max_y = probe.max_y;
            }
        }
    }
    println!("day17 part1 = {}", max_y);
    println!("day17 part2 = {}", count);

    Ok(())
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Probe {
    pos: Vec2,
    velocity: Vec2,
    target_x: RangeInclusive<i64>,
    target_y: RangeInclusive<i64>,
    max_y: i64,
}

impl Probe {
    fn new(
        pos: Vec2,
        velocity: Vec2,
        target_x: RangeInclusive<i64>,
        target_y: RangeInclusive<i64>,
    ) -> Self {
        Self {
            pos,
            velocity,
            target_x,
            target_y,
            max_y: i64::MIN,
        }
    }

    fn reset_with_velocity(&mut self, velocity: Vec2) {
        self.pos = Vec2::default();
        self.velocity = velocity;
        self.max_y = i64::MIN;
    }

    fn step(&mut self) {
        self.pos.x += self.velocity.x;
        self.pos.y += self.velocity.y;
        if self.pos.y > self.max_y {
            self.max_y = self.pos.y;
        }

        // drag
        if self.velocity.x > 0 {
            self.velocity.x -= 1;
        } else if self.velocity.x < 0 {
            self.velocity.x += 1;
        }
        // gravity
        self.velocity.y -= 1;
    }

    fn hits_target(&mut self) -> bool {
        loop {
            self.step();
            if (self.velocity.x > 0 && self.pos.x > *self.target_x.end())
            || (self.velocity.x < 0 && self.pos.x < *self.target_x.start())
            || (self.velocity.y < 0 && self.pos.y < *self.target_y.start()){
                // We've got past the target area: we will never hit it
                return false;
            } else if self.target_x.contains(&self.pos.x) && self.target_y.contains(&self.pos.y) {
                // we've hit the target area
                return true;
            }
            // otherwise, just keep going
        }
    }
}
