///code inspire by 
/// <li><a href="https://gitlab.com/ideasman42/rust-simple-examples/blob/master/polyfill_2d/src/main.rs">gitlab.com/ideasman42
/// <li><a href="https://github.com/libgdx/libgdx/blob/master/gdx/src/com/badlogic/gdx/math/EarClippingTriangulator.java">github.com/libgdx


#[derive(Debug,Clone, Copy)]
///Define the concativity of a point
enum Concativity{
    Concave,
    Convex,
    Tangential
}

//TODO