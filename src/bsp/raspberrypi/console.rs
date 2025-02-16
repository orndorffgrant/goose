use crate::{console, synchronization, synchronization::NullLock};
use core::fmt;


struct QEMUOutputInner {
    chars_written: usize,
}

impl QEMUOutputInner {
    pub const fn new() -> QEMUOutputInner {
        QEMUOutputInner { chars_written: 0 }
    }
    
    pub fn write_char(&mut self, c: char) {
        unsafe {
            core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8)
        }
        self.chars_written += 1;
    }
}

impl fmt::Write for QEMUOutputInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            if c == '\n' {
                self.write_char('\r');
            }
            self.write_char(c);
        }
        Ok(())
    }
}

pub struct QEMUOutput {
    inner: NullLock<QEMUOutputInner>,
}

impl QEMUOutput {
    pub const fn new() -> QEMUOutput {
        QEMUOutput {
            inner: NullLock::new(QEMUOutputInner::new()),
        }
    }
}

use synchronization::interface::Mutex;

impl console::interface::Write for QEMUOutput {
    fn write_fmt(&self, args: core::fmt::Arguments) -> fmt::Result {
        self.inner.lock(|inner| fmt::Write::write_fmt(inner, args))
    }
    fn write_char(&self, _c: char) {
        unimplemented!()
    }
    fn flush(&self) {
        unimplemented!()
    }
}

impl console::interface::Read for QEMUOutput {
    fn read_char(&self) -> char {
        unimplemented!()
    }
    fn clear_rx(&self) {
        unimplemented!()
    }
}

impl console::interface::Statistics for QEMUOutput {
    fn chars_written(&self) -> usize {
        self.inner.lock(|inner| inner.chars_written)
    }
}

impl console::interface::All for QEMUOutput {}

static QEMU_OUTPUT: QEMUOutput = QEMUOutput::new();

pub fn console() -> &'static dyn console::interface::All {
    &super::driver::PL011_UART
}
