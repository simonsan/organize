#[cfg(test)]
mod tests {
    use crate::string::{Capitalize, Placeholder};
    use std::{
        io::{Error, ErrorKind},
        path::Path,
    };

    #[test]
    fn capitalize() -> Result<(), Error> {
        let tested = String::from("house");
        let expected = String::from("House");
        if tested.capitalize() == expected {
            Ok(())
        } else {
            Err(Error::from(ErrorKind::Other))
        }
    }
    #[test]
    fn single_placeholder() -> Result<(), Error> {
        let tested = String::from("/home/cabero/Downloads/{parent.name}");
        let new_path = tested.expand_placeholders(&Path::new("/home/cabero/Documents/test.pdf"))?;
        let expected = String::from("/home/cabero/Downloads/Documents");
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::from(ErrorKind::Other))
        }
    }
    #[test]
    fn multiple_placeholders() -> Result<(), Error> {
        let tested = String::from("/home/cabero/{extension}/{parent.name}");
        let new_path = tested.expand_placeholders(&Path::new("/home/cabero/Documents/test.pdf"))?;
        let expected = String::from("/home/cabero/pdf/Documents");
        if new_path == expected {
            Ok(())
        } else {
            Err(Error::from(ErrorKind::Other))
        }
    }
}
