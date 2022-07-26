pub mod err;
mod edge;
mod img;
pub use err::*;


fn main() -> Result<(),BoxErr> {
    let image = img::load("test.png")?;
    Ok(())
}


