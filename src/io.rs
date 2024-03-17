// Copyright © 2023, 2024 Andrea Corbellini and contributors
// SPDX-License-Identifier: BSD-3-Clause

#![cfg(feature = "use_std")]

use crate::CircularBuffer;
use std::io::Read;
use std::io::Result;
use std::io::Write;

impl<const N: usize> Write for CircularBuffer<N, u8> {
    #[inline]
    fn write(&mut self, src: &[u8]) -> Result<usize> {
        self.extend_from_slice(src);
        Ok(src.len())
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl<const N: usize> Read for CircularBuffer<N, u8> {
    fn read(&mut self, dst: &mut [u8]) -> Result<usize> {
        let (mut right, mut left) = self.as_slices();
        let mut count = right.read(dst)?;
        count += left.read(&mut dst[count..])?;
        self.truncate_front(self.len() - count);
        Ok(count)
    }
}
