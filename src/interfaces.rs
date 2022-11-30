use crate::{
    accessible::AccessibleProxy, action::ActionProxy, application::ApplicationProxy,
    cache::CacheProxy, collection::CollectionProxy, component::ComponentProxy,
    device_event_controller::DeviceEventControllerProxy,
    device_event_listener::DeviceEventListenerProxy, document::DocumentProxy,
    editable_text::EditableTextProxy, hyperlink::HyperlinkProxy, hypertext::HypertextProxy,
    image::ImageProxy, registry::RegistryProxy, selection::SelectionProxy, socket::SocketProxy,
    table::TableProxy, table_cell::TableCellProxy, text::TextProxy, value::ValueProxy,
};
use enumflags2::{bitflags, BitFlag, BitFlags};
use serde::{
    de::{self, Deserialize, Deserializer, Visitor},
    ser::{Serialize, Serializer},
};
use std::fmt;
use zbus::{
    zvariant::{Signature, Type},
    ProxyDefault,
};

/// Indicates AT-SPI interfaces an |`crate::accessible::AccessibleProxy`] can implement.
#[bitflags]
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Interface {
    Accessible,
    Action,
    Application,
    Cache,
    Collection,
    Component,
    Document,
    DeviceEventController,
    DeviceEventListener,
    EditableText,
    Hyperlink,
    Hypertext,
    Image,
    Registry,
    Selection,
    Socket,
    Table,
    TableCell,
    Text,
    Value,
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Interface {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InterfaceVisitor;

        impl<'de> Visitor<'de> for InterfaceVisitor {
            type Value = Interface;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an AT-SPI interface name")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    AccessibleProxy::INTERFACE => Ok(Interface::Accessible),
                    ActionProxy::INTERFACE => Ok(Interface::Action),
                    ApplicationProxy::INTERFACE => Ok(Interface::Application),
                    CacheProxy::INTERFACE => Ok(Interface::Cache),
                    CollectionProxy::INTERFACE => Ok(Interface::Collection),
                    ComponentProxy::INTERFACE => Ok(Interface::Component),
                    DeviceEventControllerProxy::INTERFACE => Ok(Interface::DeviceEventController),
                    DeviceEventListenerProxy::INTERFACE => Ok(Interface::DeviceEventListener),
                    DocumentProxy::INTERFACE => Ok(Interface::Document),
                    EditableTextProxy::INTERFACE => Ok(Interface::EditableText),
                    HyperlinkProxy::INTERFACE => Ok(Interface::Hyperlink),
                    HypertextProxy::INTERFACE => Ok(Interface::Hypertext),
                    ImageProxy::INTERFACE => Ok(Interface::Image),
                    RegistryProxy::INTERFACE => Ok(Interface::Registry),
                    SelectionProxy::INTERFACE => Ok(Interface::Selection),
                    SocketProxy::INTERFACE => Ok(Interface::Socket),
                    TableProxy::INTERFACE => Ok(Interface::Table),
                    TableCellProxy::INTERFACE => Ok(Interface::TableCell),
                    TextProxy::INTERFACE => Ok(Interface::Text),
                    ValueProxy::INTERFACE => Ok(Interface::Value),
                    _ => Err(de::Error::custom("unknown interface")),
                }
            }
        }

        deserializer.deserialize_identifier(InterfaceVisitor)
    }
}

#[cfg(feature = "serde")]
impl Serialize for Interface {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            Interface::Accessible => AccessibleProxy::INTERFACE,
            Interface::Action => ActionProxy::INTERFACE,
            Interface::Application => ApplicationProxy::INTERFACE,
            Interface::Cache => CacheProxy::INTERFACE,
            Interface::Collection => CollectionProxy::INTERFACE,
            Interface::Component => ComponentProxy::INTERFACE,
            Interface::DeviceEventController => DeviceEventControllerProxy::INTERFACE,
            Interface::DeviceEventListener => DeviceEventListenerProxy::INTERFACE,
            Interface::Document => DocumentProxy::INTERFACE,
            Interface::EditableText => EditableTextProxy::INTERFACE,
            Interface::Hyperlink => HyperlinkProxy::INTERFACE,
            Interface::Hypertext => HypertextProxy::INTERFACE,
            Interface::Image => ImageProxy::INTERFACE,
            Interface::Registry => RegistryProxy::INTERFACE,
            Interface::Selection => SelectionProxy::INTERFACE,
            Interface::Socket => SocketProxy::INTERFACE,
            Interface::Table => TableProxy::INTERFACE,
            Interface::TableCell => TableCellProxy::INTERFACE,
            Interface::Text => TextProxy::INTERFACE,
            Interface::Value => ValueProxy::INTERFACE,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InterfaceSet(BitFlags<Interface>);

impl InterfaceSet {
    pub fn new<B: Into<BitFlags<Interface>>>(value: B) -> Self {
        Self(value.into())
    }

