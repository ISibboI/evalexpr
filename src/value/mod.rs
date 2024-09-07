use std::cmp::Ordering;
use std::convert::TryInto;
use std::fmt;
use crate::error::{EvalexprError, EvalexprResult};
use std::hash::{Hash, Hasher};
mod display;
pub mod value_type;

/// The type used to represent integers in `Value::Int`.
pub type IntType = i64;

/// The type used to represent floats in `Value::Float`.
pub type FloatType = f64;

/// The type used to represent tuples in `Value::Tuple`.
pub type TupleType = Vec<Value>;

/// The type used to represent empty values in `Value::Empty`.
pub type EmptyType = ();

/// The value of the empty type to be used in rust.
pub const EMPTY_VALUE: () = ();

/// The value type used by the parser.
/// Values can be of different subtypes that are the variants of this enum.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[repr(C)]
pub enum Value {
    /// A string value.
    String(CowData),
    /// A float value.
    Float(FloatType),
    /// An integer value.
    Int(IntType),
    /// A boolean value.
    Boolean(bool),
    /// A tuple value.
    Tuple(TupleType),
    /// An empty value.
    Empty,
}


// Implement PartialEq for CowData<String> and &str
impl PartialEq<str> for CowData {
    fn eq(&self, other: &str) -> bool {
        unsafe { self.as_ref() == other }
    }
}

// Implement PartialEq for &str and CowData<String>
impl PartialEq<CowData> for str {
    fn eq(&self, other: &CowData) -> bool {
        other == self
    }
}

#[cfg(feature = "serde_json_support")]
impl From<CowData> for serde_json::Value {
    fn from(value: CowData) -> Self {
        Self::String(value.into_owned())
    }
}

impl Value{
    pub fn into_owned(self) -> Value {
        match self {
            Value::String(s) => Value::String(s.into_owned().into()),
            v => v
        }
    }
}


/// A helper enum for handling owned data or references with raw pointers.
#[derive(Clone)]
pub enum CowData{
    Owned(Box<Pin<String>>),
    Borrowed {
        data: *const u8,
        length: usize,
    },
}


impl From<CowData> for String {
    fn from(cow_data: CowData) -> String {
        match cow_data {
            CowData::Owned(s) => Pin::into_inner(*s),
            CowData::Borrowed { data, length, .. } => {
                // Safely convert the borrowed data to a String
                unsafe {
                    let slice = std::slice::from_raw_parts(data, length);
                    std::str::from_utf8(slice).unwrap().to_string()
                }
            }
        }
    }
}

impl DeepSizeOf for CowData{
    fn deep_size_of_children(&self, context: &mut Context) -> usize {
        return match self {
            CowData::Owned(data) => data.deep_size_of_children(context),
            CowData::Borrowed{data,length} => 0,
        }
    }
}

impl FromStr for CowData{
    type Err = EvalexprError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CowData::Owned(Box::new(Pin::new(s.to_string()))))
    }
}
unsafe impl Send for CowData{}
unsafe impl Sync for CowData{}

impl Hash for CowData
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            CowData::Owned(ref data) => data.hash(state),
            CowData::Borrowed{data,length} => unsafe {
                let slice = std::slice::from_raw_parts(*data, *length);
                std::str::from_utf8_unchecked(slice).hash(state)
            },
        }
    }
}



impl fmt::Display for CowData

{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CowData::Owned(ref data) => write!(f, "{}", data),
            CowData::Borrowed{data,length} => unsafe {
                let slice = std::slice::from_raw_parts(*data, *length);
                write!(f, "{:?}", std::str::from_utf8_unchecked(slice)) 
            
            },
        }
    }
}

impl CowData {
    /// Access the data, either as a reference or as a mutable reference if it's owned.
    pub unsafe fn as_ref(&self) -> &str {
        match self {
            CowData::Owned(ref data) => data,
            CowData::Borrowed{data,length} => {
                let slice = std::slice::from_raw_parts(*data, *length);
                std::str::from_utf8_unchecked(slice)
            }
        }
    }
    
    pub fn len(&self) -> usize {
        match self {
            CowData::Owned(ref data) => data.len(),
            CowData::Borrowed{length, ..} => *length,
        }
    }

