use std::fmt;

#[derive(Debug, Clone)]
pub struct CensusError {
    pub err_msg: String,
    pub parent_err: Option<String>,
}

impl fmt::Display for CensusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.parent_err {
            Some(parent) => {
                write!(
                    f,
                    "Census Error: {} \nParent Error: {}",
                    self.err_msg, parent
                )
            }
            None => {
                write!(f, "Census Error: {}", self.err_msg)
            }
        }
    }
}
