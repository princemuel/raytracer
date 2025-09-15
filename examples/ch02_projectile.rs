use std::{fs, io};

use raytracer::prelude::*;

fn main() -> io::Result<()> {
    use io::Write as _;

    fs::create_dir_all("output")?;

    let file = fs::File::create("output/ch01_projectile.txt")?;

    let mut buffer = io::BufWriter::new(file);

    let mut projectile = Projectile {
        position: point(0, 1, 0),
        velocity: unsafe { vector(1, 1, 0).normalize_unchecked() },
    };
    let environment = Environment {
        gravity: vector(0.0, -0.1, 0.0),
        wind:    vector(-0.01, 0.0, 0.0),
    };

    while projectile.position.y() >= 0.0 {
        let x = projectile.position.x();
        let y = projectile.position.y();
        let z = projectile.position.z();

        writeln!(&mut buffer, " x: {x:.2}\t|\ty: {y:.2}\t|\tz: {z:.2}",)?;

        projectile = tick(projectile, environment);
    }

    Ok(())
}

fn tick(mut projectile: Projectile, environment: Environment) -> Projectile {
    projectile.position = projectile.position + projectile.velocity;
    projectile.velocity = projectile.velocity + environment.gravity + environment.wind;
    projectile
}

#[derive(Default, Debug, Clone, Copy)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

#[derive(Default, Debug, Clone, Copy)]
struct Environment {
    gravity: Vector,
    wind:    Vector,
}
