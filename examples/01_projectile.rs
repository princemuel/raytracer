use std::fs::File;
use std::io::BufWriter;
use std::{fs, io};

use raytracer::prelude::*;

fn main() -> io::Result<()> {
    use io::Write as _;

    fs::create_dir_all("output")?;

    let file = File::create("output/ch01_projectile.txt")?;

    let mut buffer = BufWriter::new(file);

    let mut p = Projectile {
        velocity: unsafe { vector(1, 1, 0).normalize_unchecked() },
        position: point(0, 1, 0),
    };
    let e = Environment {
        wind:    vector(-0.01, 0, 0),
        gravity: vector(0, -0.1, 0),
    };

    while p.position.y() >= 0.0 {
        let x = p.position.x();
        let y = p.position.y();
        let z = p.position.z();

        writeln!(&mut buffer, " x: {x:.2}\t|\ty: {y:.2}\t|\tz: {z:.2}",)?;

        p = tick(p, e);
    }

    Ok(())
}

fn tick(mut p: Projectile, e: Environment) -> Projectile {
    p.position = p.position + p.velocity;
    p.velocity = p.velocity + e.gravity + e.wind;
    p
}

#[derive(Clone, Copy, Debug, Default)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

#[derive(Clone, Copy, Debug, Default)]
struct Environment {
    gravity: Vector,
    wind:    Vector,
}
