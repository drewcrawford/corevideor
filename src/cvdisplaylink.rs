use std::ffi::c_void;
use crate::cvreturn::CVReturn;
use crate::cvbase::{CVTimeStamp, CVOptionFlags};
use std::ops::{DerefMut, Deref};
use coregraphicsr::CGDirectDisplayID;

///Use pointers to this like `CVDisplayLinkRef`
#[repr(C)]
pub struct CVDisplayLink(c_void);

#[derive(Debug)]
///Managed type, will release CVDisplayLink on drop
pub struct Managed(*mut CVDisplayLink);
impl Drop for Managed {
    fn drop(&mut self) {
        unsafe{ CVDisplayLinkRelease(self.0) }
    }
}
/*
typedef CVReturn (*CVDisplayLinkOutputCallback)(
		CVDisplayLinkRef CV_NONNULL displayLink,
		const CVTimeStamp * CV_NONNULL inNow,
		const CVTimeStamp * CV_NONNULL inOutputTime,
		CVOptionFlags flagsIn,
		CVOptionFlags * CV_NONNULL flagsOut,
		void * CV_NULLABLE displayLinkContext );
 */
pub type CVDisplayLinkOutputCallback = extern "C" fn(display_link: *mut CVDisplayLink, in_now: *const CVTimeStamp, in_output_time: *const CVTimeStamp,flags_in: CVOptionFlags, flags_out: *mut CVOptionFlags, display_link_context: *mut c_void) -> CVReturn;


extern "C" {
    fn CVDisplayLinkCreateWithActiveCGDisplays(out: *mut *mut CVDisplayLink) -> CVReturn;
    fn CVDisplayLinkRelease(link: *mut CVDisplayLink );
    fn CVDisplayLinkSetOutputCallback(link: *mut CVDisplayLink, callback: CVDisplayLinkOutputCallback, user_info: *mut c_void) -> CVReturn;
    fn CVDisplayLinkSetCurrentCGDisplay(display_Link: *mut CVDisplayLink, display_id: CGDirectDisplayID) -> CVReturn;
    fn CVDisplayLinkStart(display_link: *mut CVDisplayLink) -> CVReturn;
    fn CVDisplayLinkStop(display_link: *mut CVDisplayLink) -> CVReturn;

}

impl CVDisplayLink {
    /// see also: [CVDisplayLink::set_output_callback_unchecked]
    pub fn set_output_callback<U: Sync>(&mut self, callback: CVDisplayLinkOutputCallback, user_info: *mut U) -> Result<(),CVReturn> {
        unsafe{ self.set_output_callback_unchecked(callback, user_info) }
    }
    ///A slightly relaxed version of set_output_callback that only requires Send.
    ///
    /// # Safety
    /// You must ensure that the type can be shared between threads safely, even though it isn't marked `Sync`.
    pub unsafe fn set_output_callback_unchecked<U>(&mut self, callback: CVDisplayLinkOutputCallback, user_info: *mut U) -> Result<(),CVReturn> {
        let r =
            CVDisplayLinkSetOutputCallback(self, callback, user_info as *mut c_void);
        match r {
            CVReturn::Success => Ok(()),
            other => Err(other)
        }
    }
    pub fn set_current_cg_display(&mut self, display_id: CGDirectDisplayID) -> Result<(),CVReturn> {
        let r = unsafe {
            CVDisplayLinkSetCurrentCGDisplay(self, display_id)
        };
        match r {
            CVReturn::Success => Ok(()),
            other => Err(other)
        }
    }
    pub fn start(&mut self) -> Result<(),CVReturn> {
        let r= unsafe {
            CVDisplayLinkStart(self)
        };
        match r {
            CVReturn::Success => Ok(()),
            other => Err(other)
        }
    }
    pub fn stop(&mut self) -> Result<(),CVReturn> {
        let r= unsafe {
            CVDisplayLinkStop(self)
        };
        match r {
            CVReturn::Success => Ok(()),
            other => Err(other)
        }
    }
}

impl Managed {
    pub fn with_active_displays() -> Result<Managed,CVReturn> {
        let mut ptr = std::ptr::null_mut();
        let result = unsafe{ CVDisplayLinkCreateWithActiveCGDisplays(&mut ptr) };
        match result {
            CVReturn::Success => Ok(Managed(ptr)),
            other => Err(other)
        }
    }
}
impl Deref for Managed {
    type Target = CVDisplayLink;
    fn deref(&self) -> &CVDisplayLink {
        unsafe {&*self.0 }
    }
}
impl DerefMut for Managed {
    fn deref_mut(&mut self) -> &mut CVDisplayLink {
        unsafe {&mut *self.0 }
    }
}
#[cfg(test)] mod tests {
    use crate::cvdisplaylink::{Managed, CVDisplayLink};
    use crate::cvbase::{CVTimeStamp, CVOptionFlags};
    use std::ffi::c_void;
    use crate::cvreturn::CVReturn;

    #[test] fn new() {
        let m = Managed::with_active_displays().unwrap();
        println!("{:?}",m);

    }
    #[test] fn smoke_test() {
        let (mut sender,receiver) = std::sync::mpsc::channel::<()>();
        let mut m = Managed::with_active_displays().unwrap();
        extern "C" fn callback(_display_link: *mut CVDisplayLink, _in_now: *const CVTimeStamp, _in_output_time: *const CVTimeStamp,_flags_in: CVOptionFlags, _flags_out: *mut CVOptionFlags, display_link_context: *mut c_void) -> CVReturn {
            println!("Hello from callback");
            let context : &std::sync::mpsc::Sender<()> = unsafe{ std::mem::transmute(display_link_context) };
            context.send(()).unwrap();
            CVReturn::Success
        }
        unsafe {
            m.set_output_callback_unchecked(callback, &mut sender).unwrap();
        }

        //todo: get the actually correct display rather than this hardcoded value?
        m.set_current_cg_display(0).unwrap();
        m.start().unwrap();
        receiver.recv().unwrap();
        m.stop().unwrap();
        //basically, wait for an actual stop
        //technically, this is slightly UB but I think it's fine for testing
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

