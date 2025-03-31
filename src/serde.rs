use serde::{Serialize, Deserialize, Serializer, Deserializer};

pub(crate) trait DeserializeOrZeroed<'de>: Sized
{
    fn deserialize_or_zeroed<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}
impl<'de, T> DeserializeOrZeroed<'de> for T
{
    default fn deserialize_or_zeroed<D>(_: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        Ok(unsafe {core::mem::zeroed()})
    }
}
impl<'de, T> DeserializeOrZeroed<'de> for T
where
    T: Deserialize<'de>
{
    fn deserialize_or_zeroed<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        Ok(Self::deserialize(deserializer).unwrap_or_else(|_| unsafe {core::mem::zeroed()}))
    }
}

pub(crate) trait MaybeSerialize
{
    fn maybe_serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}
impl<T> MaybeSerialize for T
where
    T: ?Sized
{
    default fn maybe_serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_none()
    }
}
impl<T> MaybeSerialize for T
where
    T: Serialize + ?Sized
{
    fn maybe_serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        self.serialize(serializer)
    }
}