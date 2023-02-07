use std::io::Error;

mod less;
mod typescript;

pub fn load() -> Result<(), Error> {
    less::load()?;
    typescript::load()?;

    Ok(())
}
