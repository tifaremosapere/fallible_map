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
pub trait FallibleMapExt<T, E> {
    /// Attempt to map a function over an optional value.
    ///
    /// # Parameters
    ///
    /// - `f`: A function that takes a value of type `T` and returns a `Result<U, E>`.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<U>`, or an error `E`.
    fn try_map<F, U>(self, f: F) -> Result<Option<U>, E>
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
    fn try_and_then<F, U>(self, f: F) -> Result<Option<U>, E>
    where
        F: FnOnce(T) -> Result<Option<U>, E>;
}

/// Implementation of `FallibleMapExt` for types implementing `ExtractOption`.
impl<C, T, E> FallibleMapExt<T, E> for C
where
    C: ExtractOption<T>,
{
    fn try_map<F, U>(self, f: F) -> Result<Option<U>, E>
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

    fn try_and_then<F, U>(self, f: F) -> Result<Option<U>, E>
    where
        F: FnOnce(T) -> Result<Option<U>, E>,
    {
        match self.extract() {
            Some(x) => f(x),
            None => Ok(None),
        }
    }
}

/// A fallible map iterator that maps a function returning a `Result` over the elements of the underlying iterator.
pub struct FallibleMapIterator<I, F, B, E> {
    iter: I,
    f: F,
    _marker: std::marker::PhantomData<(B, E)>,
}

impl<I, F, B, E> FallibleMapIterator<I, F, B, E> {
    pub fn new(iter: I, f: F) -> Self {
        FallibleMapIterator {
            iter,
            f,
            _marker: std::marker::PhantomData,
        }
    }
}

/// Implement `Iterator` for `FallibleMap` where the iterator item is a `Result`.
impl<I, F, B, E> Iterator for FallibleMapIterator<I, F, B, E>
where
    I: Iterator,
    F: FnMut(I::Item) -> Result<B, E>,
{
    type Item = Result<B, E>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(&mut self.f)
    }
}

/// Extend iterator with fallible map functionality.
pub trait FallibleMapIteratorExt: Iterator {
    /// Attempt to map a function over an iterator, returning a `Result` iterator.
    ///
    /// # Parameters
    ///
    /// - `f`: A function that takes an item and returns a `Result<B, E>`.
    ///
    /// # Returns
    ///
    /// An iterator where each item is a `Result<B, E>`.
    fn try_map<B, F, E>(self, f: F) -> FallibleMapIterator<Self, F, B, E>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Result<B, E>;
}

/// Implementation of `FallibleMapIteratorExt` for all iterators.
impl<I> FallibleMapIteratorExt for I
where
    I: Iterator,
{
    fn try_map<B, F, E>(self, f: F) -> FallibleMapIterator<Self, F, B, E>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Result<B, E>,
    {
        FallibleMapIterator::new(self, f)
    }
}