    /// Convert to an owned version, cloning the data if it was borrowed.
    pub fn into_owned(self) -> String
    {
        match self {
            CowData::Owned(data) => Pin::into_inner(*data),
            CowData::Borrowed{data,length} => unsafe {
                let slice = std::slice::from_raw_parts(data, length);
                std::str::from_utf8_unchecked(slice).to_string()
            },
        }
    }    pub fn ref_into_owned(&self) -> String
    
    {
        match self {
            CowData::Owned(data) => data.to_string(),
            CowData::Borrowed{data,length} => unsafe {
                let slice = std::slice::from_raw_parts(*data, *length);
                std::str::from_utf8_unchecked(slice).to_string()
            },
        }
    }
}


impl fmt::Debug for CowData

{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CowData::Owned(ref data) => {
                f.debug_tuple("Owned")
                    .field(data)
                    .finish()
            },
            CowData::Borrowed{data,length} => {
                // Attempt to safely print the borrowed data
              
                unsafe {
                    let slice = std::slice::from_raw_parts(*data, *length);
                    let value = std::str::from_utf8_unchecked(slice).to_string();
                    f.debug_tuple("Borrowed")
                        .field(&value)
                        .finish()
                }
            },
        }
    }
}

impl Serialize for CowData
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CowData::Owned(ref data) => data.serialize(serializer),
            CowData::Borrowed { data,length}=> unsafe {
                let slice = std::slice::from_raw_parts(*data, *length);
                let value = std::str::from_utf8_unchecked(slice).to_string();
                value.serialize(serializer) 
            
            },
        }
    }
}

impl<'de> Deserialize<'de> for CowData
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let owned_data = String::deserialize(deserializer)?;
        Ok(CowData::Owned(Box::new(Pin::new(owned_data))))
    }
}


impl Eq for Value {}

// Implement Hash for Value
impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::String(s) => {
                s.hash(state);
            }
            Value::Float(f) => {
                OrderedFloat::from(*f).to_bits().hash(state); // Hash the bit representation of the float
            }
            Value::Int(i) => {
                i.hash(state);
            }
            Value::Boolean(b) => {
                b.hash(state);
            }
            Value::Tuple(t) => {
                t.hash(state);
            }
            Value::Empty => {
                // Use a constant to represent the Empty variant
                std::mem::discriminant(self).hash(state);
            }
        }
    }
}


impl From<String> for CowData {
    fn from(s: String) -> Self {
        CowData::Owned(Box::new(Pin::new(s)))
    }
}

impl From<&str> for CowData {
    fn from(s: &str) -> Self {
        CowData::Owned(Box::new(Pin::new(s.to_string())))
    }
}


impl From<&Value> for Value {
    fn from(value: &Value) -> Self {
        match value {
            Value::String(s) => Value::String(s.clone()),
            Value::Float(f) => Value::Float(*f),
            Value::Int(i) => Value::Int(*i),
            Value::Boolean(b) => Value::Boolean(*b),
            Value::Tuple(t) => Value::Tuple(t.iter().map(|v| v.into()).collect()),
            Value::Empty => Value::Empty,
        }
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        // Assuming that `partial_cmp` should never return `None` for `Ord` types
        self.partial_cmp(other).expect(format!("Cannot compare {:?} and {:?}", self, other).as_str())
    }
}


impl Ord for CowData {
    fn cmp(&self, other: &Self) -> Ordering {
        unsafe { self.as_ref().cmp(other.as_ref()) }
    } 
}

impl PartialOrd for CowData
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unsafe { self.as_ref().partial_cmp(other.as_ref()) }
    }
}

impl Eq for CowData {}
    
