#[repr(transparent)]
#[derive(PartialEq,Eq)]
#[derive(Debug)]
pub struct CVReturn(i32);

#[allow(non_upper_case_globals)]
impl CVReturn {
    pub const Success: CVReturn = CVReturn(0);
    pub const First: CVReturn = CVReturn(-6660);
    pub const Error: CVReturn = CVReturn::Last;
    pub const InvalidArgument: CVReturn = CVReturn(-6661);
    pub const AllocationFailed: CVReturn = CVReturn(-6662);
    pub const Unsupported: CVReturn = CVReturn(-6663);
    pub const InvalidDisplay: CVReturn = CVReturn(-6670);
    pub const DisplayLinkAlreadyRunning: CVReturn = CVReturn(-6671);
    pub const DisplayLinkNotRunning: CVReturn = CVReturn(-6672);
    pub const DisplayLinkCallbacksNotSet: CVReturn = CVReturn(-6673);
    pub const InvalidPixelFormat: CVReturn = CVReturn(-6680);
    pub const InvalidSize: CVReturn = CVReturn(-6681);
    pub const InvalidPixelBufferAttributes: CVReturn = CVReturn(-6682);
    pub const PixelBufferNotOpenGLCompatible: CVReturn = CVReturn(-6683);
    pub const PixelBufferNotMetalCompatible: CVReturn = CVReturn(-6684);
    pub const WouldExceedAllocationThreshold: CVReturn = CVReturn(-6689);
    pub const PoolAllocationFailed: CVReturn = CVReturn(-6690);
    pub const InvalidPoolAttributes: CVReturn = CVReturn(-6691);
    pub const Retry: CVReturn = CVReturn(-6692);
    pub const Last: CVReturn = CVReturn(-6699);
}
