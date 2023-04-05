use serde::{Deserialize, Serialize};

/// A GraphQL `ID`
///
/// Any field in a GraphQL schema that has the type `ID` should be represented
/// by this struct.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ref_cast::RefCast)]
#[repr(transparent)]
pub struct Id(String);

impl Id {
    /// Constructs an `ID` from a `String`, `&str` or similar
    ///
    /// ```
    /// cynic::Id::assume_exists("123");
    /// ```
    pub fn assume_exists(s: impl Into<String>) -> Self {
        Id(s.into())
    }

    /// Returns a reference to the value of this `Id`
    pub fn inner(&self) -> &str {
        &self.0
    }

    /// Converts this `Id` into its inner value
    pub fn into_inner(self) -> String {
        self.0
    }

    /// Converts a reference to a String to a reference to an Id
    ///
    /// To be used when you can access an `&String` which you want to assume is
    /// an `Id` for use in Cynic structures without reallocating
    ///
    /// If you don't have a `String` at hand but only an `&str`, you
    /// may construct a `&IdSlice`
    pub fn from_ref(s: &String) -> &Self {
        ref_cast::RefCast::ref_cast(s)
    }
}

impl std::ops::Deref for Id {
    type Target = IdSlice;

    fn deref(&self) -> &Self::Target {
        IdSlice::assume_exists(self.0.as_str())
    }
}

impl<T: Into<String>> From<T> for Id {
    fn from(s: T) -> Id {
        Id(s.into())
    }
}

impl<'a> From<&'a String> for &'a Id {
    fn from(s: &'a String) -> Self {
        Id::from_ref(s)
    }
}

/// [`IdSlice`] is to [`Id`] what [`str`] is to [`String`]
///
/// An `&IdSlice` can be constructed from an `&str`, and `Id` `deref`s to that,
/// which means you can construct your input structs simply by borrowing an
/// `Id`:
/// ```
/// # use cynic::{Id, IdSlice};
///
/// // #[derive(InputObject)]
/// struct Input<'a> {
///     id: &'a IdSlice,
/// }
///
/// let input = Input {
///     id: "i_know_that_this_is_an_id".into(),
/// };
///
/// let some_id_elsewhere = &Id::assume_exists("owned id");
///
/// let input_from_id = Input {
///     id: &some_id_elsewhere, // This works thanks to `Deref`
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Serialize, ref_cast::RefCast)]
#[repr(transparent)]
pub struct IdSlice(str);

impl IdSlice {
    /// Constructs an `&IdSlice` from an `&str`
    ///
    /// ```
    /// let id: &'static IdSlice = cynic::IdSlice::assume_exists("123");
    /// ```
    pub fn assume_exists(s: &str) -> &Self {
        ref_cast::RefCast::ref_cast(s)
    }

    /// Returns a reference to the value of this `Id`
    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl<'a> From<&'a str> for &'a IdSlice {
    fn from(s: &'a str) -> Self {
        IdSlice::assume_exists(s)
    }
}
