#[derive(Debug, Clone, Copy)]
pub struct Ewwow;

impl Ewwow {
    pub fn raise(self) -> Result<(), Self> {
        Err(self)
    }
}

unsafe impl Send for Ewwow {}
unsafe impl Sync for Ewwow {}

impl std::error::Error for Ewwow {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }

    fn provide<'a>(&'a self, _request: &mut std::error::Request<'a>) {}
}

impl std::fmt::Display for Ewwow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OOPSIE WOOPSIE!! uwu We made a fucky wucky!!")
    }
}
