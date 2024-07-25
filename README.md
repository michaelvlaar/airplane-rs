# Airplane Library

This Rust library is designed for easy and accurate airplane-related calculations, focusing on weight and balance, as well as visualization. The library provides functions to calculate center of gravity, moment arms, and weight distribution, ensuring aircraft safety and compliance with regulations. Additionally, it includes tools for visualizing weight and balance envelopes and other key metrics, making it a comprehensive tool for pilots, engineers, and aviation enthusiasts. With a simple and intuitive API, this library simplifies complex aeronautical calculations and visualizations.

## Example
```rust
use std::fs;

use airplane::{visualizer::WeightBalanceVisualization, weight_and_balance::{
    Airplane, CenterOfGravity, LeverArm, Limits, Mass, Moment, Volume,
}};

fn main() {
    let plane = Airplane::new(
        String::from("PHXXX"),
        vec![
            Moment::new(LeverArm::Meter(0.4294), Mass::Kilo(517.0)),
            Moment::new(LeverArm::Meter(0.515), Mass::Kilo(95.0)),
            Moment::new(LeverArm::Meter(0.515), Mass::Kilo(89.0)),
            Moment::new(LeverArm::Meter(1.3), Mass::Kilo(5.0)),
            Moment::new(LeverArm::Meter(0.325), Mass::Avgas(Volume::Liter(55.0))),
        ],
        Limits::new(
            Mass::Kilo(558.0),
            Mass::Kilo(750.0),
            CenterOfGravity::Millimeter(427.0),
            CenterOfGravity::Millimeter(523.0),
        ),
    );

    match airplane::visualizer::weight_and_balance(plane, WeightBalanceVisualization::new((1000,1000),(230.0..420.0, 550.0..760.0))) {
        airplane::visualizer::Visualization::Svg(svg) =>  {
            let _ = fs::write("image.svg", svg);
        },
    };
}
```
