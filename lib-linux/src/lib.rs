#![no_std]
#![cfg_attr(feature = "abort", feature(core_intrinsics))]

use core::{
    ffi::{self, c_char},
    hint::unreachable_unchecked,
    marker::PhantomData,
    mem::MaybeUninit,
    num::{NonZero, NonZeroUsize},
    ptr::{null_mut, NonNull},
};
use syscalls::{syscall, Errno, Sysno};
type RawFd = i32;

pub static STDIN: OwnedFd = OwnedFd(0);
pub static STDOUT: OwnedFd = OwnedFd(1);
pub static STDERR: OwnedFd = OwnedFd(2);

#[repr(transparent)]
pub struct Args(*mut usize);

#[repr(transparent)]
pub struct InitArgsMut(*mut usize);

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct EnvAndAux(*mut *mut u8);

// Define a custom CStr because `core::ffi::CStr` requires strlen on creation
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CStr<'a>(pub *mut u8, PhantomData<&'a ()>);

impl CStr<'_> {
    pub fn bytes(&self) -> impl Iterator<Item = NonZero<u8>> {
        let mut a = self.0;
        core::iter::from_fn(move || unsafe {
            let v = *a;
            a = a.add(1);
            core::num::NonZeroU8::new(v)
        })
        .fuse()
    }
}

impl<'a> From<CStr<'a>> for &'a ffi::CStr {
    fn from(value: CStr<'a>) -> Self {
        unsafe { ffi::CStr::from_ptr(value.0 as *mut c_char) }
    }
}

impl Args {
    pub fn get(&self) -> *mut usize {
        self.0
    }

    pub fn argc(&self) -> usize {
        unsafe { *self.0 }
    }

    pub fn argv(&self) -> &[CStr] {
        unsafe { core::slice::from_raw_parts_mut(self.0.add(1) as *mut CStr, self.argc()) }
    }

    pub fn envp(&self) -> EnvAndAux {
        EnvAndAux(unsafe { self.0.add(self.argc() + 2) as *mut *mut u8 })
    }

    pub fn into_args_mut(self) -> InitArgsMut {
        unsafe {
            *self.0.add(self.argc() + 1) = self.envp().end_of_args() as usize;
        }
        InitArgsMut(self.0)
    }
}

impl InitArgsMut {
    pub fn argc(&self) -> usize {
        unsafe { *self.0 }
    }

    pub fn argv(&self) -> &[*mut u8] {
        unsafe { core::slice::from_raw_parts_mut(self.0.add(1) as *mut *mut u8, self.argc()) }
    }

    pub fn argx(&self) -> &[*mut u8] {
        unsafe { core::slice::from_raw_parts_mut(self.0.add(1) as *mut *mut u8, self.argc() + 1) }
    }

    pub fn args(&mut self) -> impl Iterator<Item = &mut [u8]> {
        let mut prev = self.argx()[0];
        self.argx()[1..].iter().copied().map(move |next| {
            let r = unsafe { core::slice::from_raw_parts_mut(prev, next as usize - prev as usize) };
            prev = next;
            r
        })
    }

    pub fn envp(&self) -> EnvAndAux {
        EnvAndAux(unsafe { self.0.add(self.argc() + 2) as *mut *mut u8 })
    }

    pub fn arg_buf(&mut self, start_idx: usize) -> &mut [u8] {
        let Some(start_of_args) = self.argv().get(start_idx).copied() else {
            return &mut [];
        };
        let end_of_args = self.argx().last().copied().unwrap();
        unsafe {
            core::slice::from_raw_parts_mut(
                start_of_args,
                end_of_args as usize - start_of_args as usize,
            )
        }
    }
}

impl EnvAndAux {
    pub fn end_of_args(&self) -> *mut u8 {
        self.envp()
            .next()
            .map(|x| x.as_ptr())
            .unwrap_or_else(|| self.execfn().0)
    }

    pub fn envp(self) -> impl Iterator<Item = NonNull<u8>> {
        let mut envp = self.0;
        core::iter::from_fn(move || unsafe {
            let r = *envp;
            envp = envp.add(1);
            core::ptr::NonNull::new(r)
        })
        .fuse()
    }

    pub fn auxv(self) -> impl Iterator<Item = (NonZeroUsize, usize)> {
        let mut auxv = unsafe { self.0.add(self.envp().count() + 1) as *mut *mut () };
        core::iter::from_fn(move || unsafe {
            let r0 = *auxv;
            let r1 = *auxv.add(1);
            auxv = auxv.add(2);
            NonZeroUsize::new(r0 as usize).map(|x| (x, r1 as usize))
        })
        .fuse()
    }

