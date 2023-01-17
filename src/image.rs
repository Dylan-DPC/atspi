//! # `DBus` interface proxy for: `org.a11y.atspi.Image`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from `DBus` introspection data.
//! Source: `Image.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

use crate::CoordType;

#[dbus_proxy(interface = "org.a11y.atspi.Image", assume_defaults = true)]
trait Image {
    /// GetImageExtents method
    fn get_image_extents(&self, coord_type: CoordType) -> zbus::Result<(i32, i32, i32, i32)>;

    /// GetImagePosition method
    fn get_image_position(&self, coord_type: CoordType) -> zbus::Result<(i32, i32)>;

    /// GetImageSize method
    fn get_image_size(&self) -> zbus::Result<(i32, i32)>;

    /// ImageDescription property
    #[dbus_proxy(property)]
    fn image_description(&self) -> zbus::Result<String>;

    /// ImageLocale property
    #[dbus_proxy(property)]
    fn image_locale(&self) -> zbus::Result<String>;
}
use crate::{AtspiProxy, Interface};
impl<'a> AtspiProxy for ImageProxy<'a>{
	const INTERFACE: Interface = Interface::Image;
}