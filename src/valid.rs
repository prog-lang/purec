trait Valid {
    type Error;
    fn validate(&self) -> Result<(), Self::Error>;
    fn is_valid(&self) -> bool {
        self.validate().is_ok()
    }
}

impl Valid for AST {
    type Error = String;

    fn validate(&self) -> Result<(), Self::Error> {
        self.validate_exports()?;
        self.validate_decls_types()
    }
}
