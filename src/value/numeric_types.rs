pub trait EvalexprNumericTypes {
    type Int: Clone;
    type Float: Clone;

    /// Convert an integer to a float using the `as` operator or a similar mechanic.
    fn int_as_float(int: &Self::Int) -> Self::Float;

    /// Convert a float to an integer using the `as` operator or a similar mechanic.
    fn float_as_int(float: &Self::Float) -> Self::Int;
}

pub struct DefaultNumericTypes;

impl EvalexprNumericTypes for DefaultNumericTypes {
    type Int = i64;

    type Float = f64;

    fn int_as_float(int: &Self::Int) -> Self::Float {
        int as Self::Float
    }

    fn float_as_int(float: &Self::Float) -> Self::Int {
        float as Self::Int
    }
}
