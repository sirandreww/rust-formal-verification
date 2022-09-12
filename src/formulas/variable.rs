// ************************************************************************************************
// use
// ************************************************************************************************

use std::fmt;

// ************************************************************************************************
// struct
// ************************************************************************************************

#[derive(Hash, PartialEq, Eq, Clone, PartialOrd, Ord)]
/// Variable representation
pub struct Variable {
    variable_number: i32,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl Variable {
    /// Returns a variable with the number given
    ///
    /// # Arguments
    ///
    /// * `variable_number` - An integer that holds the number of the variable
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use rust_formal_verification::formulas::Variable;
    /// let var1 = Variable::new(30);
    /// println!("var1 number is {}", var1);
    /// ```
    pub fn new(variable_number: i32) -> Self {
        debug_assert!(
            variable_number > 0,
            "Variable number must be strictly positive."
        );
        Self { variable_number }
    }

    pub fn get_number(&self) -> i32 {
        self.variable_number
    }
}

// ************************************************************************************************
// printing
// ************************************************************************************************

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x{}", self.variable_number)
    }
}
