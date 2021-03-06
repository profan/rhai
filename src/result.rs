//! Module containing error definitions for the evaluation process.

use crate::any::Dynamic;
use crate::error::ParseError;
use crate::parser::{Position, INT};

use crate::stdlib::{
    error::Error,
    fmt,
    string::{String, ToString},
};

#[cfg(not(feature = "no_std"))]
use crate::stdlib::path::PathBuf;

/// Evaluation result.
///
/// All wrapped `Position` values represent the location in the script where the error occurs.
#[derive(Debug)]
pub enum EvalAltResult {
    /// Syntax error.
    ErrorParsing(ParseError),
    /// Call to an unknown function. Wrapped value is the name of the function.
    ErrorFunctionNotFound(String, Position),
    /// Function call has incorrect number of arguments.
    /// Wrapped values are the name of the function, the number of parameters required
    /// and the actual number of arguments passed.
    ErrorFunctionArgsMismatch(String, usize, usize, Position),
    /// Non-boolean operand encountered for boolean operator. Wrapped value is the operator.
    ErrorBooleanArgMismatch(String, Position),
    /// Non-character value encountered where a character is required.
    ErrorCharMismatch(Position),
    /// Array access out-of-bounds.
    /// Wrapped values are the current number of elements in the array and the index number.
    ErrorArrayBounds(usize, INT, Position),
    /// String indexing out-of-bounds.
    /// Wrapped values are the current number of characters in the string and the index number.
    ErrorStringBounds(usize, INT, Position),
    /// Trying to index into a type that is not an array and not a string.
    ErrorIndexingType(String, Position),
    /// Trying to index into an array or string with an index that is not `i64`.
    ErrorIndexExpr(Position),
    /// The guard expression in an `if` or `while` statement does not return a boolean value.
    ErrorLogicGuard(Position),
    /// The `for` statement encounters a type that is not an iterator.
    ErrorFor(Position),
    /// Usage of an unknown variable. Wrapped value is the name of the variable.
    ErrorVariableNotFound(String, Position),
    /// Assignment to an inappropriate LHS (left-hand-side) expression.
    ErrorAssignmentToUnknownLHS(Position),
    /// Assignment to a constant variable.
    ErrorAssignmentToConstant(String, Position),
    /// Returned type is not the same as the required output type.
    /// Wrapped value is the type of the actual result.
    ErrorMismatchOutputType(String, Position),
    /// Error reading from a script file. Wrapped value is the path of the script file.
    #[cfg(not(feature = "no_std"))]
    ErrorReadingScriptFile(PathBuf, std::io::Error),
    /// Inappropriate member access.
    ErrorDotExpr(String, Position),
    /// Arithmetic error encountered. Wrapped value is the error message.
    ErrorArithmetic(String, Position),
    /// Call stack over maximum limit.
    ErrorStackOverflow(Position),
    /// Run-time error encountered. Wrapped value is the error message.
    ErrorRuntime(String, Position),
    /// Breaking out of loops - not an error if within a loop.
    ErrorLoopBreak(Position),
    /// Not an error: Value returned from a script via the `return` keyword.
    /// Wrapped value is the result value.
    Return(Dynamic, Position),
}

impl EvalAltResult {
    pub(crate) fn desc(&self) -> &str {
        match self {
            Self::ErrorParsing(p) => p.desc(),
            Self::ErrorFunctionNotFound(_, _) => "Function not found",
            Self::ErrorFunctionArgsMismatch(_, _, _, _) => {
                "Function call with wrong number of arguments"
            }
            Self::ErrorBooleanArgMismatch(_, _) => "Boolean operator expects boolean operands",
            Self::ErrorCharMismatch(_) => "Character expected",
            Self::ErrorIndexExpr(_) => "Indexing into an array or string expects an integer index",
            Self::ErrorIndexingType(_, _) => {
                "Indexing can only be performed on an array or a string"
            }
            Self::ErrorArrayBounds(_, index, _) if *index < 0 => {
                "Array access expects non-negative index"
            }
            Self::ErrorArrayBounds(0, _, _) => "Access of empty array",
            Self::ErrorArrayBounds(_, _, _) => "Array index out of bounds",
            Self::ErrorStringBounds(_, index, _) if *index < 0 => {
                "Indexing a string expects a non-negative index"
            }
            Self::ErrorStringBounds(0, _, _) => "Indexing of empty string",
            Self::ErrorStringBounds(_, _, _) => "String index out of bounds",
            Self::ErrorLogicGuard(_) => "Boolean expression expected",
            Self::ErrorFor(_) => "For loop expects array or range",
            Self::ErrorVariableNotFound(_, _) => "Variable not found",
            Self::ErrorAssignmentToUnknownLHS(_) => {
                "Assignment to an unsupported left-hand side expression"
            }
            Self::ErrorAssignmentToConstant(_, _) => "Assignment to a constant variable",
            Self::ErrorMismatchOutputType(_, _) => "Output type is incorrect",
            #[cfg(not(feature = "no_std"))]
            Self::ErrorReadingScriptFile(_, _) => "Cannot read from script file",
            Self::ErrorDotExpr(_, _) => "Malformed dot expression",
            Self::ErrorArithmetic(_, _) => "Arithmetic error",
            Self::ErrorStackOverflow(_) => "Stack overflow",
            Self::ErrorRuntime(_, _) => "Runtime error",
            Self::ErrorLoopBreak(_) => "Break statement not inside a loop",
            Self::Return(_, _) => "[Not Error] Function returns value",
        }
    }
}

