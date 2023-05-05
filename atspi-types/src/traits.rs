use zbus_names::UniqueName;
use zvariant::{Type, ObjectPath, Value, OwnedValue};
use serde::{Serialize, Deserialize};
use crate::{
	//accessible::Accessible,
	types::Accessible,
	error::AtspiError,
};
use std::collections::HashMap;

/// Shared behavior of bus `Signal` events.
pub trait GenericEvent<'a> {
	const DBUS_MEMBER: &'static str;
	const DBUS_INTERFACE: &'static str;
	const MATCH_RULE_STRING: &'static str;
	const REGISTRY_EVENT_STRING: &'static str;

	/// What is the body type of this event.
	type Body: Type + Serialize + Deserialize<'a>;

	/// Build the event from the object pair (Accessible and the Body).
	fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError>
	where
		Self: Sized;

	/// Path of the signalling object.
	fn path(&self) -> ObjectPath<'_>;

	/// Sender of the signal.
	///
	/// ### Errors
	/// - when deserializeing the header failed, or
	/// - When `zbus::get_field!` finds that 'sender' is an invalid field.
	fn sender(&self) -> UniqueName<'_>;

	/// The body of the object.
	fn body(&self) -> Self::Body;
}
pub trait HasMatchRule {
	const MATCH_RULE_STRING: &'static str;
}

pub trait HasRegistryEventString {
	const REGISTRY_EVENT_STRING: &'static str;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventBody<'a> {
	#[serde(rename = "type")]
	pub kind: &'a str,
	pub detail1: i32,
	pub detail2: i32,
	#[serde(borrow)]
	pub any_data: Value<'a>,
	#[serde(borrow)]
	pub properties: HashMap<&'a str, Value<'a>>,
}

impl Type for EventBody<'_> {
	fn signature() -> Signature<'static> {
		<(&str, i32, i32, Value, HashMap<&str, Value>)>::signature()
	}
}

// Signature:  "siiv(so)",
#[derive(Debug, Serialize, Deserialize, Type)]
pub struct EventBodyQT<'a> {
	#[serde(rename = "type")]
	pub kind: &'a str,
	pub detail1: i32,
	pub detail2: i32,
	#[serde(borrow)]
	pub any_data: Value<'a>,
	#[serde(borrow)]
	pub properties: (&'a str, Value<'a>),
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

impl Default for EventBodyOwned {
	fn default() -> Self {
		EventBodyOwned {
			kind: String::default(),
			detail1: 0,
			detail2: 0,
			any_data: Value::U8(0).into(),
			properties: HashMap::new(),
		}
	}
}
