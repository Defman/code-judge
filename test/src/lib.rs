pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Judge {
    fn judge(&self);
}
