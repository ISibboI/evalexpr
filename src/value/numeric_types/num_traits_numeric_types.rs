use std::marker::PhantomData;

/// See [`EvalexprNumericTypes`].
///
/// This empty struct uses the given type parameters as int and float types.
/// Note that the type parameters need to fulfil the right set of `num-traits` traits for this type to implement `EvalexprNumericTypes`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NumTraitsNumericTypes<Int, Float> {
    phantom_data: PhantomData<(Int, Float)>,
}
