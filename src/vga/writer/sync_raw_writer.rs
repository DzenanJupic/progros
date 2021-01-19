use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

use crate::vga::writer::raw_writer::RawWriter;

static BUFFER_IS_USED: AtomicBool = AtomicBool::new(false);

// maybe make a generic lock out of this
pub struct SyncRawWriter(RawWriter);

impl SyncRawWriter {
    pub const fn new() -> Self {
        // SAFETY:
        // [`BUFFER_IS_USED`] to synchronize all instances of SyncRawWriter
        unsafe { Self(RawWriter::new()) }
    }

    pub fn lock(&mut self) -> WriterGuard {
        Self::acquire_lock();
        WriterGuard(&mut self.0)
    }

    fn acquire_lock() {
        while let Err(_) = BUFFER_IS_USED.compare_exchange_weak(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed,
        ) {
            core::hint::spin_loop()
        }
    }
}

pub struct WriterGuard<'a>(&'a mut RawWriter);

impl Drop for WriterGuard<'_> {
    fn drop(&mut self) {
        BUFFER_IS_USED.store(false, Ordering::Release);
    }
}

impl Deref for WriterGuard<'_> {
    type Target = RawWriter;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WriterGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
