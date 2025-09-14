use std::io::Write;
use std::{fs, io};

use raytracer::prelude::*;

fn main() -> io::Result<()> {
    fs::create_dir_all("outputs/01")?;
    let file = fs::File::create("outputs/01/projectile.txt")?;
    let mut writer = io::BufWriter::new(file);

    let gravity = vector(0.0, -0.1, 0.0);
    let position = point(0, 1, 0);
    let velocity = unsafe { vector(1, 1, 0).normalize_unchecked() };
    let wind = vector(-0.01, 0.0, 0.0);

    let e = Environment::new(gravity, wind);
    let mut p = Projectile::new(position, velocity);
    let mut count = 1;

    while p.position.y() > 0.0 {
        writeln!(
            writer,
            " x: {:.2}\t|\ty: {:.2}\t|\tz: {:.2}\t|\ttick: {}",
            p.position.x(),
            p.position.y(),
            p.position.z(),
            count
        )?;

        p = tick(e, p);
        count += 1;
    }

    Ok(())
}

fn tick(e: Environment, p: Projectile) -> Projectile {
    let position = p.position + p.velocity;
    let velocity = p.velocity + e.gravity + e.wind;

    Projectile { position, velocity }
}

#[derive(Default, Debug, Clone, Copy)]
struct Projectile {
    position: Point,
    velocity: Vector,
}
impl Projectile {
    pub fn new(position: Point, velocity: Vector) -> Self { Self { position, velocity } }
}

#[derive(Default, Debug, Clone, Copy)]
struct Environment {
    gravity: Vector,
    wind:    Vector,
}
impl Environment {
    pub fn new(gravity: Vector, wind: Vector) -> Self { Self { gravity, wind } }
}
