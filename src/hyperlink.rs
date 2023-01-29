//! # `DBus` interface proxy for: `org.a11y.atspi.Hyperlink`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from `DBus` introspection data.
//! Source: `Hyperlink.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use atspi_macros::atspi_proxy;
use async_trait::async_trait;

#[atspi_proxy(interface = "org.a11y.atspi.Hyperlink", assume_defaults = true)]
trait Hyperlink {
    /// GetObject method
    fn get_object(&self, i: i32) -> zbus::Result<(String, zbus::zvariant::OwnedObjectPath)>;

    /// GetURI method
    fn get_uri(&self, i: i32) -> zbus::Result<String>;

    /// IsValid method
    fn is_valid(&self) -> zbus::Result<bool>;

    /// EndIndex property
    #[dbus_proxy(property)]
    fn end_index(&self) -> zbus::Result<i32>;

    /// NAnchors property
    #[dbus_proxy(property)]
    fn nanchors(&self) -> zbus::Result<i16>;

    /// StartIndex property
    #[dbus_proxy(property)]
    fn start_index(&self) -> zbus::Result<i32>;
}
