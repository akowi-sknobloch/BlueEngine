/*
 * Blue Engine by Elham Aryanpur
 *
 * Triangle example using pre-defined shapes
 *
 * The license is same as the one on the root.
*/

use blue_engine::{
    header::{Engine, ObjectSettings, WindowDescriptor},
    primitive_shapes::triangle,
};

fn main() {
    let mut engine = Engine::new(WindowDescriptor::default()).expect("win");

    let _ = triangle("Triangl", ObjectSettings::default(), &mut engine).unwrap();

    engine
        .update_loop(move |_, _, _, _, _| {})
        .expect("Error during update loop");
}
