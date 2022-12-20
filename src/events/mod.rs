//! # DBus interface proxies for: `org.a11y.atspi.Event.Object`, `org.a11y.atspi.Event.Window`, `org.a11y.atspi.Event.Mouse`, `org.a11y.atspi.Event.Keyboard`, `org.a11y.atspi.Event.Terminal`, `org.a11y.atspi.Event.Document`, `org.a11y.atspi.Event.Focus`
//!
//! This code was generated by `zbus-xmlgen` `2.0.1` from DBus introspection data.
//! Source: `Event.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

pub mod document;
pub mod focus;
pub mod keyboard;
pub mod mouse;
pub mod object;
pub mod terminal;
pub mod window;

use std::{collections::HashMap, error::Error, sync::Arc};

use serde::{Deserialize, Serialize};
use zbus::{
    names::{InterfaceName, MemberName, UniqueName},
    zvariant::{self, ObjectPath, OwnedObjectPath, OwnedValue, Signature, Type, Value},
    Message,
};

use crate::{cache::CacheItem, connection, ATSPI_EVENT};

#[derive(Debug, Serialize, Deserialize)]
pub struct EventBody<'a, T> {
    #[serde(rename = "type")]
    pub kind: T,
    pub detail1: i32,
    pub detail2: i32,
    #[serde(borrow)]
    pub any_data: Value<'a>,
    #[serde(borrow)]
    pub properties: HashMap<&'a str, Value<'a>>,
}

impl<T> Type for EventBody<'_, T> {
    fn signature() -> Signature<'static> {
        <(&str, i32, i32, Value, HashMap<&str, Value>)>::signature()
    }
}

// Signature:  "siiv(so)",
#[derive(Debug, Serialize, Deserialize, Type)]
pub struct EventBodyQT {
    #[serde(rename = "type")]
    pub kind: String,
    pub detail1: i32,
    pub detail2: i32,
    pub any_data: OwnedValue,
    pub properties: (String, OwnedObjectPath),
}

// Signature (siiva{sv}),
#[derive(Clone, Debug, Serialize, Deserialize, Type)]
pub struct EventBodyOwned {
    #[serde(rename = "type")]
    pub kind: String,
    pub detail1: i32,
    pub detail2: i32,
    pub any_data: OwnedValue,
    pub properties: HashMap<String, OwnedValue>,
}

impl From<EventBodyQT> for EventBodyOwned {
    fn from(body: EventBodyQT) -> Self {
        let mut props = HashMap::new();
        props.insert(
            body.properties.0,
            Value::ObjectPath(body.properties.1.into_inner()).to_owned(),
        );
        Self {
            kind: body.kind,
            detail1: body.detail1,
            detail2: body.detail2,
            any_data: body.any_data,
            properties: props,
        }
    }
}

/// Encapsulates the different bus signal types
#[derive(Debug, Clone)]
pub enum Event {
    /// Includes Atspi and Qspi events
    Atspi(AtspiEvent),
    /// Both `CacheAdd` and `CacheRemove` signals
    Cache(CacheEvent),
    //  Device(DeviceEvent),
    //  Listener(EventListener),
}

#[derive(Debug, Clone)]
pub enum CacheEvent {
    Add(CacheAddEvent),
    Remove(CacheRemoveEvent),
}

impl TryFrom<Arc<Message>> for CacheEvent {
    type Error = Box<dyn Error>;

