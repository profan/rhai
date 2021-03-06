//! Helper module which defines the `Any` trait to to allow dynamic value handling.

use crate::stdlib::{
    any::{type_name, Any as StdAny, TypeId},
    boxed::Box,
    fmt,
};

/// An raw value of any type.
pub type Variant = dyn Any;

/// A boxed dynamic type containing any value.
pub type Dynamic = Box<Variant>;

/// A trait covering any type.
pub trait Any: StdAny {
    /// Get the `TypeId` of this type.
    fn type_id(&self) -> TypeId;

    /// Get the name of this type.
    fn type_name(&self) -> &'static str;

    /// Convert into `Dynamic`.
    fn into_dynamic(&self) -> Dynamic;

    /// This trait may only be implemented by `rhai`.
    #[doc(hidden)]
    fn _closed(&self) -> _Private;
}

impl<T: Clone + StdAny + ?Sized> Any for T {
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn type_name(&self) -> &'static str {
        type_name::<T>()
    }

    fn into_dynamic(&self) -> Dynamic {
        Box::new(self.clone())
    }

    fn _closed(&self) -> _Private {
        _Private
    }
}

impl Variant {
    /// Is this `Variant` a specific type?
    pub(crate) fn is<T: Any>(&self) -> bool {
        let t = TypeId::of::<T>();
        let boxed = <Variant as Any>::type_id(self);

        t == boxed
    }

    /// Get a reference of a specific type to the `Variant`.
    pub(crate) fn downcast_ref<T: Any>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const Variant as *const T)) }
        } else {
            None
        }
    }

    /// Get a mutable reference of a specific type to the `Variant`.
    pub(crate) fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut Variant as *mut T)) }
        } else {
            None
        }
    }
}

impl fmt::Debug for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("?")
    }
}

impl Clone for Dynamic {
    fn clone(&self) -> Self {
        Any::into_dynamic(self.as_ref())
    }
}

/// An extension trait that allows down-casting a `Dynamic` value to a specific type.
pub trait AnyExt: Sized {
    /// Get a copy of a `Dynamic` value as a specific type.
    fn downcast<T: Any + Clone>(self) -> Result<Box<T>, Self>;

    /// This trait may only be implemented by `rhai`.
    #[doc(hidden)]
    fn _closed(&self) -> _Private;
}

impl AnyExt for Dynamic {
    /// Get a copy of the `Dynamic` value as a specific type.
    ///
    /// # Example
    ///
    /// ```
    /// use rhai::{Dynamic, Any, AnyExt};
    ///
    /// let x: Dynamic = 42_u32.into_dynamic();
    ///
    /// assert_eq!(*x.downcast::<u32>().unwrap(), 42);
    /// ```
    fn downcast<T: Any + Clone>(self) -> Result<Box<T>, Self> {
        if self.is::<T>() {
            unsafe {
                let raw: *mut Variant = Box::into_raw(self);
                Ok(Box::from_raw(raw as *mut T))
            }
        } else {
            Err(self)
        }
    }

    fn _closed(&self) -> _Private {
        _Private
    }
}

/// Private type which ensures that `rhai::Any` and `rhai::AnyExt` can only
/// be implemented by this crate.
#[doc(hidden)]
pub struct _Private;
