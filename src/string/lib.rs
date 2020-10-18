#[cfg(test)]
mod tests {
    use crate::string::Capitalize;
    use std::io::{Error, ErrorKind};

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
}
