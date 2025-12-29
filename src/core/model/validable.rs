/// Declares a struct has concrete validation rules to be instantiated.
pub trait Validable {
    fn validate(&self) -> Result<(), &'static str>;
}
