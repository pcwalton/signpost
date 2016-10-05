// signpost/src/lib.rs

#[cfg(target_os = "macos")]
const DBG_APPS: u32 = 33;
#[cfg(target_os = "macos")]
const DBG_FUNC_START: u32 = 1;
#[cfg(target_os = "macos")]
const DBG_FUNC_END: u32 = 2;
#[cfg(target_os = "macos")]
const DBG_FUNC_NONE: u32 = 0;
#[cfg(target_os = "macos")]
const DBG_MACH_CHUD: u32 = 0x0a;
#[cfg(target_os = "macos")]
const KDBG_CLASS_OFFSET: u32 = 24;
#[cfg(target_os = "macos")]
const KDBG_SUBCLASS_OFFSET: u32 = 16;
#[cfg(target_os = "macos")]
const KDBG_CODE_OFFSET: u32 = 2;

/// When passed as the last argument to a trace, and when "color using last
/// argument" is checked in Instruments' "Points of Interest" options sidebar,
/// controls the color a trace is rendered with.
///
/// ```
/// signpost::start(42, &[0, 0, 0, signpost::Color::Blue as usize]);
/// // Do stuff...
/// signpost::end(42, &[0, 0, 0, signpost::Color::Blue as usize]);
/// ```
#[cfg(target_os="macos")]
pub enum Color {
    Blue = 0,
    Green = 1,
    Purple = 2,
    Orange = 3,
    Red = 4,
}

#[cfg(target_os = "macos")]
fn appsdbg_code(subclass: u32, code: u32) -> u32 {
    kdbg_code(DBG_APPS, subclass, code)
}

#[cfg(target_os = "macos")]
fn kdbg_code(class: u32, subclass: u32, code: u32) -> u32 {
    kdbg_eventid(class, subclass, code)
}

#[cfg(target_os = "macos")]
fn kdbg_eventid(class: u32, subclass: u32, code: u32) -> u32 {
    ((class & 0xff) << KDBG_CLASS_OFFSET) | ((subclass & 0xff) << KDBG_SUBCLASS_OFFSET) |
        ((code & 0x3fff) << KDBG_CODE_OFFSET)
}

#[cfg(target_os = "macos")]
pub fn start(code: u32, args: &[usize; 4]) {
    unsafe {
        kdebug_trace(appsdbg_code(DBG_MACH_CHUD, code) | DBG_FUNC_START,
                     args[0],
                     args[1],
                     args[2],
                     args[3],
                     0)
    }
}

#[cfg(target_os = "macos")]
pub fn end(code: u32, args: &[usize; 4]) {
    unsafe {
        kdebug_trace(appsdbg_code(DBG_MACH_CHUD, code) | DBG_FUNC_END,
                     args[0],
                     args[1],
                     args[2],
                     args[3],
                     0)
    }
}

#[cfg(target_os = "macos")]
pub fn trace(code: u32, args: &[usize; 4]) {
    unsafe {
        kdebug_trace(appsdbg_code(DBG_MACH_CHUD, code) | DBG_FUNC_NONE,
                     args[0],
                     args[1],
                     args[2],
                     args[3],
                     0)
    }
}

#[cfg(not(target_os = "macos"))]
pub fn start(code: u32, args: &[usize; 4]) {}
#[cfg(not(target_os = "macos"))]
pub fn end(code: u32, args: &[usize; 4]) {}
#[cfg(not(target_os = "macos"))]
pub fn trace(code: u32, args: &[usize; 4]) {}

pub fn trace_function<R, F>(code: u32, args: &[usize; 4], func: F) -> R where F: FnOnce() -> R {
    start(code, args);
    let result = func();
    end(code, args);
    result
}

/// An RAII class to automatically add start and end traces on creation and drop
/// respectively.
pub struct AutoTrace<'a> {
    code: u32,
    args: &'a [usize; 4],
}

impl<'a> AutoTrace<'a> {
    pub fn new(code: u32, args: &'a [usize; 4]) -> AutoTrace<'a> {
        start(code, args);
        AutoTrace {
            code: code,
            args: args,
        }
    }
}

impl<'a> Drop for AutoTrace<'a> {
    fn drop(&mut self) {
        end(self.code, self.args);
    }
}

#[cfg(target_os = "macos")]
extern {
    fn kdebug_trace(code: u32, arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize);
}
