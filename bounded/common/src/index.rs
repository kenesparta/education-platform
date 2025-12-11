use std::cmp::Ordering;
use std::fmt;

use thiserror::Error;

/// A zero-based index value object for entities.
///
/// `Index` represents a position within a collection or sequence.
/// It provides comparison operations and can be used to track ordering
/// of items in lists, arrays, or any ordered collection.
///
/// As a Value Object:
/// - Immutable once created
/// - Compared by value
/// - Thread-safe and copyable
///
/// # Examples
///
/// ```
/// use education_platform_common::Index;
///
/// let first = Index::new(0);
/// let second = Index::new(1);
/// let third = Index::new(2);
///
/// assert!(first < second);
/// assert!(second < third);
/// assert_eq!(first.value(), 0);
///
/// // Compare two indices
/// assert!(first.is_less_than(&second));
/// assert!(third.is_greater_than(&second));
/// assert!(first.is_equal_to(&Index::new(0)));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Index {
    value: usize,
}

impl Index {
    /// Creates a new `Index` from a value.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Index;
    ///
    /// let index = Index::new(5);
    /// assert_eq!(index.value(), 5);
    ///
    /// let zero = Index::new(0);
    /// assert_eq!(zero.value(), 0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(value: usize) -> Self {
        Self { value }
    }

    /// Creates an `Index` representing the first position (0).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Index;
    ///
    /// let first = Index::first();
    /// assert_eq!(first.value(), 0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn first() -> Self {
        Self { value: 0 }
    }

    /// Returns the underlying value.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Index;
    ///
    /// let index = Index::new(42);
    /// assert_eq!(index.value(), 42);
    /// ```
    #[inline]
    #[must_use]
    pub const fn value(&self) -> usize {
        self.value
    }

    /// Returns the next index (incremented by 1).
    ///
    /// # Errors
    ///
    /// Returns `IndexError::Overflow` if incrementing would overflow.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Index;
    ///
    /// let index = Index::new(5);
    /// let next = index.next().unwrap();
    /// assert_eq!(next.value(), 6);
    /// ```
    pub fn next(&self) -> Result<Self, IndexError> {
        self.value
            .checked_add(1)
            .map(Self::new)
            .ok_or(IndexError::Overflow)
    }

    /// Returns the previous index (decremented by 1).
    ///
    /// # Errors
    ///
    /// Returns `IndexError::Underflow` if the index is already 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Index;
    ///
    /// let index = Index::new(5);
    /// let prev = index.previous().unwrap();
    /// assert_eq!(prev.value(), 4);
    ///
    /// let zero = Index::new(0);
    /// assert!(zero.previous().is_err());
    /// ```
    pub fn previous(&self) -> Result<Self, IndexError> {
        self.value
            .checked_sub(1)
            .map(Self::new)
            .ok_or(IndexError::Underflow)
    }

    /// Returns true if this index is less than another.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Index;
    ///
    /// let a = Index::new(1);
    /// let b = Index::new(5);
    ///
    /// assert!(a.is_less_than(&b));
    /// assert!(!b.is_less_than(&a));
    /// assert!(!a.is_less_than(&a));
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_less_than(&self, other: &Self) -> bool {
        self.value < other.value
    }

    /// Returns true if this index is greater than another.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Index;
    ///
    /// let a = Index::new(5);
    /// let b = Index::new(1);
    ///
    /// assert!(a.is_greater_than(&b));
    /// assert!(!b.is_greater_than(&a));
    /// assert!(!a.is_greater_than(&a));
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_greater_than(&self, other: &Self) -> bool {
        self.value > other.value
    }

    /// Returns true if this index is equal to another.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Index;
    ///
    /// let a = Index::new(5);
    /// let b = Index::new(5);
    /// let c = Index::new(3);
    ///
    /// assert!(a.is_equal_to(&b));
    /// assert!(!a.is_equal_to(&c));
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_equal_to(&self, other: &Self) -> bool {
        self.value == other.value
    }

    /// Returns true if this index is less than or equal to another.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Index;
    ///
    /// let a = Index::new(3);
    /// let b = Index::new(5);
    /// let c = Index::new(3);
    ///
    /// assert!(a.is_less_than_or_equal(&b));
    /// assert!(a.is_less_than_or_equal(&c));
    /// assert!(!b.is_less_than_or_equal(&a));
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_less_than_or_equal(&self, other: &Self) -> bool {
        self.value <= other.value
    }

    /// Returns true if this index is greater than or equal to another.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Index;
    ///
    /// let a = Index::new(5);
    /// let b = Index::new(3);
    /// let c = Index::new(5);
    ///
    /// assert!(a.is_greater_than_or_equal(&b));
    /// assert!(a.is_greater_than_or_equal(&c));
    /// assert!(!b.is_greater_than_or_equal(&a));
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_greater_than_or_equal(&self, other: &Self) -> bool {
        self.value >= other.value
    }

    /// Compares this index with another and returns the ordering.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Index;
    /// use std::cmp::Ordering;
    ///
    /// let a = Index::new(1);
    /// let b = Index::new(5);
    /// let c = Index::new(1);
    ///
    /// assert_eq!(a.compare(&b), Ordering::Less);
    /// assert_eq!(b.compare(&a), Ordering::Greater);
    /// assert_eq!(a.compare(&c), Ordering::Equal);
    /// ```
    #[inline]
    #[must_use]
    pub const fn compare(&self, other: &Self) -> Ordering {
        match (self.value, other.value) {
            (a, b) if a < b => Ordering::Less,
            (a, b) if a > b => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }

    /// Returns true if this is the first index (0).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Index;
    ///
    /// let first = Index::first();
    /// assert!(first.is_first());
    ///
    /// let second = Index::new(1);
    /// assert!(!second.is_first());
    /// ```
    #[inline]
    #[must_use]
    pub const fn is_first(&self) -> bool {
        self.value == 0
    }

    /// Returns the distance between this index and another.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Index;
    ///
    /// let a = Index::new(10);
    /// let b = Index::new(3);
    ///
    /// assert_eq!(a.distance_from(&b), 7);
    /// assert_eq!(b.distance_from(&a), 7);
    /// ```
    #[inline]
    #[must_use]
    pub const fn distance_from(&self, other: &Self) -> usize {
        self.value.abs_diff(other.value)
    }
}

