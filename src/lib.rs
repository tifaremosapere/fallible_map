/// `fallible_map_ext` provides utilities for fallible mapping over `Option`
/// types and iterators, allowing the use of functions that can return `Result`s.

/// A helper trait to extract the inner value of an optional container.
pub trait ExtractOption<T> {
    /// Extract the inner value as an `Option`.
    fn extract(self) -> Option<T>;
}

/// Implementation of `ExtractOption` for `Option`.
impl<T> ExtractOption<T> for Option<T> {
    fn extract(self) -> Option<T> {
        self
    }
}

/// Extend `Option` with fallible methods.
///
/// Useful for mapping fallible operations (i.e., operations that return `Result`),
/// over an optional type. The result will be `Result<Option<U>>`, making it easy
/// to handle errors originating from inside the closure being mapped.
///
/// # Type Parameters
///
/// - `C`: The container type that implements `ExtractOption`
/// - `T`: The input container's value type
/// - `U`: The output container's value type
/// - `E`: The possible error type during the mapping
pub trait FallibleMapExt<T, U, E> {
    /// Attempt to map a function over an optional value.
    ///
    /// # Parameters
    ///
    /// - `f`: A function that takes a value of type `T` and returns a `Result<U, E>`.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<U>`, or an error `E`.
    fn try_map<F>(self, f: F) -> Result<Option<U>, E>
    where
        F: FnOnce(T) -> Result<U, E>;

    /// Unwrap an optional value or compute a fallback.
    ///
    /// # Parameters
    ///
    /// - `f`: A function that returns a `Result<T, E>`.
    ///
    /// # Returns
    ///
    /// A `Result` containing a value of type `T`, or an error `E`.
    fn try_unwrap_or<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>;

    /// Chain computation that returns another optional value.
    ///
    /// # Parameters
    ///
    /// - `f`: A function that takes a value of type `T` and returns a `Result<Option<U>, E>`.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<U>`, or an error `E`.
    fn try_and_then<F>(self, f: F) -> Result<Option<U>, E>
    where
        F: FnOnce(T) -> Result<Option<U>, E>;
}

/// Implementation of `FallibleMapExt` for types implementing `ExtractOption`.
impl<C, T, U, E> FallibleMapExt<T, U, E> for C
where
    C: ExtractOption<T>,
{
    fn try_map<F>(self, f: F) -> Result<Option<U>, E>
    where
        F: FnOnce(T) -> Result<U, E>,
    {
        match self.extract() {
            Some(x) => f(x).map(Some),
            None => Ok(None),
        }
    }

    fn try_unwrap_or<F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce() -> Result<T, E>,
    {
        match self.extract() {
            Some(x) => Ok(x),
            None => f(),
        }
    }

    fn try_and_then<F>(self, f: F) -> Result<Option<U>, E>
    where
        F: FnOnce(T) -> Result<Option<U>, E>,
    {
        match self.extract() {
            Some(x) => f(x),
            None => Ok(None),
        }
    }
}

/// Extend iterator with fallible map functionality.
///
/// This trait provides a fallible version of the `map` method, allowing
/// the use of functions that return `Result`s during iteration.
///
/// # Example
///
/// ```
/// use my_crate::TryMapIteratorExt;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let result: Result<Vec<_>, _> = numbers.into_iter().try_map(|num| {
///     if num % 2 == 0 {
///         Ok(num * 2)
///     } else {
///         Err("Odd number")
///     }
/// });
/// assert_eq!(result, Err("Odd number"));
/// ```
pub trait TryMapIteratorExt: Iterator {
    /// Attempt to map a function over an iterator.
    ///
    /// # Parameters
    ///
    /// - `f`: A function that takes an item and returns a `Result<B, E>`.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec<B>`, or an error `E`.
    fn try_map<B, F, E>(self, f: F) -> Result<Vec<B>, E>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Result<B, E>;
}

/// Implementation of `TryMapIteratorExt` for all iterators.
impl<I> TryMapIteratorExt for I
where
    I: Iterator,
{
    fn try_map<B, F, E>(mut self, mut f: F) -> Result<Vec<B>, E>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Result<B, E>,
    {
        self.try_fold(Vec::new(), |mut results, item| {
            results.push(f(item)?);
            Ok(results)
        })
    }
}