impl PartialEq for CowData
{
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.as_ref() == other.as_ref() }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            (Value::Float(a), Value::Float(b)) => OrderedFloat::from(*a as FloatType).partial_cmp(&OrderedFloat::from(*b as FloatType)),
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            (Value::Float(a), Value::Int(b)) => OrderedFloat::from(*a as FloatType).partial_cmp(&OrderedFloat::from((*b as FloatType))),
            (Value::Int(a), Value::Float(b)) => (OrderedFloat::from(*a as FloatType)).partial_cmp(&OrderedFloat::from(*b as FloatType)),
            (Value::Boolean(a), Value::Boolean(b)) => a.partial_cmp(b),
            // For simplicity, Tuple and Empty comparisons are not implemented
            // Implementing tuple comparison would require comparing each element of the tuple, which is beyond this simple example
            (Value::Tuple(_), Value::Tuple(_)) => None,
            (Value::Empty, Value::Empty) => Some(Ordering::Equal),
            (_, Value::Empty) => Some(Ordering::Greater),
            (Value::Empty, _) => Some(Ordering::Greater),
            // All other combinations are considered incomparable
            _ => None,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => OrderedFloat::from(*a) == OrderedFloat::from(*b),
            (Value::Float(a), Value::Int(b)) => OrderedFloat::from(*a) == OrderedFloat::from(*b as FloatType),
            (Value::Int(a), Value::Float(b)) => OrderedFloat::from(*a as FloatType) == OrderedFloat::from(*b),
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            // For simplicity, Tuple and Empty equality checks are not fully implemented
            (Value::Tuple(_), Value::Tuple(_)) => false, // Simplified; real implementation would require element-wise comparison
            (Value::Empty, Value::Empty) => true,
            (_, Value::Empty) => false,
            (Value::Empty, _) => false,
            (left,right) => panic!("Cannot compare {:?} and {:?}", left, right),
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Empty
    }
}

impl TryInto<bool> for &Value{
    type Error = Error;

    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            Value::Boolean(b) => Ok(b.clone()),
            value => Err(EvalexprError::expected_boolean(value.clone()).into()),
        }
    }
}