impl Default for Index {
    /// Creates an index at the first position (0).
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Index;
    ///
    /// let index = Index::default();
    /// assert_eq!(index.value(), 0);
    /// ```
    #[inline]
    fn default() -> Self {
        Self::first()
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<usize> for Index {
    fn from(value: usize) -> Self {
        Self::new(value)
    }
}

impl From<Index> for usize {
    fn from(index: Index) -> Self {
        index.value
    }
}

/// Error types for index operations.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum IndexError {
    #[error("Index overflow: cannot increment beyond maximum value")]
    Overflow,

    #[error("Index underflow: cannot decrement below zero")]
    Underflow,
}

#[cfg(test)]
mod tests {
    use super::*;

    mod constructors {
        use super::*;

        #[test]
        fn test_new_creates_index() {
            let index = Index::new(5);
            assert_eq!(index.value(), 5);
        }

        #[test]
        fn test_new_with_zero() {
            let index = Index::new(0);
            assert_eq!(index.value(), 0);
        }

        #[test]
        fn test_first_creates_zero_index() {
            let index = Index::first();
            assert_eq!(index.value(), 0);
        }

        #[test]
        fn test_default_creates_zero_index() {
            let index = Index::default();
            assert_eq!(index.value(), 0);
        }

        #[test]
        fn test_from_usize() {
            let index: Index = 42.into();
            assert_eq!(index.value(), 42);
        }

        #[test]
        fn test_into_usize() {
            let index = Index::new(42);
            let value: usize = index.into();
            assert_eq!(value, 42);
        }
    }

