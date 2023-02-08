//! # `DBus` interface proxy for: `org.a11y.atspi.Registry`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from `DBus` introspection data.
//! Source: `Registry.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use async_trait::async_trait;
use atspi_macros::atspi_proxy;
use zbus::names::OwnedBusName;

#[atspi_proxy(
	interface = "org.a11y.atspi.Registry",
	default_service = "org.a11y.atspi.Registry",
	default_path = "/org/a11y/atspi/registry"
)]
trait Registry {
	/// DeregisterEvent method
	fn deregister_event(&self, event: &str) -> zbus::Result<()>;

	/// GetRegisteredEvents method
	#[dbus_proxy(name = "GetRegisteredEvents")]
	fn registered_events(&self) -> zbus::Result<Vec<(OwnedBusName, String)>>;

	/// RegisterEvent method
	fn register_event(&self, event: &str) -> zbus::Result<()>;

	/// EventListenerDeregistered signal
	#[dbus_proxy(signal)]
	fn event_listener_deregistered(&self, bus: &str, path: &str) -> zbus::Result<()>;

	/// EventListenerRegistered signal
	#[dbus_proxy(signal)]
	fn event_listener_registered(&self, bus: &str, path: &str) -> zbus::Result<()>;
}
