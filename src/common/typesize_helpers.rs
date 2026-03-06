use std::sync::Arc;

/// Wrapper around `SizableArc<T, Owned>`` with support for disabling typesize.
///
/// This denotes an Arc where T's size should be considered when calling `TypeSize::get_size`
#[derive(Debug)]
pub(crate) struct MaybeOwnedArc<T>(
    #[cfg(feature = "typesize")] typesize::ptr::SizableArc<T, typesize::ptr::Owned>,
    #[cfg(not(feature = "typesize"))] Arc<T>,
);

impl<T> MaybeOwnedArc<T> {
    pub(crate) fn new(inner: T) -> Self {
        Self(Arc::new(inner).into())
    }

    pub(crate) fn get_inner(self) -> Arc<T> {
        #[cfg(feature = "typesize")]
        let inner = self.0 .0;
        #[cfg(not(feature = "typesize"))]
        let inner = self.0;

        inner
    }
}

#[cfg(feature = "typesize")]
impl<T: typesize::TypeSize> typesize::TypeSize for MaybeOwnedArc<T> {
    fn extra_size(&self) -> usize {
        self.0.extra_size()
    }

    typesize::if_typesize_details! {
        fn get_collection_item_count(&self) -> Option<usize> {
            self.0.get_collection_item_count()
        }
    }
}

impl<T> std::ops::Deref for MaybeOwnedArc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Clone for MaybeOwnedArc<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone().into())
    }
}