impl Value {
    /// Returns true if `self` is a `Value::String`.
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }
    /// Returns true if `self` is a `Value::Int`.
    pub fn is_int(&self) -> bool {
        matches!(self, Value::Int(_))
    }

    /// Returns true if `self` is a `Value::Float`.
    pub fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
    }

    /// Returns true if `self` is a `Value::Int` or `Value::Float`.
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Int(_) | Value::Float(_))
    }

    /// Returns true if `self` is a `Value::Boolean`.
    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }

    /// Returns true if `self` is a `Value::Tuple`.
    pub fn is_tuple(&self) -> bool {
        matches!(self, Value::Tuple(_))
    }

    /// Returns true if `self` is a `Value::Empty`.
    pub fn is_empty(&self) -> bool {
        matches!(self, Value::Empty)
    }

    /// Clones the value stored in `self` as `String`, or returns `Err` if `self` is not a `Value::String`.
    pub fn as_string(&self) -> EvalexprResult<String> {
        match self {
            Value::String(string) => Ok(string.ref_into_owned()),
            value => Err(EvalexprError::expected_string(value.clone())),
        }
    }
 /// Clones the value stored in `self` as `String`, or returns `Err` if `self` is not a `Value::String`.
    pub fn as_string_or_none(&self) -> EvalexprResult<Option<String>> {
        match self {
            Value::String(string) => Ok(Some(string.ref_into_owned())),
            Value::Empty => Ok(None),
            value => Err(EvalexprError::expected_string(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `IntType`, or returns `Err` if `self` is not a `Value::Int`.
    pub fn as_int(&self) -> EvalexprResult<IntType> {
        match self {
            Value::Int(i) => Ok(*i),
            value => Err(EvalexprError::expected_int(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `IntType`, or returns `Err` if `self` is not a `Value::Int`.
    pub fn as_int_or_none(&self) -> EvalexprResult<Option<IntType>> {
        match self {
            Value::Int(i) => Ok(Some(*i)),
            Value::Empty => Ok(None),
            value => Err(EvalexprError::expected_int(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `FloatType`, or returns `Err` if `self` is not a `Value::Float`.
    pub fn as_float(&self) -> EvalexprResult<FloatType> {
        match self {
            Value::Float(f) => Ok(*f),
            Value::Int(i) => Ok(*i as FloatType),
            value => Err(EvalexprError::expected_float(value.clone())),
        }
    }
    /// Clones the value stored in  `self` as `FloatType`, or returns `Err` if `self` is not a `Value::Float`.
    pub fn as_float_or_none(&self) -> EvalexprResult<Option<FloatType>> {
        match self {
            Value::Float(f) => Ok(Some(*f)),
            Value::Int(i) => Ok(Some(*i as FloatType)),
            Value::Empty => Ok(None),
            value => Err(EvalexprError::expected_float(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `FloatType`, or returns `Err` if `self` is not a `Value::Float` or `Value::Int`.
    /// Note that this method silently converts `IntType` to `FloatType`, if `self` is a `Value::Int`.
    pub fn as_number(&self) -> EvalexprResult<FloatType> {
        match self {
            Value::Float(f) => Ok(*f),
            Value::Int(i) => Ok(*i as FloatType),
            value => Err(EvalexprError::expected_number(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `FloatType`, or returns `Err` if `self` is not a `Value::Float` or `Value::Int`.
    /// Note that this method silently converts `IntType` to `FloatType`, if `self` is a `Value::Int`.
    pub fn as_number_or_none(&self) -> EvalexprResult<Option<FloatType>> {
        match self {
            Value::Float(f) => Ok(Some(*f)),
            Value::Int(i) => Ok(Some(*i as FloatType)),
            Value::Empty => Ok(None),
            value => Err(EvalexprError::expected_number(value.clone())),
        }
    }

    /// Clones the value stored in  `self` as `bool`, or returns `Err` if `self` is not a `Value::Boolean`.
    pub fn as_boolean(&self) -> EvalexprResult<bool> {
        match self {
            Value::Boolean(boolean) => Ok(*boolean),
            value => Err(EvalexprError::expected_boolean(value.clone())),
        }
    }
    /// Clones the value stored in  `self` as `bool`, or returns `Err` if `self` is not a `Value::Boolean`.
    pub fn as_boolean_or_none(&self) -> EvalexprResult<Option<bool>> {
        match self {
            Value::Boolean(boolean) => Ok( Some(*boolean)),
            Value::Empty => Ok(None),
            value => Err(EvalexprError::expected_boolean(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `TupleType`, or returns `Err` if `self` is not a `Value::Tuple`.
    pub fn as_tuple(&self) -> EvalexprResult<TupleType> {
        match self {
            Value::Tuple(tuple) => Ok(tuple.clone()),
            value => Err(EvalexprError::expected_tuple(value.clone())),
        }
    }

    /// Clones the value stored in `self` as `TupleType` or returns `Err` if `self` is not a `Value::Tuple` of the required length.
    pub fn as_fixed_len_tuple(&self, len: usize) -> EvalexprResult<TupleType> {
        match self {
            Value::Tuple(tuple) => {
                if tuple.len() == len {
                    Ok(tuple.clone())
                } else {
                    Err(EvalexprError::expected_fixed_len_tuple(len, self.clone()))
                }
            },
            value => Err(EvalexprError::expected_tuple(value.clone())),
        }
    }

    /// Returns `()`, or returns`Err` if `self` is not a `Value::Tuple`.
    pub fn as_empty(&self) -> EvalexprResult<()> {
        match self {
            Value::Empty => Ok(()),
            value => Err(EvalexprError::expected_empty(value.clone())),
        }
    }
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Value::String(CowData::Owned(Box::new(Pin::new(string))))
    }
}

impl From<&str> for Value {
    fn from(string: &str) -> Self {
        Value::String(CowData::Owned(Box::new(Pin::new(string.to_string()))))
    }
}

impl From<FloatType> for Value {
    fn from(float: FloatType) -> Self {
        Value::Float(float)
    }
}

impl From<IntType> for Value {
    fn from(int: IntType) -> Self {
        Value::Int(int)
    }
}

impl From<bool> for Value {
    fn from(boolean: bool) -> Self {
        Value::Boolean(boolean)
    }
}

impl From<TupleType> for Value {
    fn from(tuple: TupleType) -> Self {
        Value::Tuple(tuple)
    }
}

impl From<Value> for EvalexprResult<Value> {
    fn from(value: Value) -> Self {
        Ok(value)
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Empty
    }
}

use std::ops::{Div, Rem};

use std::ops::Mul;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Error {
    UnsupportedArithmeticBetweenTypes,
    UnsupportedOperation,
    DivisionByZero,
    NonNumericType,
    InvalidArgumentType,
    InvalidInputString,
    InvalidDateFormat,
    CustomError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnsupportedArithmeticBetweenTypes => write!(f, "Unsupported arithmetic between types"),
            Error::UnsupportedOperation => write!(f, "Unsupported operation"),
            Error::DivisionByZero => write!(f, "Division by zero"),
            Error::NonNumericType => write!(f, "Non-numeric type"),
            Error::InvalidArgumentType => write!(f, "Invalid argument type"),
            Error::InvalidInputString => write!(f, "Invalid input string"),
            Error::InvalidDateFormat => write!(f, "Invalid date format"),
            Error::CustomError(ref msg) => write!(f, "Custom error: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<EvalexprError> for Error {
    fn from(err: EvalexprError) -> Self {
        Error::CustomError(format!("{}",err))
    }
}

pub trait ToErrorType {
    fn to_error_code(&self) -> i32;
    fn to_error_message(&self) -> Option<String>;
}

impl Error{
    pub fn from_error_code(code: i32, custom_error: Option<String>) -> Self {
        match code {
            1 => Error::UnsupportedOperation,
            2 => Error::DivisionByZero,
            3 => Error::NonNumericType,
            4 => Error::UnsupportedArithmeticBetweenTypes,
            5 => Error::InvalidArgumentType,
            6 => Error::InvalidInputString,
            7 => Error::InvalidDateFormat,
            8 => Error::CustomError(custom_error.unwrap_or("Custom error".to_string())),
            _ => Error::UnsupportedOperation,
        }
    }
}

impl ToErrorType for Error {
    fn to_error_code(&self) -> i32 {
        match self {
            Error::UnsupportedOperation => 1,
            Error::DivisionByZero => 2,
            Error::NonNumericType => 3,
            Error::UnsupportedArithmeticBetweenTypes => 4,
            Error::InvalidArgumentType => 5,
            Error::InvalidInputString => 6,
            Error::InvalidDateFormat => 7,
            Error::CustomError(_) => 8
        }
    }

    fn to_error_message(&self) -> Option<String> {
        match self {
            Error::CustomError(message) => Some(message.clone()),
            _ => None,
        }
    }
}

use std::ops::Sub;

use std::ops::Add;

use std::ops::Neg;
use std::pin::Pin;
use std::str::FromStr;
use deepsize::{Context, DeepSizeOf};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::ordered_float::OrderedFloat;
use crate::value;

impl Neg for Value {
    type Output = Result<Self, Error>;

    fn neg(self) -> Self::Output {
        match self {
            (Value::Empty) => Ok(Value::Empty),
            Value::Int(a) => Ok(Value::Int(-a)),
            Value::Float(a) => Ok(Value::Float(-a)),
            _ => Err(Error::UnsupportedArithmeticBetweenTypes),
        }
    }
}




impl Rem for &Value {
    type Output = Result<Value, Error>;

    fn rem(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Empty, Value::Empty) => Ok(Value::Empty),
            (Value::Empty, _) => Ok(Value::Empty),
            (_, Value::Empty) => Ok(Value::Empty),
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a % b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a % b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float(*a as f64 % b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(*a % *b as f64)),
            _ => Err(Error::UnsupportedArithmeticBetweenTypes),
        }
    }
}

impl Add for &Value {
    type Output = Result<Value, Error>; // Assuming you have an error type defined

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Empty, Value::Empty) => Ok(Value::Empty),
            (Value::Empty, _) => Ok(Value::Empty),
            (_, Value::Empty) => Ok(Value::Empty),
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float(*a + *b as f64)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => Ok(Value::Float(*a as FloatType + b)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b).into())),
            // Handle combinations with strings and numeric types if desired
            (Value::Int(a), Value::String(b)) | (Value::String(b), Value::Int(a)) => Ok(Value::String(format!("{}{}", a, b).into())),
            (Value::Float(a), Value::String(b)) | (Value::String(b), Value::Float(a)) => Ok(Value::String(format!("{}{}", a, b).into())),
            // Add cases for other Value variants as necessary
            _ => Err(Error::UnsupportedArithmeticBetweenTypes),
        }
    }
}


impl Sub for &Value {
    type Output = Result<Value, Error>;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Empty, Value::Empty) => Ok(Value::Empty),
            (Value::Empty, _) => Ok(Value::Empty),
            (_, Value::Empty) => Ok(Value::Empty),
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float((&OrderedFloat::from(*a) - &OrderedFloat::from(*b)).into())),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Float((&OrderedFloat::from(*a as f64) - &OrderedFloat::from(*b)).into())),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Float((&OrderedFloat::from(*a) - &OrderedFloat::from(*b as f64)).into())),
            _ => Err(Error::UnsupportedArithmeticBetweenTypes),
        }
    }
}

impl Mul for &Value {
    type Output = Result<Value, Error>;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Empty, Value::Empty) => Ok(Value::Empty),
            (Value::Empty, _) => Ok(Value::Empty),
            (_, Value::Empty) => Ok(Value::Empty),
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Int(a), Value::Float(b)) | (Value::Float(b), Value::Int(a)) => Ok(Value::Float(*a as f64 * b)),
            _ => Err(Error::UnsupportedArithmeticBetweenTypes),
        }
    }
}

impl Div for &Value {
    type Output = Result<Value, Error>;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Value::Empty, Value::Empty) => Ok(Value::Empty),
            (Value::Empty, _) => Ok(Value::Empty),
            (_, Value::Empty) => Ok(Value::Empty),
            (Value::Int(a), Value::Int(b)) => {
                if b == &0 {
                    Err(Error::DivisionByZero)
                } else {
                    Ok(Value::Int(a / b))
                }
            },
            (Value::Float(a), Value::Float(b)) => {
                if b == &0.0 {
                    Err(Error::DivisionByZero)
                } else {
                    Ok(Value::Float(a / b))
                }
            },
            (Value::Int(a), Value::Float(b)) => {
                if b == &0.0 {
                    Err(Error::DivisionByZero)
                } else {
                    Ok(Value::Float(*a as f64 / b))
                }
            },
            (Value::Float(a), Value::Int(b)) => {
                if b == &0 {
                    Err(Error::DivisionByZero)
                } else {
                    Ok(Value::Float(*a / *b as f64))
                }
            },
            // Add cases for other combinations as needed, returning UnsupportedOperation for non-numeric types
            _ => Err(Error::UnsupportedArithmeticBetweenTypes),
        }
    }
}



#[repr(C)]
pub struct FfiResult<T> {
    /// The value, which will be a default value in case of an error.
    pub value: T,
    /// An integer error code. 0 indicates success, non-zero indicates an error.
    pub error_code: i32,

    pub error_message: String,
}

/// Converts a Rust `Result<T, i32>` to an `FfiResult<T>`, where `T: Default`.
pub fn to_ffi_result<T: Default, E: ToErrorType>(result: Result<T, E>) -> FfiResult<T> {
    match result {
        Ok(value) => FfiResult {
            value,
            error_code: 0, // Indicate success
            error_message: "".to_string(),
        },
        Err(e) => FfiResult {
            value: T::default(),
            error_code : e.to_error_code(), // Use the provided error code
            error_message: format!("{}", e.to_error_message().unwrap_or("".to_string())),
        },
    }
}

pub fn to_nested_ffi_result<T: Default, E: ToErrorType>(
    result: Result<Result<T, E>, Box<dyn std::any::Any + Send + 'static>>,
) -> FfiResult<T> {
    match result {
        Ok(inner_result) => match inner_result {
            Ok(value) => FfiResult {
                value,
                error_code: 0, // Indicate success
                error_message: "".to_string(),
            },
            Err(e) => FfiResult {
                value: T::default(),
                error_code: e.to_error_code(),
                error_message: e.to_error_message().unwrap_or_else(|| "".to_string()),
            },
        },
        Err(panic_info) => {
            let error_message = if let Some(s) = panic_info.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_info.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic".to_string()
            };

            FfiResult {
                value: T::default(),
                error_code: 8,
                error_message,
            }
        }
    }
}





macro_rules! declare_arithmetic_for_result {
    ($trait:ident, $fn:ident) => {
        impl std::ops::$trait<Value> for Result<Value, Error> {
            type Output = Result<Value, Error>;

            fn $fn(self, other: Value) -> Self::Output {
                match self {
                    Ok(ref self_val) => self_val.$fn(&other),
                    Err(e) => Err(e),
                }
            }
        }

        impl std::ops::$trait<Result<Value, Error>> for Value {
            type Output = Result<Value, Error>;

            fn $fn(self, other: Result<Value, Error>) -> Self::Output {
                match other {
                    Ok(ref other_val) => (&self).$fn(other_val),
                    Err(e) => Err(e),
                }
            }
        }

        impl std::ops::$trait for Value {
            type Output = Result<Value, Error>;

            fn $fn(self, other: Self) -> Self::Output {
                (&self).$fn(&other)
            }
        }

        impl std::ops::$trait<&Value> for Value {
            type Output = Result<Value, Error>;
            fn $fn(self, other: &Self) -> Self::Output {
                 (&self).$fn(other)
            }
        }

        impl std::ops::$trait<Value> for &Value {
            type Output = Result<Value, Error>;
            fn $fn(self, other: Value) -> Self::Output {
                 self.$fn(&other)
            }
        }

        impl std::ops::$trait<Result<Value, Error>> for &Value {
            type Output = Result<Value, Error>;

            fn $fn(self, other: Result<Value, Error>) -> Self::Output {
                match other {
                    Ok(ref other_val) => self.$fn(other_val),
                    Err(e) => Err(e),
                }
            }
        }

    };
}



declare_arithmetic_for_result!(Rem, rem);
declare_arithmetic_for_result!(Add, add);
declare_arithmetic_for_result!(Sub, sub);
declare_arithmetic_for_result!(Mul, mul);
declare_arithmetic_for_result!(Div, div);


#[cfg(test)]
mod tests {
    use crate::value::{TupleType, Value};
    use super::*;
    #[test]
    fn test_value_conversions() {
        assert_eq!(
            Value::from("string").as_string(),
            Ok(String::from("string"))
        );
        assert_eq!(Value::from(3).as_int(), Ok(3));
        assert_eq!(Value::from(3.3).as_float(), Ok(3.3));
        assert_eq!(Value::from(true).as_boolean(), Ok(true));
        assert_eq!(
            Value::from(TupleType::new()).as_tuple(),
            Ok(TupleType::new())
        );
    }

    #[test]
    fn test_value_checks() {
        assert!(Value::from("string").is_string());
        assert!(Value::from(3).is_int());
        assert!(Value::from(3.3).is_float());
        assert!(Value::from(true).is_boolean());
        assert!(Value::from(TupleType::new()).is_tuple());
    }

    #[test]
    fn test_add_integers() {
        let a = Value::Int(10);
        let b = Value::Int(20);
        // Unwrap the result to compare the value directly
        assert_eq!(a.add(b).unwrap(), Value::Int(30));
    }

    #[test]
    fn test_add_integers_to_add() {
        let a = Value::Int(10);
        let b = Value::Int(20);
        let c = Value::Int(20);
        // Unwrap the result to compare the value directly
        assert_eq!(a.add(b).add(c).unwrap(), Value::Int(50));
    }

    #[test]
    fn test_subtract_floats() {
        let a = Value::Float(20.5);
        let b = Value::Float(10.25);
        // Unwrap the result to compare the value directly
        assert_eq!(a.sub(b).unwrap(), Value::Float(10.25));
    }

    #[test]
    fn test_multiply_int_float() {
        let a = Value::Int(2);
        let b = Value::Float(3.5);
        // Unwrap the result to compare the value directly
        assert_eq!(a.mul(b).unwrap(), Value::Float(7.0));
    }

    #[test]
    fn test_divide_float_by_int() {
        let a = Value::Float(10.0);
        let b = Value::Int(2);
        // Unwrap the result to compare the value directly
        assert_eq!(a.div(b).unwrap(), Value::Float(5.0));
    }

    #[test]
    fn test_integer_remainder() {
        let a = Value::Int(10);
        let b = Value::Int(4);
        // Unwrap the result to compare the value directly
        assert_eq!(a.rem(b).unwrap(), Value::Int(2));
    }

    #[test]
    fn test_error_on_divide_by_zero() {
        let a = Value::Int(10);
        let b = Value::Int(0);
        // Here, we expect an error, so no unwrap is needed
        assert!(matches!(a.div(b), Err(Error::DivisionByZero)));
    }
 #[test]
    fn test_error_divide_zero_by_something() {
        let a = Value::Int(10);
        let b = Value::Int(0);
        // Here, we expect an error, so no unwrap is needed
        assert!(matches!(b.div(a).unwrap(),  Value::Int(0)));
    }


        #[test]
        fn test_add_integers_with_refs() {
            let a = Value::Int(10);
            let b = Value::Int(20);
            // Using references in the add operation
            assert_eq!((&a).add(&b).unwrap(), Value::Int(30));
        }

        #[test]
        fn test_add_integers_to_add_with_refs() {
            let a = Value::Int(10);
            let b = Value::Int(20);
            let c = Value::Int(20);
            // Using references in chained add operations
            assert_eq!((&a).add(&b).unwrap().add(&c).unwrap(), Value::Int(50));
        }

        #[test]
        fn test_subtract_floats_with_refs() {
            let a = Value::Float(20.5);
            let b = Value::Float(10.25);
            // Using references in the sub operation
            assert_eq!((&a).sub(&b).unwrap(), Value::Float(10.25));
        }

        #[test]
        fn test_multiply_int_float_with_refs() {
            let a = Value::Int(2);
            let b = Value::Float(3.5);
            // Using references in the mul operation
            assert_eq!((&a).mul(&b).unwrap(), Value::Float(7.0));
        }

        #[test]
        fn test_divide_float_by_int_with_refs() {
            let a = Value::Float(10.0);
            let b = Value::Int(2);
            // Using references in the div operation
            assert_eq!((&a).div(&b).unwrap(), Value::Float(5.0));
        }

        #[test]
        fn test_integer_remainder_with_refs() {
            let a = Value::Int(10);
            let b = Value::Int(4);
            // Using references in the rem operation
            assert_eq!((&a).rem(&b).unwrap(), Value::Int(2));
        }

        #[test]
        fn test_error_on_divide_by_zero_with_refs() {
            let a = Value::Int(10);
            let b = Value::Int(0);
            // Using references, expecting an error on division by zero
            assert!(matches!((&a).div(&b), Err(Error::DivisionByZero)));
        }

    #[test]
    fn test_add_ref_and_value() {
        let a = Value::Int(10);
        let b = Value::Int(20);
        // Reference on the left, value on the right
        assert_eq!((&a).add(b.clone()).unwrap(), Value::Int(30));
        // Value on the left, reference on the right
        assert_eq!(a.add(&b).unwrap(), Value::Int(30));
    }

    #[test]
    fn test_subtract_ref_and_value() {
        let a = Value::Float(20.5);
        let b = Value::Float(10.25);
        // Reference on the left, value on the right
        assert_eq!((&a).sub(b.clone()).unwrap(), Value::Float(10.25));
        // Value on the left, reference on the right
        assert_eq!(a.sub(&b).unwrap(), Value::Float(10.25));
    }

    #[test]
    fn test_multiply_ref_and_value() {
        let a = Value::Int(2);
        let b = Value::Float(3.5);
        // Reference on the left, value on the right
        assert_eq!((&a).mul(b.clone()).unwrap(), Value::Float(7.0));
        // Value on the left, reference on the right
        assert_eq!(a.mul(&b).unwrap(), Value::Float(7.0));
    }

    #[test]
    fn test_divide_ref_and_value() {
        let a = Value::Float(10.0);
        let b = Value::Int(2);
        // Reference on the left, value on the right
        assert_eq!((&a).div(b.clone()).unwrap(), Value::Float(5.0));
        // Value on the left, reference on the right
        assert_eq!(a.div(&b).unwrap(), Value::Float(5.0));
    }

    #[test]
    fn test_remainder_ref_and_value() {
        let a = Value::Int(10);
        let b = Value::Int(4);
        // Reference on the left, value on the right
        assert_eq!((&a).rem(b.clone()).unwrap(), Value::Int(2));
        // Value on the left, reference on the right
        assert_eq!(a.rem(&b).unwrap(), Value::Int(2));
    }

    #[test]
    fn test_error_on_divide_by_zero_ref_and_value() {
        let a = Value::Int(10);
        let b = Value::Int(0);
        // Reference on the left, value on the right
        assert!(matches!((&a).div(b.clone()), Err(Error::DivisionByZero)));
        // Value on the left, reference on the right
        assert!(matches!(a.div(&b), Err(Error::DivisionByZero)));
    }

    }