    mod comparison {
        use super::*;

        #[test]
        fn test_is_less_than_returns_true_when_less() {
            let a = Index::new(1);
            let b = Index::new(5);
            assert!(a.is_less_than(&b));
        }

        #[test]
        fn test_is_less_than_returns_false_when_greater() {
            let a = Index::new(5);
            let b = Index::new(1);
            assert!(!a.is_less_than(&b));
        }

        #[test]
        fn test_is_less_than_returns_false_when_equal() {
            let a = Index::new(5);
            let b = Index::new(5);
            assert!(!a.is_less_than(&b));
        }

        #[test]
        fn test_is_greater_than_returns_true_when_greater() {
            let a = Index::new(5);
            let b = Index::new(1);
            assert!(a.is_greater_than(&b));
        }

        #[test]
        fn test_is_greater_than_returns_false_when_less() {
            let a = Index::new(1);
            let b = Index::new(5);
            assert!(!a.is_greater_than(&b));
        }

        #[test]
        fn test_is_greater_than_returns_false_when_equal() {
            let a = Index::new(5);
            let b = Index::new(5);
            assert!(!a.is_greater_than(&b));
        }

        #[test]
        fn test_is_equal_to_returns_true_when_equal() {
            let a = Index::new(5);
            let b = Index::new(5);
            assert!(a.is_equal_to(&b));
        }

        #[test]
        fn test_is_equal_to_returns_false_when_different() {
            let a = Index::new(5);
            let b = Index::new(3);
            assert!(!a.is_equal_to(&b));
        }

        #[test]
        fn test_is_less_than_or_equal_when_less() {
            let a = Index::new(3);
            let b = Index::new(5);
            assert!(a.is_less_than_or_equal(&b));
        }

        #[test]
        fn test_is_less_than_or_equal_when_equal() {
            let a = Index::new(5);
            let b = Index::new(5);
            assert!(a.is_less_than_or_equal(&b));
        }

        #[test]
        fn test_is_less_than_or_equal_when_greater() {
            let a = Index::new(5);
            let b = Index::new(3);
            assert!(!a.is_less_than_or_equal(&b));
        }

        #[test]
        fn test_is_greater_than_or_equal_when_greater() {
            let a = Index::new(5);
            let b = Index::new(3);
            assert!(a.is_greater_than_or_equal(&b));
        }

        #[test]
        fn test_is_greater_than_or_equal_when_equal() {
            let a = Index::new(5);
            let b = Index::new(5);
            assert!(a.is_greater_than_or_equal(&b));
        }

        #[test]
        fn test_is_greater_than_or_equal_when_less() {
            let a = Index::new(3);
            let b = Index::new(5);
            assert!(!a.is_greater_than_or_equal(&b));
        }

        #[test]
        fn test_compare_returns_less() {
            let a = Index::new(1);
            let b = Index::new(5);
            assert_eq!(a.compare(&b), Ordering::Less);
        }

        #[test]
        fn test_compare_returns_greater() {
            let a = Index::new(5);
            let b = Index::new(1);
            assert_eq!(a.compare(&b), Ordering::Greater);
        }

        #[test]
        fn test_compare_returns_equal() {
            let a = Index::new(5);
            let b = Index::new(5);
            assert_eq!(a.compare(&b), Ordering::Equal);
        }
    }

    mod navigation {
        use super::*;

        #[test]
        fn test_next_increments_index() {
            let index = Index::new(5);
            let next = index.next().unwrap();
            assert_eq!(next.value(), 6);
        }

        #[test]
        fn test_next_from_zero() {
            let index = Index::new(0);
            let next = index.next().unwrap();
            assert_eq!(next.value(), 1);
        }

        #[test]
        fn test_next_overflow_returns_error() {
            let index = Index::new(usize::MAX);
            let result = index.next();
            assert!(matches!(result, Err(IndexError::Overflow)));
        }

