# Ray Tracer Challenge

![CI Status](https://github.com/agentbellnorm/ray-tracer-challenge/actions/workflows/rust-ci.yml/badge.svg)

A **ray tracer written in Rust**, inspired by [_The Ray Tracer Challenge_](http://raytracerchallenge.com/) by Jamis Buck.  
The book provides pseudocode and test cases; the implementation is left to the reader.

---

## Table of Contents

- [Overview](#overview)
- [What is Ray Tracing?](#what-is-ray-tracing)
- [Features](#features)
- [Run](#run)
- [Screenshots](#screenshots)
- [Roadmap](#roadmap)
- [Learning Notes](#learning-notes)
- [Resources](#resources)
- [Author](#author)

---

## Overview

This project serves two purposes:

- **Showcase**: demonstrate progress in building a ray tracer step by step.
- **Learning log**: capture insights about Rust, graphics programming, and math along the way.

---

## What is Ray Tracing?

Ray tracing is a rendering technique that simulates how light interacts with objects and materials to produce highly realistic images.  
It’s powerful for accuracy but computationally expensive.

🔗 [Ray Tracing on Wikipedia](<https://en.wikipedia.org/wiki/Ray_tracing_(graphics)>)

---

## Features

- ✅ Ray–sphere intersections
- ✅ Simple shading & lighting
- ✅ Render output to PPM/PNG
- 🚧 Reflections, refractions, and more primitives

---

## Run

```sh
cargo run --release
```

---

## Screenshots

<!-- _(Example renders go here — from first spheres to more complex scenes. Show progress, not just final results.)_ -->

---

## Roadmap

- [ ] Basic ray–sphere renderer
- [ ] Shadows & multiple light sources
- [ ] Reflections & refractions
- [ ] Additional primitives: planes, cubes, cylinders, cones etc
- [ ] Patterned materials (stripes, gradients, checkers, perturbed textures)
- [ ] Complex scenes with multiple objects and light sources
- [ ] Performance optimizations (bounding volume hierarchy, parallelism, SIMD?)
- [ ] Full test coverage with language-agnostic examples from the book
- [ ] Comprehensive Rust documentation (`cargo doc` with examples)
- [ ] Project restructuring to target **WebAssembly**
- [ ] Build a web app frontend to generate 3D images interactively
- [ ] Publish online demo with examples and benchmarks

---

## Learning Notes

Things I’ve learned while working on this project:

- Rust’s **ownership model** pushed me toward cleaner data flow between rays, geometry, and scenes.
- Writing **matrix math** from scratch clarified transformations and coordinate systems.
- Debugging intersections required intermediate visualizations (like surface normals).
- Translating pseudocode into idiomatic Rust often meant rethinking data structures.

---

## Resources

- [_The Ray Tracer Challenge_ — Jamis Buck](http://raytracerchallenge.com/)
- [Ray Tracing on Wikipedia](<https://en.wikipedia.org/wiki/Ray_tracing_(graphics)>)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)
- [Cucumber Rust Repo](https://github.com/cucumber-rs/cucumber)
- [Cucumber Rust Docs](https://cucumber-rs.github.io/cucumber/main/)

---

## Author

- GitHub — [princemuel](https://github.com/princemuel)
- Twitter — [@iamprincemuel](https://x.com/iamprincemuel) princemuel
- Website — princemuel

---