    pub fn empty() -> InterfaceSet {
        InterfaceSet(Interface::empty())
    }

    pub fn bits(&self) -> u32 {
        self.0.bits()
    }

    pub fn contains<B: Into<BitFlags<Interface>>>(self, other: B) -> bool {
        self.0.contains(other)
    }

    pub fn insert<B: Into<BitFlags<Interface>>>(&mut self, other: B) {
        self.0.insert(other);
    }

    pub fn iter(self) -> impl Iterator<Item = Interface> {
        self.0.iter()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for InterfaceSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct InterfaceSetVisitor;

        impl<'de> Visitor<'de> for InterfaceSetVisitor {
            type Value = InterfaceSet;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence comprised of valid AT-SPI interface names")
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                match <Vec<Interface> as Deserialize>::deserialize(deserializer) {
                    Ok(interfaces) => Ok(InterfaceSet(BitFlags::from_iter(interfaces))),
                    Err(e) => Err(e),
                }
            }
        }

        deserializer.deserialize_newtype_struct("InterfaceSet", InterfaceSetVisitor)
    }
}

#[cfg(feature = "serde")]
impl Serialize for InterfaceSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer
            .serialize_newtype_struct("InterfaceSet", &self.0.iter().collect::<Vec<Interface>>())
    }
}

impl Type for InterfaceSet {
    fn signature() -> Signature<'static> {
        <Vec<String> as Type>::signature()
    }
}

impl From<Interface> for InterfaceSet {
    fn from(value: Interface) -> Self {
        Self(value.into())
    }
}

impl std::ops::BitAnd for InterfaceSet {
    type Output = InterfaceSet;

    fn bitand(self, other: Self) -> Self::Output {
        InterfaceSet(self.0 & other.0)
    }
}

impl std::ops::BitXor for InterfaceSet {
    type Output = InterfaceSet;

    fn bitxor(self, other: Self) -> Self::Output {
        InterfaceSet(self.0 ^ other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::LE;
    use zbus::zvariant::{from_slice, to_bytes, EncodingContext as Context};

    #[test]
    fn serialize_empty_interface_set() {
        let ctxt = Context::<LE>::new_dbus(0);
        let encoded = to_bytes(ctxt, &InterfaceSet::empty()).unwrap();
        assert_eq!(encoded, &[0, 0, 0, 0]);
    }

    #[test]
    fn deserialize_empty_interface_set() {
        let ctxt = Context::<LE>::new_dbus(0);
        let decoded: InterfaceSet = from_slice(&[0, 0, 0, 0], ctxt).unwrap();
        assert_eq!(decoded, InterfaceSet::empty());
    }

    #[test]
    fn serialize_interface_set_accessible() {
        let ctxt = Context::<LE>::new_dbus(0);
        let encoded = to_bytes(ctxt, &InterfaceSet::new(Interface::Accessible)).unwrap();
        assert_eq!(
            encoded,
            &[
                30, 0, 0, 0, 25, 0, 0, 0, 111, 114, 103, 46, 97, 49, 49, 121, 46, 97, 116, 115,
                112, 105, 46, 65, 99, 99, 101, 115, 115, 105, 98, 108, 101, 0
            ]
        );
    }

    #[test]
    fn deserialize_interface_set_accessible() {
        let ctxt = Context::<LE>::new_dbus(0);
        let decoded: InterfaceSet = from_slice(
            &[
                30, 0, 0, 0, 25, 0, 0, 0, 111, 114, 103, 46, 97, 49, 49, 121, 46, 97, 116, 115,
                112, 105, 46, 65, 99, 99, 101, 115, 115, 105, 98, 108, 101, 0,
            ],
            ctxt,
        )
        .unwrap();
        assert_eq!(decoded, InterfaceSet::new(Interface::Accessible));
    }

    #[test]
    fn can_handle_multiple_interfaces() {
        let ctxt = Context::<LE>::new_dbus(0);
        let object =
            InterfaceSet::new(Interface::Accessible | Interface::Action | Interface::Component);
        let encoded = to_bytes(ctxt, &object).unwrap();
        let decoded: InterfaceSet = from_slice(&encoded, ctxt).unwrap();
        assert!(object == decoded);
    }
}
