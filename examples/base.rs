//! Example showing how to create a set of `Quantity` type aliases for a different set of base
//! units.

#[macro_use]
extern crate uom;

use uom::si::common::time::second;
use uom::si::geometry::length::{centimeter, meter};

mod cgs {
    ISQ!(uom::si, f32, (centimeter, gram, second, ampere, kelvin, mole, candela));
}

fn main() {
    let l1 = uom::si::f32::geometry::Length::new::<meter>(1.0);
    let l2 = cgs::Length::new::<centimeter>(1.0);
    let t1 = uom::si::f32::common::Time::new::<second>(15.0);

    println!("{}: {:?}", uom::si::geometry::length::description(), l1);
    println!("{}: {:?}", uom::si::geometry::length::description(), l2);
    println!("{:?} + {:?} = {:?}", l1, l2, (l1 + l2));
    println!("{:?} + {:?} = {:?}", l2, l1, (l2 + l1));
    println!("{:?} / {:?} = {:?}", l2, t1, (l2 / t1));
}
