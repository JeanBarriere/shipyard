use crate::unknown_storage::UnknownStorage;

/// Type used to [`FakeBorrow`] unique storages.
///
/// [`FakeBorrow`]: struct.FakeBorrow.html
pub struct Unique<T> {
    pub(crate) value: T,
    pub(crate) is_modified: bool,
}

impl<T: 'static> UnknownStorage for Unique<T> {}

impl<T> Unique<T> {
    pub(crate) fn new(value: T) -> Self {
        Unique {
            value,
            is_modified: false,
        }
    }
}
