use std::{fs, io};

use raytracer::prelude::*;

fn main() -> io::Result<()> {
    fs::create_dir_all("output")?;

    let mut canvas = Canvas::new(900usize, 550usize);

    let mut p = Projectile {
        velocity: vector(1, 1.8, 0).normalize_or_zero() * 11.25,
        position: point(0, 1, 0),
    };

    let color = Color3::RED;
    let e = Environment {
        gravity: vector(0, -0.1, 0),
        wind:    vector(-0.01, 0, 0),
    };

    while p.position.y() > 0.0 {
        let x = p.position.x().round() as usize;
        let y = p.position.y().round() as usize;

        for w in x..=x + 5 {
            for h in y..=y + 5 {
                if w > 0 && w < canvas.width() && h > 0 && h < canvas.height() {
                    canvas.write_pixel(w, canvas.height() - h, color);
                }
            }
        }

        p = tick(p, e);
    }

    canvas.export("output/ch02_projectile.ppm")
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
