// The `common` module is pub(crate) and must not be accessible from outside the crate.
fn main() {
    let _ = greek_meander::common::Point { x: 0.0, y: 0.0 };
}
