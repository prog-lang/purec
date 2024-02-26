pub trait Valid {
    type Error;

    fn validate(&self) -> Result<(), Self::Error>;

    fn valid(self) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        self.validate().map(|_| self)
    }

    fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}
