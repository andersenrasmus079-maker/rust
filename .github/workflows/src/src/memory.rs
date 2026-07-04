use windows::Win32::System::Memory::{ReadProcessMemory, PROCESS_VM_READ};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};
use windows::Win32::Foundation::{CloseHandle, HANDLE};

pub struct MemoryReader {
    pub handle: HANDLE,
}

impl MemoryReader {
    pub fn new(pid: u32) -> Option<Self> {
        unsafe {
            let handle = OpenProcess(
                PROCESS_QUERY_LIMITED_INFORMATION | PROCESS_VM_READ,
                false,
                pid,
            ).ok()?;
            Some(Self { handle })
        }
    }

    pub fn read<T: Sized>(&self, address: usize) -> Option<T> {
        let mut buffer: T = unsafe { std::mem::zeroed() };
        let mut read = 0;
        
        unsafe {
            ReadProcessMemory(
                self.handle,
                address as *const _,
                &mut buffer as *mut T as *mut _,
                std::mem::size_of::<T>(),
                Some(&mut read),
            ).ok()?;
        }
        
        if read == std::mem::size_of::<T>() {
            Some(buffer)
        } else {
            None
        }
    }
}

impl Drop for MemoryReader {
    fn drop(&mut self) {
        unsafe { let _ = CloseHandle(self.handle); }
    }
}
