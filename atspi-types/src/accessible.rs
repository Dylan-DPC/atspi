use std::ops::Deref;
use zvariant::{OwnedObjectPath, ObjectPath, Signature, Type, Value};
use zbus_names::{OwnedUniqueName, UniqueName};
use crate::{
	error::AtspiError,
	types::Accessible,
};
use serde::{Serialize, Deserialize};

// Event body signatures: These outline the event specific deserialized event types.
// Safety: These are evaluated at compile time.
// ----
// The signal signature "(so)" (an Accessible) is ambiguous, because it is used in:
// -  Cache : RemoveAccessible
// -  Socket: Available  *( signals the availability of the `Registry` daeomon.)
//
// ATSPI- and QSPI both describe the generic events. These can be converted into
// specific signal types with TryFrom implementations. See crate::[`identify`]
//  EVENT_LISTENER_SIGNATURE is a type signature used to notify when events are registered or deregistered.
//  CACHE_ADD_SIGNATURE and *_REMOVE have very different types
pub const ATSPI_EVENT_SIGNATURE: Signature<'_> =
	Signature::from_static_str_unchecked("(siiva{sv})");
pub const QSPI_EVENT_SIGNATURE: Signature<'_> = Signature::from_static_str_unchecked("(siiv(so))");
pub const ACCESSIBLE_PAIR_SIGNATURE: Signature<'_> = Signature::from_static_str_unchecked("(so)");
pub const EVENT_LISTENER_SIGNATURE: Signature<'_> = Signature::from_static_str_unchecked("(ss)");
pub const CACHE_ADD_SIGNATURE: Signature<'_> =
	Signature::from_static_str_unchecked("((so)(so)(so)iiassusau)");

/*
// TODO: Try to make borrowed versions work,
// check where the lifetimes of the borrow are tied to, see also: comment on `interface()` method
// in `DefaultEvent` impl
// then rename into Owned for this one.
/// Owned Accessible type
/// Emitted by `CacheRemove` and `Available`
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
pub struct Accessible {
	pub name: OwnedUniqueName,
	pub path: OwnedObjectPath,
}
impl<'a> TryFrom<zvariant::Value<'a>> for Accessible<'a> 
{
	type Error = AtspiError;
	fn try_from(value: zvariant::Value<'a>) -> Result<Self, Self::Error> {
		match value {
			zvariant::Value::Structure(s) => {
				if s.signature() != ACCESSIBLE_PAIR_SIGNATURE {
					return Err(zvariant::Error::SignatureMismatch(s.signature(), format!("To turn a zvariant::Value into an atspi::Accessible, it must be of type {}", ACCESSIBLE_PAIR_SIGNATURE.as_str())).into());
				}
				let fields: Vec<Value<'a>> = s.into_fields();
				let name: &'a str = fields
					.get(0)
					.ok_or::<AtspiError>(zvariant::Error::IncorrectType.into())?
					.try_into()?;
				//let path: &str = fields
				//	.get(1)
				//	.ok_or::<AtspiError>(zvariant::Error::IncorrectType.into())?
				//	.try_into()?;
				Ok(Accessible { name: "name", path: "path" })
			}
			_ => Err(zvariant::Error::IncorrectType.into()),
		}
	}
}
*/
impl<'a> From<Accessible<'a>> for zvariant::Structure<'a> {
	fn from(accessible: Accessible<'a>) -> Self {
		(accessible.name.to_string(), ObjectPath::try_from(accessible.path.to_string()).unwrap()).into()
	}
}
impl Default for Accessible<'_> {
	fn default() -> Self {
		Accessible {
			name: ":0.0",
			path: "/org/a11y/atspi/accessible/null"
		}
	}
}
