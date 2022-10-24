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
    variable_number: u32,
}

// ************************************************************************************************
// impl
// ************************************************************************************************

impl Variable {
    /// Returns a variable with the number given
    ///
    /// # Arguments
    ///
    /// * `variable_number` - An integer that holds the number of the variable, must be positive.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_formal_verification::formulas::Variable;
    /// let var1 = Variable::new(30);
    /// println!("var1 is {}", var1);
    /// ```
    pub fn new(variable_number: u32) -> Self {
        debug_assert!(
            variable_number <= (u32::MAX >> 1),
            "Variable number is too big."
        );
        Self { variable_number }
    }

    /// Returns a .the number of the variable
    ///
    /// # Arguments
    ///
    /// * `&self` - The variable.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust_formal_verification::formulas::Variable;
    /// let var1 = Variable::new(30);
    /// assert_eq!(var1.get_number(), 30);
    /// ```
    pub fn get_number(&self) -> u32 {
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
