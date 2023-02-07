use std::{io::Error};

mod typescript;
mod less;

pub fn load() -> Result<(), Error> {
    less::load()?;
    typescript::load()?;

    Ok(())
}