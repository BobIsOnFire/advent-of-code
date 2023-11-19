pub type GenericResult<T> = Result<T, Box<dyn std::error::Error>>;

mod array_stack;
pub use array_stack::ArrayStack;

mod bitset;
pub use bitset::BitSet;

pub mod iter;

pub mod lexer;
pub use lexer::Lexer;

mod number_range;
pub use number_range::NumberRange;

mod vecmatrix;
pub use vecmatrix::{MatrixIndex, VecMatrix};
