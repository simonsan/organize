mod lib;

pub trait Capitalize<T> {
    fn capitalize(&self) -> T;
}

impl Capitalize<String> for String {
    fn capitalize(&self) -> String {
        if self.is_empty() {
            return self.clone();
        }
        let mut c = self.chars();
        c.next().unwrap().to_uppercase().collect::<String>() + c.as_str()
    }
}