impl Error for EvalAltResult {}

impl fmt::Display for EvalAltResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let desc = self.desc();

        match self {
            Self::ErrorFunctionNotFound(s, pos) => write!(f, "{}: '{}' ({})", desc, s, pos),
            Self::ErrorVariableNotFound(s, pos) => write!(f, "{}: '{}' ({})", desc, s, pos),
            Self::ErrorIndexingType(_, pos) => write!(f, "{} ({})", desc, pos),
            Self::ErrorIndexExpr(pos) => write!(f, "{} ({})", desc, pos),
            Self::ErrorLogicGuard(pos) => write!(f, "{} ({})", desc, pos),
            Self::ErrorFor(pos) => write!(f, "{} ({})", desc, pos),
            Self::ErrorAssignmentToUnknownLHS(pos) => write!(f, "{} ({})", desc, pos),
            Self::ErrorAssignmentToConstant(s, pos) => write!(f, "{}: '{}' ({})", desc, s, pos),
            Self::ErrorMismatchOutputType(s, pos) => write!(f, "{}: {} ({})", desc, s, pos),
            Self::ErrorDotExpr(s, pos) if !s.is_empty() => write!(f, "{} {} ({})", desc, s, pos),
            Self::ErrorDotExpr(_, pos) => write!(f, "{} ({})", desc, pos),
            Self::ErrorArithmetic(s, pos) => write!(f, "{} ({})", s, pos),
            Self::ErrorStackOverflow(pos) => write!(f, "{} ({})", desc, pos),
            Self::ErrorRuntime(s, pos) => {
                write!(f, "{} ({})", if s.is_empty() { desc } else { s }, pos)
            }
            Self::ErrorLoopBreak(pos) => write!(f, "{} ({})", desc, pos),
            Self::Return(_, pos) => write!(f, "{} ({})", desc, pos),
            #[cfg(not(feature = "no_std"))]
            Self::ErrorReadingScriptFile(path, err) => {
                write!(f, "{} '{}': {}", desc, path.display(), err)
            }
            Self::ErrorParsing(p) => write!(f, "Syntax error: {}", p),
            Self::ErrorFunctionArgsMismatch(fun, 0, n, pos) => write!(
                f,
                "Function '{}' expects no argument but {} found ({})",
                fun, n, pos
            ),
            Self::ErrorFunctionArgsMismatch(fun, 1, n, pos) => write!(
                f,
                "Function '{}' expects one argument but {} found ({})",
                fun, n, pos
            ),
            Self::ErrorFunctionArgsMismatch(fun, need, n, pos) => write!(
                f,
                "Function '{}' expects {} argument(s) but {} found ({})",
                fun, need, n, pos
            ),
            Self::ErrorBooleanArgMismatch(op, pos) => {
                write!(f, "{} operator expects boolean operands ({})", op, pos)
            }
            Self::ErrorCharMismatch(pos) => {
                write!(f, "string indexing expects a character value ({})", pos)
            }
            Self::ErrorArrayBounds(_, index, pos) if *index < 0 => {
                write!(f, "{}: {} < 0 ({})", desc, index, pos)
            }
            Self::ErrorArrayBounds(0, _, pos) => write!(f, "{} ({})", desc, pos),
            Self::ErrorArrayBounds(1, index, pos) => write!(
                f,
                "Array index {} is out of bounds: only one element in the array ({})",
                index, pos
            ),
            Self::ErrorArrayBounds(max, index, pos) => write!(
                f,
                "Array index {} is out of bounds: only {} elements in the array ({})",
                index, max, pos
            ),
            Self::ErrorStringBounds(_, index, pos) if *index < 0 => {
                write!(f, "{}: {} < 0 ({})", desc, index, pos)
            }
            Self::ErrorStringBounds(0, _, pos) => write!(f, "{} ({})", desc, pos),
            Self::ErrorStringBounds(1, index, pos) => write!(
                f,
                "String index {} is out of bounds: only one character in the string ({})",
                index, pos
            ),
            Self::ErrorStringBounds(max, index, pos) => write!(
                f,
                "String index {} is out of bounds: only {} characters in the string ({})",
                index, max, pos
            ),
        }
    }
}

