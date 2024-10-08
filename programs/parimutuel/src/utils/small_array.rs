use std::io::{self, Read, Write};
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};

use borsh::{BorshDeserialize, BorshSerialize};
use borsh_size::{BorshSize, BorshSizeProperties};

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
        let len = u8::deserialize_reader(reader)? as usize;

        let mut vec = Vec::<T>::with_capacity(len);

        for uninit in vec.spare_capacity_mut().iter_mut() {
            let element = T::deserialize_reader(reader)?;

            *uninit = MaybeUninit::new(element);
        }

        // SAFETY: `len == vec.capacity()` and elements `0..len` are now initialized.
        unsafe { vec.set_len(len) };

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

impl<T: BorshSize> BorshSize for SmallArray<T> {
    const MIN_SIZE: usize = u8::MIN_SIZE;
    const MAX_SIZE: Option<usize> = None;

    fn borsh_size(&self) -> usize {
        if T::IS_FIXED_SIZE {
            Self::MIN_SIZE + (self.len() as usize) * T::FIXED_SIZE
        } else {
            let mut size = Self::MIN_SIZE;

            for element in self.iter() {
                size += element.borsh_size();
            }

            size
        }
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
