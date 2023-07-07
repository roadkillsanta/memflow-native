
mod windows;
pub use self::windows::WindowsOs as NativeOs;

use memflow::cglue;
use memflow::prelude::v1::*;

cglue_impl_group!(NativeOs, OsInstance, {});

#[cfg_attr(feature = "plugins", os(name = "native", return_wrapped = true))]
pub fn create_os(args: &OsArgs, lib: LibArc) -> Result<OsInstanceArcBox<'static>> {
    let os = NativeOs::new(args)?;
    Ok(memflow::plugins::os::create_instance(os, lib, args))
}
