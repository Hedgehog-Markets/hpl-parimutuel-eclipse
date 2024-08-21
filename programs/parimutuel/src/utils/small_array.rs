use std::io::{self, Read, Write};
use std::ops::{Deref, DerefMut};

use borsh::{BorshDeserialize, BorshSerialize};

pub type SmallU64Array = SmallArray<u64>;

#[derive(Clone)]
#[repr(transparent)]
pub struct SmallArray<T> {
    vec: Vec<T>,
}

impl<T> SmallArray<T> {
    #[inline]
    pub fn len(&self) -> u8 {
        self.vec.len() as u8
    }
}

impl<T: Clone> SmallArray<T> {
    #[inline]
    pub fn from_elem(element: T, length: u8) -> Self {
        Self { vec: vec![element; length as usize] }
    }
}

impl<T> Deref for SmallArray<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.vec.deref()
    }
}

impl<T> DerefMut for SmallArray<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.vec.deref_mut()
    }
}

impl<T: BorshDeserialize> BorshDeserialize for SmallArray<T> {
    fn deserialize_reader<R: Read>(reader: &mut R) -> io::Result<Self> {
        let len = u8::deserialize_reader(reader)?;

        let mut vec = Vec::<T>::with_capacity(len as usize);

        for _ in 0..len {
            let element = T::deserialize_reader(reader)?;

            vec.push(element);
        }

        Ok(Self { vec })
    }
}

impl<T: BorshSerialize> BorshSerialize for SmallArray<T> {
    fn serialize<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        u8::serialize(&self.len(), writer)?;

        for element in self.iter() {
            T::serialize(element, writer)?;
        }

        Ok(())
    }
}

impl<T> TryFrom<Vec<T>> for SmallArray<T> {
    type Error = Vec<T>;

    #[inline]
    fn try_from(vec: Vec<T>) -> Result<Self, Self::Error> {
        if vec.len() > u8::MAX as usize {
            return Err(vec);
        }
        Ok(Self { vec })
    }
}

impl<T> From<SmallArray<T>> for Vec<T> {
    #[inline]
    fn from(value: SmallArray<T>) -> Self {
        value.vec
    }
}