        #[test]
        fn test_previous_decrements_index() {
            let index = Index::new(5);
            let prev = index.previous().unwrap();
            assert_eq!(prev.value(), 4);
        }

        #[test]
        fn test_previous_from_one() {
            let index = Index::new(1);
            let prev = index.previous().unwrap();
            assert_eq!(prev.value(), 0);
        }

        #[test]
        fn test_previous_underflow_returns_error() {
            let index = Index::new(0);
            let result = index.previous();
            assert!(matches!(result, Err(IndexError::Underflow)));
        }
    }

    mod utility {
        use super::*;

        #[test]
        fn test_is_first_returns_true_for_zero() {
            let index = Index::new(0);
            assert!(index.is_first());
        }

        #[test]
        fn test_is_first_returns_false_for_non_zero() {
            let index = Index::new(1);
            assert!(!index.is_first());
        }

        #[test]
        fn test_distance_from_when_self_is_greater() {
            let a = Index::new(10);
            let b = Index::new(3);
            assert_eq!(a.distance_from(&b), 7);
        }

        #[test]
        fn test_distance_from_when_other_is_greater() {
            let a = Index::new(3);
            let b = Index::new(10);
            assert_eq!(a.distance_from(&b), 7);
        }

        #[test]
        fn test_distance_from_same_index() {
            let a = Index::new(5);
            let b = Index::new(5);
            assert_eq!(a.distance_from(&b), 0);
        }

        #[test]
        fn test_display_format() {
            let index = Index::new(42);
            assert_eq!(format!("{}", index), "42");
        }

        #[test]
        fn test_debug_format() {
            let index = Index::new(42);
            let debug = format!("{:?}", index);
            assert!(debug.contains("Index"));
            assert!(debug.contains("42"));
        }
    }

    mod value_object_semantics {
        use super::*;

        #[test]
        fn test_equality_for_same_value() {
            let a = Index::new(5);
            let b = Index::new(5);
            assert_eq!(a, b);
        }

        #[test]
        fn test_inequality_for_different_values() {
            let a = Index::new(5);
            let b = Index::new(6);
            assert_ne!(a, b);
        }

        #[test]
        fn test_clone_creates_equal_instance() {
            let a = Index::new(5);
            let b = a.clone();
            assert_eq!(a, b);
        }

        #[test]
        fn test_copy_creates_equal_instance() {
            let a = Index::new(5);
            let b = a;
            assert_eq!(a, b);
        }

        #[test]
        fn test_hash_consistency() {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            let a = Index::new(5);
            let b = Index::new(5);
            set.insert(a);
            assert!(set.contains(&b));
        }

        #[test]
        fn test_ordering_with_operators() {
            let a = Index::new(1);
            let b = Index::new(5);
            let c = Index::new(10);

            assert!(a < b);
            assert!(b < c);
            assert!(a < c);
            assert!(c > b);
            assert!(b > a);
        }

        #[test]
        fn test_partial_ord_consistency() {
            let a = Index::new(5);
            let b = Index::new(5);
            assert!(a <= b);
            assert!(a >= b);
        }
    }

    mod edge_cases {
        use super::*;

        #[test]
        fn test_max_usize_value() {
            let index = Index::new(usize::MAX);
            assert_eq!(index.value(), usize::MAX);
        }

        #[test]
        fn test_distance_from_zero() {
            let a = Index::new(100);
            let b = Index::new(0);
            assert_eq!(a.distance_from(&b), 100);
        }

        #[test]
        fn test_chained_next_operations() {
            let index = Index::new(0);
            let result = index.next().unwrap().next().unwrap().next().unwrap();
            assert_eq!(result.value(), 3);
        }

        #[test]
        fn test_chained_previous_operations() {
            let index = Index::new(5);
            let result = index
                .previous()
                .unwrap()
                .previous()
                .unwrap()
                .previous()
                .unwrap();
            assert_eq!(result.value(), 2);
        }
    }
}
