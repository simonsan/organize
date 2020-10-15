#[cfg(test)]
mod expand_env_var {
    use std::{
        io::{
            Error,
            ErrorKind,
        },
        path::PathBuf,
    };

    use crate::path::Expand;
    use dirs::home_dir;

    #[test]
    fn home() -> Result<(), Error> {
        let tested = PathBuf::from("$HOME/Documents").expand();
        let expected = home_dir().unwrap().join("Documents");
        if tested == expected {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::InvalidData,
                "the environment variable wasn't properly expanded",
            ))
        }
    }
}