impl From<ParseError> for EvalAltResult {
    fn from(err: ParseError) -> Self {
        Self::ErrorParsing(err)
    }
}

impl<T: AsRef<str>> From<T> for EvalAltResult {
    fn from(err: T) -> Self {
        Self::ErrorRuntime(err.as_ref().to_string(), Position::none())
    }
}

impl EvalAltResult {
    pub fn position(&self) -> Position {
        match self {
            #[cfg(not(feature = "no_std"))]
            Self::ErrorReadingScriptFile(_, _) => Position::none(),

            Self::ErrorParsing(err) => err.position(),

            Self::ErrorFunctionNotFound(_, pos)
            | Self::ErrorFunctionArgsMismatch(_, _, _, pos)
            | Self::ErrorBooleanArgMismatch(_, pos)
            | Self::ErrorCharMismatch(pos)
            | Self::ErrorArrayBounds(_, _, pos)
            | Self::ErrorStringBounds(_, _, pos)
            | Self::ErrorIndexingType(_, pos)
            | Self::ErrorIndexExpr(pos)
            | Self::ErrorLogicGuard(pos)
            | Self::ErrorFor(pos)
            | Self::ErrorVariableNotFound(_, pos)
            | Self::ErrorAssignmentToUnknownLHS(pos)
            | Self::ErrorAssignmentToConstant(_, pos)
            | Self::ErrorMismatchOutputType(_, pos)
            | Self::ErrorDotExpr(_, pos)
            | Self::ErrorArithmetic(_, pos)
            | Self::ErrorStackOverflow(pos)
            | Self::ErrorRuntime(_, pos)
            | Self::ErrorLoopBreak(pos)
            | Self::Return(_, pos) => *pos,
        }
    }

    /// Consume the current `EvalAltResult` and return a new one
    /// with the specified `Position`.
    pub(crate) fn set_position(mut self, new_position: Position) -> Self {
        match self {
            #[cfg(not(feature = "no_std"))]
            Self::ErrorReadingScriptFile(_, _) => (),

            Self::ErrorParsing(ParseError(_, ref mut pos))
            | Self::ErrorFunctionNotFound(_, ref mut pos)
            | Self::ErrorFunctionArgsMismatch(_, _, _, ref mut pos)
            | Self::ErrorBooleanArgMismatch(_, ref mut pos)
            | Self::ErrorCharMismatch(ref mut pos)
            | Self::ErrorArrayBounds(_, _, ref mut pos)
            | Self::ErrorStringBounds(_, _, ref mut pos)
            | Self::ErrorIndexingType(_, ref mut pos)
            | Self::ErrorIndexExpr(ref mut pos)
            | Self::ErrorLogicGuard(ref mut pos)
            | Self::ErrorFor(ref mut pos)
            | Self::ErrorVariableNotFound(_, ref mut pos)
            | Self::ErrorAssignmentToUnknownLHS(ref mut pos)
            | Self::ErrorAssignmentToConstant(_, ref mut pos)
            | Self::ErrorMismatchOutputType(_, ref mut pos)
            | Self::ErrorDotExpr(_, ref mut pos)
            | Self::ErrorArithmetic(_, ref mut pos)
            | Self::ErrorStackOverflow(ref mut pos)
            | Self::ErrorRuntime(_, ref mut pos)
            | Self::ErrorLoopBreak(ref mut pos)
            | Self::Return(_, ref mut pos) => *pos = new_position,
        }

        self
    }
}
