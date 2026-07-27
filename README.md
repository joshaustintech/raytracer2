# raytracer2 [![brainmade logo](img/brainmade.png)](https://brainmade.org/) [![Rust](https://github.com/joshaustintech/raytracer2/actions/workflows/rust.yml/badge.svg)](https://github.com/joshaustintech/raytracer2/actions/workflows/rust.yml)
CPU raytracer

# Introduction

## Current Progress
"Hello World" gradient image generated. Images are saved by sending `stdout` to a file like so:
```sh
./target/debug/raytracer2 > helloworld.ppm
```

![hello world image](img/helloworld.jpg)

## Background
In 2023 I started writing [a simple CPU raytracer](https://github.com/joshaustintech/raytracer)
during the slower moments of my first parental leave. It was nice to have something intellectually
stimulating to pick up and then be able to drop at a moment's notice to help with anything.

I was using [Ray Tracing In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
but sadly never got past making spheres.

3 years later I wanted to give it another attempt, but this time deliberately forego the use of LLM assistance,
even for fancy autocomplete. I use enough of that for work anyway! I firmly believe that
[my brain was made to struggle](https://ericadhawan.substack.com/p/your-brain-was-built-to-struggle) and that
through struggle I multiply my understanding. I want to get better at Rust and I've always been fascinated with
computer graphics ever since trying out Blender for the first time in 2004.

## Why the Brainmade mark?
I believe in explicit disclosure regarding machine-generated code in both directions. If I have a machine generating
my code, I have recently tried disclosing which ones in my git commits. If I have my brain generating the code, then
I want to share my work with no ambiguity or doubt about its origins.

# Technical
## Requirements
- Rust 1.97.1 or newer
## Usage
```sh
cargo run
```
