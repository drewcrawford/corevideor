mod cvdisplaylink;
mod cvreturn;
mod cvbase;

pub use cvdisplaylink::CVDisplayLink;
pub use cvdisplaylink::CVDisplayLinkOutputCallback;
pub use cvdisplaylink::Managed as ManagedDisplayLink;

pub use cvreturn::CVReturn;
pub use cvbase::{CVOptionFlags, CVTimeStamp, CVSMPTETime};

#[link(name="CoreVideo",kind="framework")] extern "C" {}