    fn try_from(value: Arc<Message>) -> Result<Self, Self::Error> {
        let rem = CacheRemoveEvent::try_from(value.clone());
        let add = CacheAddEvent::try_from(value);
        match (rem, add) {
            (Ok(rem), _) => Ok(CacheEvent::Remove(rem)),
            (_, Ok(add)) => Ok(CacheEvent::Add(add)),
            _ => Err("conversion to cache variant failed".into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CacheAddEvent {
    message: Arc<Message>,
    body: CacheItem,
}

#[derive(Debug, Clone)]
pub struct CacheRemoveEvent {
    message: Arc<Message>,
    body: Accessible,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct Accessible {
    name: String,
    path: OwnedObjectPath,
}

impl TryFrom<Arc<Message>> for CacheRemoveEvent {
    type Error = Box<dyn Error>;

    fn try_from(value: Arc<Message>) -> Result<Self, Self::Error> {
        // TODO: iface static string should be from the enum elsewhere. (Or, perhapps const interface names..)
        let iface = InterfaceName::from_static_str("org.a11y.atspi.Cache")?;
        if value.interface() != Some(iface) {
            return Err("incorrect interface, not Cache".into());
        }
        if value.member() == Some(MemberName::from_static_str("Remove").unwrap()) {
            let body = value.body::<Accessible>()?;
            Ok(Self { message: value, body })
        } else {
            Err("convert to CacheRemoveEvent failed".into())
        }
    }
}

impl TryFrom<Arc<Message>> for CacheAddEvent {
    type Error = Box<dyn Error>;

    fn try_from(value: Arc<Message>) -> Result<Self, Self::Error> {
        // TODO: iface static string should be from the enum elsewhere. (Or, perhapps const interface names..)
        let iface = InterfaceName::from_static_str("org.a11y.atspi.Cache")?;
        if value.interface() != Some(iface) {
            return Err("incorrect interface, not Cache".into());
        }
        if value.member() == Some(MemberName::from_static_str("Add").unwrap()) {
            let body = value.body::<CacheItem>()?;
            Ok(Self { message: value, body })
        } else {
            Err("conversion to CacheAddEvent failed".into())
        }
    }
}

#[derive(Debug, Clone)]
pub struct AtspiEvent {
    message: Arc<Message>,
    body: EventBodyOwned,
}

impl TryFrom<Arc<Message>> for AtspiEvent {
    type Error = zbus::Error;

    fn try_from(message: Arc<Message>) -> zbus::Result<Self> {
        let qt_sig = Signature::try_from("siiv(so)").unwrap();
        let body: EventBodyOwned = match message.body_signature() {
            Ok(sig) => {
                if sig == qt_sig {
                    EventBodyOwned::from(message.body::<EventBodyQT>()?)
                } else {
                    message.body::<EventBodyOwned>()?
                }
            }
            Err(e) => return Err(e),
        };
        Ok(Self { message, body })
    }
}

impl TryFrom<Arc<Message>> for Event {
    type Error = Box<dyn Error>;

    fn try_from(message: Arc<Message>) -> Result<Self, Self::Error> {
        let atspistr: &str = connection::ATSPI_EVENT.as_str();
        let qspistr: &str = connection::QSPI_EVENT.as_str();
        let cache_add_str: &str = connection::CACHE_ADD.as_str();
        let cache_rem_str: &str = connection::CACHE_REM.as_str();

        match message.body_signature()?.as_str() {
            atspistr => {
                let ev = AtspiEvent::try_from(message)?;
                Ok(Event::Atspi(ev))
            }
            qspistr => {
                let ev = AtspiEvent::try_from(message)?;
                Ok(Event::Atspi(ev))
            }
            cache_add_str => {
                let ev = CacheEvent::try_from(message)?;
                Ok(Event::Cache(ev))
            }
            cache_rem_str => {
                let ev = CacheEvent::try_from(message)?;
                Ok(Event::Cache(ev))
            }
            _ => Err("(currently) unsupported bus type signature".into()),
        }
    }
}

// TODO: exxtract methods that apply to all Event types and create a trait for all Events.
impl AtspiEvent {
    /// Identifies the `sender` of the `Event`.
    /// # Errors
    /// - when deserializeing the header failed, or
    /// - When `zbus::get_field!` finds that 'sender' is an invalid field.
    pub fn sender(&self) -> zbus::Result<Option<UniqueName>> {
        Ok(self.message.header()?.sender()?.cloned())
    }

    /// The object path to the object where the signal is emitted from.
    #[must_use]
    pub fn path(&self) -> Option<zvariant::ObjectPath> {
        self.message.path()
    }

    /// Returns the atspi event string for this event type (E.G. "Object:StateChanged:Focused").
    ///
    /// This should not be used for matching on events as it needlessly allocates and copies the 3
    /// components of the event type. It is meant for logging, etc.
    #[must_use]
    pub fn event_string(&self) -> String {
        let interface = self.interface().expect("Event should have an interface");
        let interface = interface.rsplit('.').next().expect("Interface should contain a '.'");
        let member = self.member().expect("Event should have a member");
        let kind = self.kind();
        format!("{interface}:{member}:{kind}")
    }

    /// For now this returns the full interface name because the lifetimes in [`zbus_names`][zbus::names] are
    /// wrong such that the `&str` you can get from a
    /// [`zbus_names::InterfaceName`][zbus::names::InterfaceName] is tied to the lifetime of that
    /// name, not to the lifetime of the message as it should be. In future, this will return only
    /// the last component of the interface name (I.E. "Object" from
    /// "org.a11y.atspi.Event.Object").
    #[must_use]
    pub fn interface(&self) -> Option<InterfaceName<'_>> {
        self.message.interface()
    }

    /// Identifies this `Event`'s interface member name on the bus.
    /// Members of the interface are either signals, methods or properties.
    /// eg. `PropertyChanged` or `TextChanged`
    #[must_use]
    pub fn member(&self) -> Option<MemberName<'_>> {
        self.message.member()
    }

    #[must_use]
    pub fn kind(&self) -> &str {
        &self.body.kind
    }

    /// Event dependant detail.
    #[must_use]
    pub fn detail1(&self) -> i32 {
        self.body.detail1
    }

    /// Event dependant detail.
    #[must_use]
    pub fn detail2(&self) -> i32 {
        self.body.detail2
    }

    /// Event dependant generic `Value`.
    #[must_use]
    pub fn any_data(&self) -> &zvariant::OwnedValue {
        &self.body.any_data
    }

    #[must_use]
    pub fn properties(&self) -> &HashMap<String, zvariant::OwnedValue> {
        &self.body.properties
    }

    /// Serialized bus message.
    #[must_use]
    pub fn message(&self) -> &Arc<Message> {
        &self.message
    }

    /// Deserialized body type.
    #[must_use]
    pub fn body(&self) -> &EventBodyOwned {
        &self.body
    }
}