    pub fn execfn(self) -> CStr<'static> {
        for (k, v) in self.auxv() {
            if k.get() == 0x1f {
                return CStr(v as *mut u8, PhantomData);
            }
        }
        panic!()
    }

    pub fn entry(self) -> Option<usize> {
        for (k, v) in self.auxv() {
            if k.get() == 0x09 {
                return Some(v);
            }
        }
        None
    }
}

pub struct BorrowedBuf<'a>(&'a mut [MaybeUninit<u8>], usize);

impl<'a> BorrowedBuf<'a> {
    #[inline(always)]
    pub fn new(buf: &'a mut [MaybeUninit<u8>]) -> Self {
        Self(buf, 0)
    }

    #[inline(always)]
    pub fn filled(&self) -> &'a [u8] {
        unsafe { core::slice::from_raw_parts(self.0.as_ptr() as *mut u8, self.1) }
    }

    #[inline(always)]
    fn free(&mut self) -> &'a mut [MaybeUninit<u8>] {
        unsafe {
            core::slice::from_raw_parts_mut(self.0.as_mut_ptr().add(self.1), self.0.len() - self.1)
        }
    }

    #[inline(always)]
    unsafe fn commit(&mut self, len: usize) {
        self.1 += len;
    }
}

const AT_FDCWD: RawFd = -100;

#[repr(transparent)]
pub struct OwnedFd(RawFd);

impl OwnedFd {
    pub fn close(self) -> Result<(), Errno> {
        unsafe {
            syscall!(Sysno::close, self.0)?;
        }
        Ok(())
    }

    #[inline(always)]
    pub fn write(&self, a: &[u8]) -> Result<usize, Errno> {
        Ok(unsafe { syscall!(Sysno::write, self.0, a.as_ptr(), a.len())? })
    }

    pub fn write_all(&self, mut a: &[u8]) -> Result<(), Errno> {
        while !a.is_empty() {
            let ret = self.write(a)?;
            a = &a[ret..];
        }
        Ok(())
    }

    #[inline(always)]
    pub fn read_buf(&self, buf: &mut BorrowedBuf) -> Result<usize, Errno> {
        let space = buf.free();
        let size = unsafe { syscall!(Sysno::read, self.0, space.as_ptr(), space.len())? };
        unsafe {
            buf.commit(size);
        }
        Ok(size)
    }

    // Using Errno instead of () increases the binary size of the cat utility considerably.
    #[allow(clippy::result_unit_err)]
    #[inline(always)]
    pub fn read_uninit(&self, buf: &mut [MaybeUninit<u8>]) -> Result<&mut [u8], ()> {
        Ok(unsafe {
            let size =
                syscall!(Sysno::read, self.0, buf.as_mut_ptr(), buf.len()).map_err(|_| ())?;
            core::slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut u8, size)
        })
    }

    #[inline(always)]
    pub fn read(&self, buf: &mut [u8]) -> Result<usize, Errno> {
        Ok(unsafe {
            let size = syscall!(Sysno::read, self.0, buf.as_mut_ptr(), buf.len())?;
            if size > buf.len() {
                unreachable_unchecked();
            }
            size
        })
    }

    pub fn openat(
        dirfd: Option<&OwnedFd>,
        path: &CStr,
        flags: i32,
        mode: i32,
    ) -> Result<Self, Errno> {
        Ok(Self(unsafe {
            syscall!(
                Sysno::openat,
                dirfd.map(|fd| fd.0).unwrap_or(AT_FDCWD),
                path.0,
                flags,
                mode
            )? as RawFd
        }))
    }
}

impl Drop for OwnedFd {
    fn drop(&mut self) {
        unsafe {
            syscall!(Sysno::close, self.0).ok(); // ignore error
        }
    }
}

pub fn exit(status: u8) -> ! {
    raw_exit(status).ok();

    #[cfg(feature = "abort")]
    core::intrinsics::abort();

    #[cfg(not(feature = "abort"))]
    #[allow(clippy::empty_loop)]
    loop {}
}

// The exit syscall can fail, e.g., when denied by seccomp-bpf.
pub fn raw_exit(status: u8) -> Result<(), Errno> {
    unsafe { syscall!(Sysno::exit, status).map(|_| ()) }
}

#[repr(C)]
struct timespec {
    tv_sec: u64,               /* seconds */
    tv_nsec: core::ffi::c_int, /* nanoseconds */
}

pub fn sleep(seconds: u64, nanos: u32) -> Result<(), Errno> {
    let mut a = timespec {
        tv_sec: seconds,
        tv_nsec: nanos as i32,
    };
    unsafe {
        syscall!(
            Sysno::nanosleep,
            (&mut a) as *mut timespec,
            null_mut() as *mut timespec
        )
        .map(|_| ())
    }
}
