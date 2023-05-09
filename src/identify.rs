#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod object {
	use crate::{
		error::AtspiError,
		events::{Accessible, EventBodyOwned, GenericEvent, HasMatchRule, HasRegistryEventString},
		Event,
	};
	use zbus;
	use zbus::names::UniqueName;
	use zbus::zvariant::ObjectPath;

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that this example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// use atspi::identify::object::PropertyChangeEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<PropertyChangeEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let output = std::process::Command::new("busctl")
	/// #       .arg("--user")
	/// #       .arg("call")
	/// #       .arg("org.a11y.Bus")
	/// #       .arg("/org/a11y/bus")
	/// #       .arg("org.a11y.Bus")
	/// #       .arg("GetAddress")
	/// #       .output()
	/// #       .unwrap();
	/// #    let addr_string = String::from_utf8(output.stdout).unwrap();
	/// #    let addr_str = addr_string
	/// #        .strip_prefix("s \"")
	/// #        .unwrap()
	/// #        .trim()
	/// #        .strip_suffix('"')
	/// #        .unwrap();
	/// #   let mut base_cmd = std::process::Command::new("busctl");
	/// #   let thing = base_cmd
	/// #       .arg("--address")
	/// #       .arg(addr_str)
	/// #       .arg("emit")
	/// #       .arg("/org/a11y/atspi/accessible/null")
	/// #       .arg("org.a11y.atspi.Event.Object")
	/// #       .arg("PropertyChange")
	/// #       .arg("siiva{sv}")
	/// #       .arg("")
	/// #       .arg("0")
	/// #       .arg("0")
	/// #       .arg("i")
	/// #       .arg("0")
	/// #       .arg("0")
	/// #       .output()
	/// #       .unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///          if let Event::Object(_event) = ev {
	/// #            break;
	///              // do things with your event here
	///          }
	/// #        else { panic!("Something went wrong receiving the event. Usually this means the wrong event was received.") };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
	pub enum ObjectEvents {
		PropertyChange(PropertyChangeEvent),
		BoundsChanged(BoundsChangedEvent),
		LinkSelected(LinkSelectedEvent),
		StateChanged(StateChangedEvent),
		ChildrenChanged(ChildrenChangedEvent),
		VisibleDataChanged(VisibleDataChangedEvent),
		SelectionChanged(SelectionChangedEvent),
		ModelChanged(ModelChangedEvent),
		ActiveDescendantChanged(ActiveDescendantChangedEvent),
		Announcement(AnnouncementEvent),
		AttributesChanged(AttributesChangedEvent),
		RowInserted(RowInsertedEvent),
		RowReordered(RowReorderedEvent),
		RowDeleted(RowDeletedEvent),
		ColumnInserted(ColumnInsertedEvent),
		ColumnReordered(ColumnReorderedEvent),
		ColumnDeleted(ColumnDeletedEvent),
		TextBoundsChanged(TextBoundsChangedEvent),
		TextSelectionChanged(TextSelectionChangedEvent),
		TextChanged(TextChangedEvent),
		TextAttributesChanged(TextAttributesChangedEvent),
		TextCaretMoved(TextCaretMovedEvent),
	}
	impl_event_conversions!(ObjectEvents, Event::Object);
	event_wrapper_test_cases!(ObjectEvents, PropertyChangeEvent);

	impl HasMatchRule for ObjectEvents {
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object'";
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::PropertyChangeEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<PropertyChangeEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = PropertyChangeEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = PropertyChangeEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct PropertyChangeEvent {
		pub item: crate::events::Accessible,
		pub property: String,
		pub value: String,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::BoundsChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<BoundsChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = BoundsChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = BoundsChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct BoundsChangedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::LinkSelectedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<LinkSelectedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = LinkSelectedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = LinkSelectedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct LinkSelectedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::StateChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<StateChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = StateChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = StateChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct StateChangedEvent {
		pub item: crate::events::Accessible,
		pub state: String,
		pub enabled: i32,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::ChildrenChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ChildrenChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = ChildrenChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = ChildrenChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct ChildrenChangedEvent {
		pub item: crate::events::Accessible,
		pub operation: String,
		pub index_in_parent: i32,
		pub child: Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::VisibleDataChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<VisibleDataChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = VisibleDataChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = VisibleDataChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct VisibleDataChangedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::SelectionChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<SelectionChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = SelectionChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = SelectionChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct SelectionChangedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::ModelChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ModelChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = ModelChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = ModelChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct ModelChangedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::ActiveDescendantChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ActiveDescendantChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = ActiveDescendantChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = ActiveDescendantChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct ActiveDescendantChangedEvent {
		pub item: crate::events::Accessible,
		pub child: Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::AnnouncementEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<AnnouncementEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = AnnouncementEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = AnnouncementEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct AnnouncementEvent {
		pub item: crate::events::Accessible,
		pub text: String,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::AttributesChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<AttributesChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = AttributesChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = AttributesChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct AttributesChangedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::RowInsertedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<RowInsertedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = RowInsertedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = RowInsertedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct RowInsertedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::RowReorderedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<RowReorderedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = RowReorderedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = RowReorderedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct RowReorderedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::RowDeletedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<RowDeletedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = RowDeletedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = RowDeletedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct RowDeletedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::ColumnInsertedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ColumnInsertedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = ColumnInsertedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = ColumnInsertedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct ColumnInsertedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::ColumnReorderedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ColumnReorderedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = ColumnReorderedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = ColumnReorderedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct ColumnReorderedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::ColumnDeletedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ColumnDeletedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = ColumnDeletedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = ColumnDeletedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct ColumnDeletedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::TextBoundsChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<TextBoundsChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = TextBoundsChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = TextBoundsChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct TextBoundsChangedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::TextSelectionChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<TextSelectionChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = TextSelectionChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = TextSelectionChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct TextSelectionChangedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::TextChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<TextChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = TextChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = TextChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct TextChangedEvent {
		pub item: crate::events::Accessible,
		pub detail: String,
		pub start_pos: i32,
		pub length: i32,
		pub text: String,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::TextAttributesChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<TextAttributesChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = TextAttributesChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = TextAttributesChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct TextAttributesChangedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::object::TextCaretMovedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<TextCaretMovedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = TextCaretMovedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = TextCaretMovedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct TextCaretMovedEvent {
		pub item: crate::events::Accessible,
		pub position: i32,
	}

	impl GenericEvent<'_> for PropertyChangeEvent {
		const DBUS_MEMBER: &'static str = "PropertyChange";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='PropertyChange'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item, property: body.kind, value: body.any_data.try_into()? })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for PropertyChangeEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::PropertyChange(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for BoundsChangedEvent {
		const DBUS_MEMBER: &'static str = "BoundsChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='BoundsChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for BoundsChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::BoundsChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for LinkSelectedEvent {
		const DBUS_MEMBER: &'static str = "LinkSelected";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='LinkSelected'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for LinkSelectedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::LinkSelected(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for StateChangedEvent {
		const DBUS_MEMBER: &'static str = "StateChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='StateChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item, state: body.kind, enabled: body.detail1 })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for StateChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::StateChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for ChildrenChangedEvent {
		const DBUS_MEMBER: &'static str = "ChildrenChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='ChildrenChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
				operation: body.kind,
				index_in_parent: body.detail1,
				child: body.any_data.try_into()?,
			})
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for ChildrenChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::ChildrenChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for VisibleDataChangedEvent {
		const DBUS_MEMBER: &'static str = "VisibleDataChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='VisibleDataChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for VisibleDataChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::VisibleDataChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for SelectionChangedEvent {
		const DBUS_MEMBER: &'static str = "SelectionChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='SelectionChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for SelectionChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::SelectionChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for ModelChangedEvent {
		const DBUS_MEMBER: &'static str = "ModelChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='ModelChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for ModelChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::ModelChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for ActiveDescendantChangedEvent {
		const DBUS_MEMBER: &'static str = "ActiveDescendantChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ActiveDescendantChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item, child: body.any_data.try_into()? })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for ActiveDescendantChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::ActiveDescendantChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for AnnouncementEvent {
		const DBUS_MEMBER: &'static str = "Announcement";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='Announcement'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item, text: body.kind })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for AnnouncementEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::Announcement(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for AttributesChangedEvent {
		const DBUS_MEMBER: &'static str = "AttributesChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='AttributesChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for AttributesChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::AttributesChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for RowInsertedEvent {
		const DBUS_MEMBER: &'static str = "RowInserted";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='RowInserted'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for RowInsertedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::RowInserted(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for RowReorderedEvent {
		const DBUS_MEMBER: &'static str = "RowReordered";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='RowReordered'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for RowReorderedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::RowReordered(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for RowDeletedEvent {
		const DBUS_MEMBER: &'static str = "RowDeleted";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='RowDeleted'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for RowDeletedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::RowDeleted(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for ColumnInsertedEvent {
		const DBUS_MEMBER: &'static str = "ColumnInserted";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnInserted'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for ColumnInsertedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::ColumnInserted(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for ColumnReorderedEvent {
		const DBUS_MEMBER: &'static str = "ColumnReordered";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnReordered'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for ColumnReorderedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::ColumnReordered(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for ColumnDeletedEvent {
		const DBUS_MEMBER: &'static str = "ColumnDeleted";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnDeleted'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for ColumnDeletedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::ColumnDeleted(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for TextBoundsChangedEvent {
		const DBUS_MEMBER: &'static str = "TextBoundsChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='TextBoundsChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for TextBoundsChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::TextBoundsChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for TextSelectionChangedEvent {
		const DBUS_MEMBER: &'static str = "TextSelectionChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='TextSelectionChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for TextSelectionChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::TextSelectionChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for TextChangedEvent {
		const DBUS_MEMBER: &'static str = "TextChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='TextChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
				detail: body.kind,
				start_pos: body.detail1,
				length: body.detail2,
				text: body.any_data.try_into()?,
			})
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for TextChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::TextChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for TextAttributesChangedEvent {
		const DBUS_MEMBER: &'static str = "TextAttributesChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='TextAttributesChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for TextAttributesChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::TextAttributesChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for TextCaretMovedEvent {
		const DBUS_MEMBER: &'static str = "TextCaretMoved";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Object',member='TextCaretMoved'";
		const REGISTRY_EVENT_STRING: &'static str = "Object:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item, position: body.detail1 })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for TextCaretMovedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Object(ObjectEvents::TextCaretMoved(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl TryFrom<&zbus::Message> for ObjectEvents {
		type Error = AtspiError;
		fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
			let member = ev
				.member()
				.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
			match member.as_str() {
				"PropertyChange" => Ok(ObjectEvents::PropertyChange(ev.try_into()?)),
				"BoundsChanged" => Ok(ObjectEvents::BoundsChanged(ev.try_into()?)),
				"LinkSelected" => Ok(ObjectEvents::LinkSelected(ev.try_into()?)),
				"StateChanged" => Ok(ObjectEvents::StateChanged(ev.try_into()?)),
				"ChildrenChanged" => Ok(ObjectEvents::ChildrenChanged(ev.try_into()?)),
				"VisibleDataChanged" => Ok(ObjectEvents::VisibleDataChanged(ev.try_into()?)),
				"SelectionChanged" => Ok(ObjectEvents::SelectionChanged(ev.try_into()?)),
				"ModelChanged" => Ok(ObjectEvents::ModelChanged(ev.try_into()?)),
				"ActiveDescendantChanged" => {
					Ok(ObjectEvents::ActiveDescendantChanged(ev.try_into()?))
				}
				"Announcement" => Ok(ObjectEvents::Announcement(ev.try_into()?)),
				"AttributesChanged" => Ok(ObjectEvents::AttributesChanged(ev.try_into()?)),
				"RowInserted" => Ok(ObjectEvents::RowInserted(ev.try_into()?)),
				"RowReordered" => Ok(ObjectEvents::RowReordered(ev.try_into()?)),
				"RowDeleted" => Ok(ObjectEvents::RowDeleted(ev.try_into()?)),
				"ColumnInserted" => Ok(ObjectEvents::ColumnInserted(ev.try_into()?)),
				"ColumnReordered" => Ok(ObjectEvents::ColumnReordered(ev.try_into()?)),
				"ColumnDeleted" => Ok(ObjectEvents::ColumnDeleted(ev.try_into()?)),
				"TextBoundsChanged" => Ok(ObjectEvents::TextBoundsChanged(ev.try_into()?)),
				"TextSelectionChanged" => Ok(ObjectEvents::TextSelectionChanged(ev.try_into()?)),
				"TextChanged" => Ok(ObjectEvents::TextChanged(ev.try_into()?)),
				"TextAttributesChanged" => Ok(ObjectEvents::TextAttributesChanged(ev.try_into()?)),
				"TextCaretMoved" => Ok(ObjectEvents::TextCaretMoved(ev.try_into()?)),
				_ => Err(AtspiError::MemberMatch("No matching member for Object".into())),
			}
		}
	}

	impl_event_conversions!(
		PropertyChangeEvent,
		ObjectEvents,
		ObjectEvents::PropertyChange,
		Event::Object
	);
	event_test_cases!(PropertyChangeEvent);
	impl_to_dbus_message!(PropertyChangeEvent);
	impl_from_dbus_message!(PropertyChangeEvent);
	impl From<PropertyChangeEvent> for EventBodyOwned {
		fn from(event: PropertyChangeEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: event.property,
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::from(event.value).into(),
			}
		}
	}

	impl_event_conversions!(
		BoundsChangedEvent,
		ObjectEvents,
		ObjectEvents::BoundsChanged,
		Event::Object
	);
	event_test_cases!(BoundsChangedEvent);
	impl_to_dbus_message!(BoundsChangedEvent);
	impl_from_dbus_message!(BoundsChangedEvent);
	impl From<BoundsChangedEvent> for EventBodyOwned {
		fn from(_event: BoundsChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		LinkSelectedEvent,
		ObjectEvents,
		ObjectEvents::LinkSelected,
		Event::Object
	);
	event_test_cases!(LinkSelectedEvent);
	impl_to_dbus_message!(LinkSelectedEvent);
	impl_from_dbus_message!(LinkSelectedEvent);
	impl From<LinkSelectedEvent> for EventBodyOwned {
		fn from(_event: LinkSelectedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		StateChangedEvent,
		ObjectEvents,
		ObjectEvents::StateChanged,
		Event::Object
	);
	event_test_cases!(StateChangedEvent);
	impl_to_dbus_message!(StateChangedEvent);
	impl_from_dbus_message!(StateChangedEvent);
	impl From<StateChangedEvent> for EventBodyOwned {
		fn from(event: StateChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: event.state,
				detail1: event.enabled,
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		ChildrenChangedEvent,
		ObjectEvents,
		ObjectEvents::ChildrenChanged,
		Event::Object
	);
	event_test_cases!(ChildrenChangedEvent);
	impl_to_dbus_message!(ChildrenChangedEvent);
	impl_from_dbus_message!(ChildrenChangedEvent);
	impl From<ChildrenChangedEvent> for EventBodyOwned {
		fn from(event: ChildrenChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: event.operation,
				detail1: event.index_in_parent,
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::from(event.child).into(),
			}
		}
	}

	impl_event_conversions!(
		VisibleDataChangedEvent,
		ObjectEvents,
		ObjectEvents::VisibleDataChanged,
		Event::Object
	);
	event_test_cases!(VisibleDataChangedEvent);
	impl_to_dbus_message!(VisibleDataChangedEvent);
	impl_from_dbus_message!(VisibleDataChangedEvent);
	impl From<VisibleDataChangedEvent> for EventBodyOwned {
		fn from(_event: VisibleDataChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		SelectionChangedEvent,
		ObjectEvents,
		ObjectEvents::SelectionChanged,
		Event::Object
	);
	event_test_cases!(SelectionChangedEvent);
	impl_to_dbus_message!(SelectionChangedEvent);
	impl_from_dbus_message!(SelectionChangedEvent);
	impl From<SelectionChangedEvent> for EventBodyOwned {
		fn from(_event: SelectionChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		ModelChangedEvent,
		ObjectEvents,
		ObjectEvents::ModelChanged,
		Event::Object
	);
	event_test_cases!(ModelChangedEvent);
	impl_to_dbus_message!(ModelChangedEvent);
	impl_from_dbus_message!(ModelChangedEvent);
	impl From<ModelChangedEvent> for EventBodyOwned {
		fn from(_event: ModelChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		ActiveDescendantChangedEvent,
		ObjectEvents,
		ObjectEvents::ActiveDescendantChanged,
		Event::Object
	);
	event_test_cases!(ActiveDescendantChangedEvent);
	impl_to_dbus_message!(ActiveDescendantChangedEvent);
	impl_from_dbus_message!(ActiveDescendantChangedEvent);
	impl From<ActiveDescendantChangedEvent> for EventBodyOwned {
		fn from(event: ActiveDescendantChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::from(event.child).into(),
			}
		}
	}

	impl_event_conversions!(
		AnnouncementEvent,
		ObjectEvents,
		ObjectEvents::Announcement,
		Event::Object
	);
	event_test_cases!(AnnouncementEvent);
	impl_to_dbus_message!(AnnouncementEvent);
	impl_from_dbus_message!(AnnouncementEvent);
	impl From<AnnouncementEvent> for EventBodyOwned {
		fn from(event: AnnouncementEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: event.text,
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		AttributesChangedEvent,
		ObjectEvents,
		ObjectEvents::AttributesChanged,
		Event::Object
	);
	event_test_cases!(AttributesChangedEvent);
	impl_to_dbus_message!(AttributesChangedEvent);
	impl_from_dbus_message!(AttributesChangedEvent);
	impl From<AttributesChangedEvent> for EventBodyOwned {
		fn from(_event: AttributesChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		RowInsertedEvent,
		ObjectEvents,
		ObjectEvents::RowInserted,
		Event::Object
	);
	event_test_cases!(RowInsertedEvent);
	impl_to_dbus_message!(RowInsertedEvent);
	impl_from_dbus_message!(RowInsertedEvent);
	impl From<RowInsertedEvent> for EventBodyOwned {
		fn from(_event: RowInsertedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		RowReorderedEvent,
		ObjectEvents,
		ObjectEvents::RowReordered,
		Event::Object
	);
	event_test_cases!(RowReorderedEvent);
	impl_to_dbus_message!(RowReorderedEvent);
	impl_from_dbus_message!(RowReorderedEvent);
	impl From<RowReorderedEvent> for EventBodyOwned {
		fn from(_event: RowReorderedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(RowDeletedEvent, ObjectEvents, ObjectEvents::RowDeleted, Event::Object);
	event_test_cases!(RowDeletedEvent);
	impl_to_dbus_message!(RowDeletedEvent);
	impl_from_dbus_message!(RowDeletedEvent);
	impl From<RowDeletedEvent> for EventBodyOwned {
		fn from(_event: RowDeletedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		ColumnInsertedEvent,
		ObjectEvents,
		ObjectEvents::ColumnInserted,
		Event::Object
	);
	event_test_cases!(ColumnInsertedEvent);
	impl_to_dbus_message!(ColumnInsertedEvent);
	impl_from_dbus_message!(ColumnInsertedEvent);
	impl From<ColumnInsertedEvent> for EventBodyOwned {
		fn from(_event: ColumnInsertedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		ColumnReorderedEvent,
		ObjectEvents,
		ObjectEvents::ColumnReordered,
		Event::Object
	);
	event_test_cases!(ColumnReorderedEvent);
	impl_to_dbus_message!(ColumnReorderedEvent);
	impl_from_dbus_message!(ColumnReorderedEvent);
	impl From<ColumnReorderedEvent> for EventBodyOwned {
		fn from(_event: ColumnReorderedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		ColumnDeletedEvent,
		ObjectEvents,
		ObjectEvents::ColumnDeleted,
		Event::Object
	);
	event_test_cases!(ColumnDeletedEvent);
	impl_to_dbus_message!(ColumnDeletedEvent);
	impl_from_dbus_message!(ColumnDeletedEvent);
	impl From<ColumnDeletedEvent> for EventBodyOwned {
		fn from(_event: ColumnDeletedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		TextBoundsChangedEvent,
		ObjectEvents,
		ObjectEvents::TextBoundsChanged,
		Event::Object
	);
	event_test_cases!(TextBoundsChangedEvent);
	impl_to_dbus_message!(TextBoundsChangedEvent);
	impl_from_dbus_message!(TextBoundsChangedEvent);
	impl From<TextBoundsChangedEvent> for EventBodyOwned {
		fn from(_event: TextBoundsChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		TextSelectionChangedEvent,
		ObjectEvents,
		ObjectEvents::TextSelectionChanged,
		Event::Object
	);
	event_test_cases!(TextSelectionChangedEvent);
	impl_to_dbus_message!(TextSelectionChangedEvent);
	impl_from_dbus_message!(TextSelectionChangedEvent);
	impl From<TextSelectionChangedEvent> for EventBodyOwned {
		fn from(_event: TextSelectionChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		TextChangedEvent,
		ObjectEvents,
		ObjectEvents::TextChanged,
		Event::Object
	);
	event_test_cases!(TextChangedEvent);
	impl_to_dbus_message!(TextChangedEvent);
	impl_from_dbus_message!(TextChangedEvent);
	impl From<TextChangedEvent> for EventBodyOwned {
		fn from(event: TextChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: event.detail,
				detail1: event.start_pos,
				detail2: event.length,
				any_data: zbus::zvariant::Value::from(event.text).into(),
			}
		}
	}

	impl_event_conversions!(
		TextAttributesChangedEvent,
		ObjectEvents,
		ObjectEvents::TextAttributesChanged,
		Event::Object
	);
	event_test_cases!(TextAttributesChangedEvent);
	impl_to_dbus_message!(TextAttributesChangedEvent);
	impl_from_dbus_message!(TextAttributesChangedEvent);
	impl From<TextAttributesChangedEvent> for EventBodyOwned {
		fn from(_event: TextAttributesChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		TextCaretMovedEvent,
		ObjectEvents,
		ObjectEvents::TextCaretMoved,
		Event::Object
	);
	event_test_cases!(TextCaretMovedEvent);
	impl_to_dbus_message!(TextCaretMovedEvent);
	impl_from_dbus_message!(TextCaretMovedEvent);
	impl From<TextCaretMovedEvent> for EventBodyOwned {
		fn from(event: TextCaretMovedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: event.position,
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	/*impl HasMatchRule for PropertyChangeEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='PropertyChange'";
	}*/
	/*impl HasMatchRule for BoundsChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='BoundsChanged'";
	}*/
	/*impl HasMatchRule for LinkSelectedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='LinkSelected'";
	}*/
	/*impl HasMatchRule for StateChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='StateChanged'";
	}*/
	/*impl HasMatchRule for ChildrenChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ChildrenChanged'";
	}*/
	/*impl HasMatchRule for VisibleDataChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='VisibleDataChanged'";
	}*/
	/*impl HasMatchRule for SelectionChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='SelectionChanged'";
	}*/
	/*impl HasMatchRule for ModelChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ModelChanged'";
	}*/
	/*impl HasMatchRule for ActiveDescendantChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ActiveDescendantChanged'";
	}*/
	/*impl HasMatchRule for AnnouncementEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='Announcement'";
	}*/
	/*impl HasMatchRule for AttributesChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='AttributesChanged'";
	}*/
	/*impl HasMatchRule for RowInsertedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='RowInserted'";
	}*/
	/*impl HasMatchRule for RowReorderedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='RowReordered'";
	}*/
	/*impl HasMatchRule for RowDeletedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='RowDeleted'";
	}*/
	/*impl HasMatchRule for ColumnInsertedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnInserted'";
	}*/
	/*impl HasMatchRule for ColumnReorderedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnReordered'";
	}*/
	/*impl HasMatchRule for ColumnDeletedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnDeleted'";
	}*/
	/*impl HasMatchRule for TextBoundsChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='TextBoundsChanged'";
	}*/
	/*impl HasMatchRule for TextSelectionChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='TextSelectionChanged'";
	}*/
	/*impl HasMatchRule for TextChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='TextChanged'";
	}*/
	/*impl HasMatchRule for TextAttributesChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='TextAttributesChanged'";
	}*/
	/*impl HasMatchRule for TextCaretMovedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='TextCaretMoved'";
	}*/
	/*impl HasRegistryEventString for PropertyChangeEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:PropertyChange";
	}*/
	/*impl HasRegistryEventString for BoundsChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:BoundsChanged";
	}*/
	/*impl HasRegistryEventString for LinkSelectedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:LinkSelected";
	}*/
	/*impl HasRegistryEventString for StateChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:StateChanged";
	}*/
	/*impl HasRegistryEventString for ChildrenChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:ChildrenChanged";
	}*/
	/*impl HasRegistryEventString for VisibleDataChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:VisibleDataChanged";
	}*/
	/*impl HasRegistryEventString for SelectionChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:SelectionChanged";
	}*/
	/*impl HasRegistryEventString for ModelChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:ModelChanged";
	}*/
	/*impl HasRegistryEventString for ActiveDescendantChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:ActiveDescendantChanged";
	}*/
	/*impl HasRegistryEventString for AnnouncementEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:Announcement";
	}*/
	/*impl HasRegistryEventString for AttributesChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:AttributesChanged";
	}*/
	/*impl HasRegistryEventString for RowInsertedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:RowInserted";
	}*/
	/*impl HasRegistryEventString for RowReorderedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:RowReordered";
	}*/
	/*impl HasRegistryEventString for RowDeletedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:RowDeleted";
	}*/
	/*impl HasRegistryEventString for ColumnInsertedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:ColumnInserted";
	}*/
	/*impl HasRegistryEventString for ColumnReorderedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:ColumnReordered";
	}*/
	/*impl HasRegistryEventString for ColumnDeletedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:ColumnDeleted";
	}*/
	/*impl HasRegistryEventString for TextBoundsChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:TextBoundsChanged";
	}*/
	/*impl HasRegistryEventString for TextSelectionChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:TextSelectionChanged";
	}*/
	/*impl HasRegistryEventString for TextChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:TextChanged";
	}*/
	/*impl HasRegistryEventString for TextAttributesChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:TextAttributesChanged";
	}*/
	/*impl HasRegistryEventString for TextCaretMovedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Object:TextCaretMoved";
	}*/
	impl HasRegistryEventString for ObjectEvents {
		const REGISTRY_EVENT_STRING: &'static str = "Object:";
	}
}

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod window {
	use crate::{
		error::AtspiError,
		events::{Accessible, EventBodyOwned, GenericEvent, HasMatchRule, HasRegistryEventString},
		Event,
	};
	use zbus;
	use zbus::names::UniqueName;
	use zbus::zvariant::ObjectPath;

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that this example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// use atspi::identify::window::PropertyChangeEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<PropertyChangeEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let output = std::process::Command::new("busctl")
	/// #       .arg("--user")
	/// #       .arg("call")
	/// #       .arg("org.a11y.Bus")
	/// #       .arg("/org/a11y/bus")
	/// #       .arg("org.a11y.Bus")
	/// #       .arg("GetAddress")
	/// #       .output()
	/// #       .unwrap();
	/// #    let addr_string = String::from_utf8(output.stdout).unwrap();
	/// #    let addr_str = addr_string
	/// #        .strip_prefix("s \"")
	/// #        .unwrap()
	/// #        .trim()
	/// #        .strip_suffix('"')
	/// #        .unwrap();
	/// #   let mut base_cmd = std::process::Command::new("busctl");
	/// #   let thing = base_cmd
	/// #       .arg("--address")
	/// #       .arg(addr_str)
	/// #       .arg("emit")
	/// #       .arg("/org/a11y/atspi/accessible/null")
	/// #       .arg("org.a11y.atspi.Event.Window")
	/// #       .arg("PropertyChange")
	/// #       .arg("siiva{sv}")
	/// #       .arg("")
	/// #       .arg("0")
	/// #       .arg("0")
	/// #       .arg("i")
	/// #       .arg("0")
	/// #       .arg("0")
	/// #       .output()
	/// #       .unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///          if let Event::Window(_event) = ev {
	/// #            break;
	///              // do things with your event here
	///          }
	/// #        else { panic!("Something went wrong receiving the event. Usually this means the wrong event was received.") };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
	pub enum WindowEvents {
		PropertyChange(PropertyChangeEvent),
		Minimize(MinimizeEvent),
		Maximize(MaximizeEvent),
		Restore(RestoreEvent),
		Close(CloseEvent),
		Create(CreateEvent),
		Reparent(ReparentEvent),
		DesktopCreate(DesktopCreateEvent),
		DesktopDestroy(DesktopDestroyEvent),
		Destroy(DestroyEvent),
		Activate(ActivateEvent),
		Deactivate(DeactivateEvent),
		Raise(RaiseEvent),
		Lower(LowerEvent),
		Move(MoveEvent),
		Resize(ResizeEvent),
		Shade(ShadeEvent),
		UUshade(UUshadeEvent),
		Restyle(RestyleEvent),
	}
	impl_event_conversions!(WindowEvents, Event::Window);
	event_wrapper_test_cases!(WindowEvents, MoveEvent);

	impl HasMatchRule for WindowEvents {
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window'";
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::PropertyChangeEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<PropertyChangeEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = PropertyChangeEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = PropertyChangeEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct PropertyChangeEvent {
		pub item: crate::events::Accessible,
		pub property: String,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::MinimizeEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<MinimizeEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = MinimizeEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = MinimizeEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct MinimizeEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::MaximizeEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<MaximizeEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = MaximizeEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = MaximizeEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct MaximizeEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::RestoreEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<RestoreEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = RestoreEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = RestoreEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct RestoreEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::CloseEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<CloseEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = CloseEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = CloseEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct CloseEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::CreateEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<CreateEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = CreateEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = CreateEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct CreateEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::ReparentEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ReparentEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = ReparentEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = ReparentEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct ReparentEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::DesktopCreateEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<DesktopCreateEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = DesktopCreateEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = DesktopCreateEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct DesktopCreateEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::DesktopDestroyEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<DesktopDestroyEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = DesktopDestroyEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = DesktopDestroyEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct DesktopDestroyEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::DestroyEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<DestroyEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = DestroyEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = DestroyEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct DestroyEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::ActivateEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ActivateEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = ActivateEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = ActivateEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct ActivateEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::DeactivateEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<DeactivateEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = DeactivateEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = DeactivateEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct DeactivateEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::RaiseEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<RaiseEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = RaiseEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = RaiseEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct RaiseEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::LowerEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<LowerEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = LowerEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = LowerEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct LowerEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::MoveEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<MoveEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = MoveEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = MoveEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct MoveEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::ResizeEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ResizeEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = ResizeEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = ResizeEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct ResizeEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::ShadeEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ShadeEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = ShadeEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = ShadeEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct ShadeEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::UUshadeEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<UUshadeEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = UUshadeEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = UUshadeEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct UUshadeEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::window::RestyleEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<RestyleEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = RestyleEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = RestyleEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct RestyleEvent {
		pub item: crate::events::Accessible,
	}

	impl GenericEvent<'_> for PropertyChangeEvent {
		const DBUS_MEMBER: &'static str = "PropertyChange";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='PropertyChange'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item, property: body.kind })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for PropertyChangeEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::PropertyChange(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for MinimizeEvent {
		const DBUS_MEMBER: &'static str = "Minimize";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Minimize'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for MinimizeEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::Minimize(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for MaximizeEvent {
		const DBUS_MEMBER: &'static str = "Maximize";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Maximize'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for MaximizeEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::Maximize(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for RestoreEvent {
		const DBUS_MEMBER: &'static str = "Restore";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Restore'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for RestoreEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::Restore(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for CloseEvent {
		const DBUS_MEMBER: &'static str = "Close";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Close'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for CloseEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::Close(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for CreateEvent {
		const DBUS_MEMBER: &'static str = "Create";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Create'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for CreateEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::Create(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for ReparentEvent {
		const DBUS_MEMBER: &'static str = "Reparent";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Reparent'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for ReparentEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::Reparent(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for DesktopCreateEvent {
		const DBUS_MEMBER: &'static str = "DesktopCreate";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopCreate'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for DesktopCreateEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::DesktopCreate(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for DesktopDestroyEvent {
		const DBUS_MEMBER: &'static str = "DesktopDestroy";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopDestroy'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for DesktopDestroyEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::DesktopDestroy(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for DestroyEvent {
		const DBUS_MEMBER: &'static str = "Destroy";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Destroy'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for DestroyEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::Destroy(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for ActivateEvent {
		const DBUS_MEMBER: &'static str = "Activate";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Activate'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for ActivateEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::Activate(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for DeactivateEvent {
		const DBUS_MEMBER: &'static str = "Deactivate";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Deactivate'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for DeactivateEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::Deactivate(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for RaiseEvent {
		const DBUS_MEMBER: &'static str = "Raise";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Raise'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for RaiseEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::Raise(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for LowerEvent {
		const DBUS_MEMBER: &'static str = "Lower";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Lower'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for LowerEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::Lower(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for MoveEvent {
		const DBUS_MEMBER: &'static str = "Move";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Move'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for MoveEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::Move(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for ResizeEvent {
		const DBUS_MEMBER: &'static str = "Resize";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Resize'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for ResizeEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::Resize(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for ShadeEvent {
		const DBUS_MEMBER: &'static str = "Shade";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Shade'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for ShadeEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::Shade(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for UUshadeEvent {
		const DBUS_MEMBER: &'static str = "uUshade";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='uUshade'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for UUshadeEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::UUshade(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for RestyleEvent {
		const DBUS_MEMBER: &'static str = "Restyle";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Window',member='Restyle'";
		const REGISTRY_EVENT_STRING: &'static str = "Window:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for RestyleEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Window(WindowEvents::Restyle(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl TryFrom<&zbus::Message> for WindowEvents {
		type Error = AtspiError;
		fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
			let member = ev
				.member()
				.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
			match member.as_str() {
				"PropertyChange" => Ok(WindowEvents::PropertyChange(ev.try_into()?)),
				"Minimize" => Ok(WindowEvents::Minimize(ev.try_into()?)),
				"Maximize" => Ok(WindowEvents::Maximize(ev.try_into()?)),
				"Restore" => Ok(WindowEvents::Restore(ev.try_into()?)),
				"Close" => Ok(WindowEvents::Close(ev.try_into()?)),
				"Create" => Ok(WindowEvents::Create(ev.try_into()?)),
				"Reparent" => Ok(WindowEvents::Reparent(ev.try_into()?)),
				"DesktopCreate" => Ok(WindowEvents::DesktopCreate(ev.try_into()?)),
				"DesktopDestroy" => Ok(WindowEvents::DesktopDestroy(ev.try_into()?)),
				"Destroy" => Ok(WindowEvents::Destroy(ev.try_into()?)),
				"Activate" => Ok(WindowEvents::Activate(ev.try_into()?)),
				"Deactivate" => Ok(WindowEvents::Deactivate(ev.try_into()?)),
				"Raise" => Ok(WindowEvents::Raise(ev.try_into()?)),
				"Lower" => Ok(WindowEvents::Lower(ev.try_into()?)),
				"Move" => Ok(WindowEvents::Move(ev.try_into()?)),
				"Resize" => Ok(WindowEvents::Resize(ev.try_into()?)),
				"Shade" => Ok(WindowEvents::Shade(ev.try_into()?)),
				"uUshade" => Ok(WindowEvents::UUshade(ev.try_into()?)),
				"Restyle" => Ok(WindowEvents::Restyle(ev.try_into()?)),
				_ => Err(AtspiError::MemberMatch("No matching member for Window".into())),
			}
		}
	}

	impl_event_conversions!(
		PropertyChangeEvent,
		WindowEvents,
		WindowEvents::PropertyChange,
		Event::Window
	);
	event_test_cases!(PropertyChangeEvent);
	impl_to_dbus_message!(PropertyChangeEvent);
	impl_from_dbus_message!(PropertyChangeEvent);
	impl From<PropertyChangeEvent> for EventBodyOwned {
		fn from(event: PropertyChangeEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: event.property,
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(MinimizeEvent, WindowEvents, WindowEvents::Minimize, Event::Window);
	event_test_cases!(MinimizeEvent);
	impl_to_dbus_message!(MinimizeEvent);
	impl_from_dbus_message!(MinimizeEvent);
	impl From<MinimizeEvent> for EventBodyOwned {
		fn from(_event: MinimizeEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(MaximizeEvent, WindowEvents, WindowEvents::Maximize, Event::Window);
	event_test_cases!(MaximizeEvent);
	impl_to_dbus_message!(MaximizeEvent);
	impl_from_dbus_message!(MaximizeEvent);
	impl From<MaximizeEvent> for EventBodyOwned {
		fn from(_event: MaximizeEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(RestoreEvent, WindowEvents, WindowEvents::Restore, Event::Window);
	event_test_cases!(RestoreEvent);
	impl_to_dbus_message!(RestoreEvent);
	impl_from_dbus_message!(RestoreEvent);
	impl From<RestoreEvent> for EventBodyOwned {
		fn from(_event: RestoreEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(CloseEvent, WindowEvents, WindowEvents::Close, Event::Window);
	event_test_cases!(CloseEvent);
	impl_to_dbus_message!(CloseEvent);
	impl_from_dbus_message!(CloseEvent);
	impl From<CloseEvent> for EventBodyOwned {
		fn from(_event: CloseEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(CreateEvent, WindowEvents, WindowEvents::Create, Event::Window);
	event_test_cases!(CreateEvent);
	impl_to_dbus_message!(CreateEvent);
	impl_from_dbus_message!(CreateEvent);
	impl From<CreateEvent> for EventBodyOwned {
		fn from(_event: CreateEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(ReparentEvent, WindowEvents, WindowEvents::Reparent, Event::Window);
	event_test_cases!(ReparentEvent);
	impl_to_dbus_message!(ReparentEvent);
	impl_from_dbus_message!(ReparentEvent);
	impl From<ReparentEvent> for EventBodyOwned {
		fn from(_event: ReparentEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		DesktopCreateEvent,
		WindowEvents,
		WindowEvents::DesktopCreate,
		Event::Window
	);
	event_test_cases!(DesktopCreateEvent);
	impl_to_dbus_message!(DesktopCreateEvent);
	impl_from_dbus_message!(DesktopCreateEvent);
	impl From<DesktopCreateEvent> for EventBodyOwned {
		fn from(_event: DesktopCreateEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		DesktopDestroyEvent,
		WindowEvents,
		WindowEvents::DesktopDestroy,
		Event::Window
	);
	event_test_cases!(DesktopDestroyEvent);
	impl_to_dbus_message!(DesktopDestroyEvent);
	impl_from_dbus_message!(DesktopDestroyEvent);
	impl From<DesktopDestroyEvent> for EventBodyOwned {
		fn from(_event: DesktopDestroyEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(DestroyEvent, WindowEvents, WindowEvents::Destroy, Event::Window);
	event_test_cases!(DestroyEvent);
	impl_to_dbus_message!(DestroyEvent);
	impl_from_dbus_message!(DestroyEvent);
	impl From<DestroyEvent> for EventBodyOwned {
		fn from(_event: DestroyEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(ActivateEvent, WindowEvents, WindowEvents::Activate, Event::Window);
	event_test_cases!(ActivateEvent);
	impl_to_dbus_message!(ActivateEvent);
	impl_from_dbus_message!(ActivateEvent);
	impl From<ActivateEvent> for EventBodyOwned {
		fn from(_event: ActivateEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(DeactivateEvent, WindowEvents, WindowEvents::Deactivate, Event::Window);
	event_test_cases!(DeactivateEvent);
	impl_to_dbus_message!(DeactivateEvent);
	impl_from_dbus_message!(DeactivateEvent);
	impl From<DeactivateEvent> for EventBodyOwned {
		fn from(_event: DeactivateEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(RaiseEvent, WindowEvents, WindowEvents::Raise, Event::Window);
	event_test_cases!(RaiseEvent);
	impl_to_dbus_message!(RaiseEvent);
	impl_from_dbus_message!(RaiseEvent);
	impl From<RaiseEvent> for EventBodyOwned {
		fn from(_event: RaiseEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(LowerEvent, WindowEvents, WindowEvents::Lower, Event::Window);
	event_test_cases!(LowerEvent);
	impl_to_dbus_message!(LowerEvent);
	impl_from_dbus_message!(LowerEvent);
	impl From<LowerEvent> for EventBodyOwned {
		fn from(_event: LowerEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(MoveEvent, WindowEvents, WindowEvents::Move, Event::Window);
	event_test_cases!(MoveEvent);
	impl_to_dbus_message!(MoveEvent);
	impl_from_dbus_message!(MoveEvent);
	impl From<MoveEvent> for EventBodyOwned {
		fn from(_event: MoveEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(ResizeEvent, WindowEvents, WindowEvents::Resize, Event::Window);
	event_test_cases!(ResizeEvent);
	impl_to_dbus_message!(ResizeEvent);
	impl_from_dbus_message!(ResizeEvent);
	impl From<ResizeEvent> for EventBodyOwned {
		fn from(_event: ResizeEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(ShadeEvent, WindowEvents, WindowEvents::Shade, Event::Window);
	event_test_cases!(ShadeEvent);
	impl_to_dbus_message!(ShadeEvent);
	impl_from_dbus_message!(ShadeEvent);
	impl From<ShadeEvent> for EventBodyOwned {
		fn from(_event: ShadeEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(UUshadeEvent, WindowEvents, WindowEvents::UUshade, Event::Window);
	event_test_cases!(UUshadeEvent);
	impl_to_dbus_message!(UUshadeEvent);
	impl_from_dbus_message!(UUshadeEvent);
	impl From<UUshadeEvent> for EventBodyOwned {
		fn from(_event: UUshadeEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(RestyleEvent, WindowEvents, WindowEvents::Restyle, Event::Window);
	event_test_cases!(RestyleEvent);
	impl_to_dbus_message!(RestyleEvent);
	impl_from_dbus_message!(RestyleEvent);
	impl From<RestyleEvent> for EventBodyOwned {
		fn from(_event: RestyleEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	/*impl HasMatchRule for PropertyChangeEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='PropertyChange'";
	}*/
	/*impl HasMatchRule for MinimizeEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Minimize'";
	}*/
	/*impl HasMatchRule for MaximizeEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Maximize'";
	}*/
	/*impl HasMatchRule for RestoreEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Restore'";
	}*/
	/*impl HasMatchRule for CloseEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Close'";
	}*/
	/*impl HasMatchRule for CreateEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Create'";
	}*/
	/*impl HasMatchRule for ReparentEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Reparent'";
	}*/
	/*impl HasMatchRule for DesktopCreateEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopCreate'";
	}*/
	/*impl HasMatchRule for DesktopDestroyEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopDestroy'";
	}*/
	/*impl HasMatchRule for DestroyEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Destroy'";
	}*/
	/*impl HasMatchRule for ActivateEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Activate'";
	}*/
	/*impl HasMatchRule for DeactivateEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Deactivate'";
	}*/
	/*impl HasMatchRule for RaiseEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Raise'";
	}*/
	/*impl HasMatchRule for LowerEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Lower'";
	}*/
	/*impl HasMatchRule for MoveEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Move'";
	}*/
	/*impl HasMatchRule for ResizeEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Resize'";
	}*/
	/*impl HasMatchRule for ShadeEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Shade'";
	}*/
	/*impl HasMatchRule for UUshadeEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='uUshade'";
	}*/
	/*impl HasMatchRule for RestyleEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Restyle'";
	}*/
	/*impl HasRegistryEventString for PropertyChangeEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:PropertyChange";
	}*/
	/*impl HasRegistryEventString for MinimizeEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:Minimize";
	}*/
	/*impl HasRegistryEventString for MaximizeEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:Maximize";
	}*/
	/*impl HasRegistryEventString for RestoreEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:Restore";
	}*/
	/*impl HasRegistryEventString for CloseEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:Close";
	}*/
	/*impl HasRegistryEventString for CreateEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:Create";
	}*/
	/*impl HasRegistryEventString for ReparentEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:Reparent";
	}*/
	/*impl HasRegistryEventString for DesktopCreateEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:DesktopCreate";
	}*/
	/*impl HasRegistryEventString for DesktopDestroyEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:DesktopDestroy";
	}*/
	/*impl HasRegistryEventString for DestroyEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:Destroy";
	}*/
	/*impl HasRegistryEventString for ActivateEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:Activate";
	}*/
	/*impl HasRegistryEventString for DeactivateEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:Deactivate";
	}*/
	/*impl HasRegistryEventString for RaiseEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:Raise";
	}*/
	/*impl HasRegistryEventString for LowerEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:Lower";
	}*/
	/*impl HasRegistryEventString for MoveEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:Move";
	}*/
	/*impl HasRegistryEventString for ResizeEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:Resize";
	}*/
	/*impl HasRegistryEventString for ShadeEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:Shade";
	}*/
	/*impl HasRegistryEventString for UUshadeEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:uUshade";
	}*/
	/*impl HasRegistryEventString for RestyleEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Window:Restyle";
	}*/
	impl HasRegistryEventString for WindowEvents {
		const REGISTRY_EVENT_STRING: &'static str = "Window:";
	}
}

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod mouse {
	use crate::{
		error::AtspiError,
		events::{Accessible, EventBodyOwned, GenericEvent, HasMatchRule, HasRegistryEventString},
		Event,
	};
	use zbus;
	use zbus::names::UniqueName;
	use zbus::zvariant::ObjectPath;

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that this example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// use atspi::identify::mouse::AbsEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<AbsEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let output = std::process::Command::new("busctl")
	/// #       .arg("--user")
	/// #       .arg("call")
	/// #       .arg("org.a11y.Bus")
	/// #       .arg("/org/a11y/bus")
	/// #       .arg("org.a11y.Bus")
	/// #       .arg("GetAddress")
	/// #       .output()
	/// #       .unwrap();
	/// #    let addr_string = String::from_utf8(output.stdout).unwrap();
	/// #    let addr_str = addr_string
	/// #        .strip_prefix("s \"")
	/// #        .unwrap()
	/// #        .trim()
	/// #        .strip_suffix('"')
	/// #        .unwrap();
	/// #   let mut base_cmd = std::process::Command::new("busctl");
	/// #   let thing = base_cmd
	/// #       .arg("--address")
	/// #       .arg(addr_str)
	/// #       .arg("emit")
	/// #       .arg("/org/a11y/atspi/accessible/null")
	/// #       .arg("org.a11y.atspi.Event.Mouse")
	/// #       .arg("Abs")
	/// #       .arg("siiva{sv}")
	/// #       .arg("")
	/// #       .arg("0")
	/// #       .arg("0")
	/// #       .arg("i")
	/// #       .arg("0")
	/// #       .arg("0")
	/// #       .output()
	/// #       .unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///          if let Event::Mouse(_event) = ev {
	/// #            break;
	///              // do things with your event here
	///          }
	/// #        else { panic!("Something went wrong receiving the event. Usually this means the wrong event was received.") };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
	pub enum MouseEvents {
		Abs(AbsEvent),
		Rel(RelEvent),
		Button(ButtonEvent),
	}
	impl_event_conversions!(MouseEvents, Event::Mouse);
	event_wrapper_test_cases!(MouseEvents, AbsEvent);

	impl HasMatchRule for MouseEvents {
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Mouse'";
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::mouse::AbsEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<AbsEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = AbsEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = AbsEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct AbsEvent {
		pub item: crate::events::Accessible,
		pub x: i32,
		pub y: i32,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::mouse::RelEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<RelEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = RelEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = RelEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct RelEvent {
		pub item: crate::events::Accessible,
		pub x: i32,
		pub y: i32,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::mouse::ButtonEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ButtonEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = ButtonEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = ButtonEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct ButtonEvent {
		pub item: crate::events::Accessible,
		pub detail: String,
		pub mouse_x: i32,
		pub mouse_y: i32,
	}

	impl GenericEvent<'_> for AbsEvent {
		const DBUS_MEMBER: &'static str = "Abs";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Abs'";
		const REGISTRY_EVENT_STRING: &'static str = "Mouse:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item, x: body.detail1, y: body.detail2 })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for AbsEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Mouse(MouseEvents::Abs(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for RelEvent {
		const DBUS_MEMBER: &'static str = "Rel";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Rel'";
		const REGISTRY_EVENT_STRING: &'static str = "Mouse:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item, x: body.detail1, y: body.detail2 })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for RelEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Mouse(MouseEvents::Rel(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for ButtonEvent {
		const DBUS_MEMBER: &'static str = "Button";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Mouse',member='Button'";
		const REGISTRY_EVENT_STRING: &'static str = "Mouse:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item, detail: body.kind, mouse_x: body.detail1, mouse_y: body.detail2 })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for ButtonEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Mouse(MouseEvents::Button(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl TryFrom<&zbus::Message> for MouseEvents {
		type Error = AtspiError;
		fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
			let member = ev
				.member()
				.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
			match member.as_str() {
				"Abs" => Ok(MouseEvents::Abs(ev.try_into()?)),
				"Rel" => Ok(MouseEvents::Rel(ev.try_into()?)),
				"Button" => Ok(MouseEvents::Button(ev.try_into()?)),
				_ => Err(AtspiError::MemberMatch("No matching member for Mouse".into())),
			}
		}
	}

	impl_event_conversions!(AbsEvent, MouseEvents, MouseEvents::Abs, Event::Mouse);
	event_test_cases!(AbsEvent);
	impl_to_dbus_message!(AbsEvent);
	impl_from_dbus_message!(AbsEvent);
	impl From<AbsEvent> for EventBodyOwned {
		fn from(event: AbsEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: event.x,
				detail2: event.y,
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(RelEvent, MouseEvents, MouseEvents::Rel, Event::Mouse);
	event_test_cases!(RelEvent);
	impl_to_dbus_message!(RelEvent);
	impl_from_dbus_message!(RelEvent);
	impl From<RelEvent> for EventBodyOwned {
		fn from(event: RelEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: event.x,
				detail2: event.y,
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(ButtonEvent, MouseEvents, MouseEvents::Button, Event::Mouse);
	event_test_cases!(ButtonEvent);
	impl_to_dbus_message!(ButtonEvent);
	impl_from_dbus_message!(ButtonEvent);
	impl From<ButtonEvent> for EventBodyOwned {
		fn from(event: ButtonEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: event.detail,
				detail1: event.mouse_x,
				detail2: event.mouse_y,
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	/*impl HasMatchRule for AbsEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Mouse',member='Abs'";
	}*/
	/*impl HasMatchRule for RelEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Mouse',member='Rel'";
	}*/
	/*impl HasMatchRule for ButtonEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Mouse',member='Button'";
	}*/
	/*impl HasRegistryEventString for AbsEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Mouse:Abs";
	}*/
	/*impl HasRegistryEventString for RelEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Mouse:Rel";
	}*/
	/*impl HasRegistryEventString for ButtonEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Mouse:Button";
	}*/
	impl HasRegistryEventString for MouseEvents {
		const REGISTRY_EVENT_STRING: &'static str = "Mouse:";
	}
}

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod keyboard {
	use crate::{
		error::AtspiError,
		events::{Accessible, EventBodyOwned, GenericEvent, HasMatchRule, HasRegistryEventString},
		Event,
	};
	use zbus;
	use zbus::names::UniqueName;
	use zbus::zvariant::ObjectPath;

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that this example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// use atspi::identify::keyboard::ModifiersEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ModifiersEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let output = std::process::Command::new("busctl")
	/// #       .arg("--user")
	/// #       .arg("call")
	/// #       .arg("org.a11y.Bus")
	/// #       .arg("/org/a11y/bus")
	/// #       .arg("org.a11y.Bus")
	/// #       .arg("GetAddress")
	/// #       .output()
	/// #       .unwrap();
	/// #    let addr_string = String::from_utf8(output.stdout).unwrap();
	/// #    let addr_str = addr_string
	/// #        .strip_prefix("s \"")
	/// #        .unwrap()
	/// #        .trim()
	/// #        .strip_suffix('"')
	/// #        .unwrap();
	/// #   let mut base_cmd = std::process::Command::new("busctl");
	/// #   let thing = base_cmd
	/// #       .arg("--address")
	/// #       .arg(addr_str)
	/// #       .arg("emit")
	/// #       .arg("/org/a11y/atspi/accessible/null")
	/// #       .arg("org.a11y.atspi.Event.Keyboard")
	/// #       .arg("Modifiers")
	/// #       .arg("siiva{sv}")
	/// #       .arg("")
	/// #       .arg("0")
	/// #       .arg("0")
	/// #       .arg("i")
	/// #       .arg("0")
	/// #       .arg("0")
	/// #       .output()
	/// #       .unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///          if let Event::Keyboard(_event) = ev {
	/// #            break;
	///              // do things with your event here
	///          }
	/// #        else { panic!("Something went wrong receiving the event. Usually this means the wrong event was received.") };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
	pub enum KeyboardEvents {
		Modifiers(ModifiersEvent),
	}
	impl_event_conversions!(KeyboardEvents, Event::Keyboard);
	event_wrapper_test_cases!(KeyboardEvents, ModifiersEvent);

	impl HasMatchRule for KeyboardEvents {
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Keyboard'";
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::keyboard::ModifiersEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ModifiersEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = ModifiersEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = ModifiersEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct ModifiersEvent {
		pub item: crate::events::Accessible,
		pub previous_modifiers: i32,
		pub current_modifiers: i32,
	}

	impl GenericEvent<'_> for ModifiersEvent {
		const DBUS_MEMBER: &'static str = "Modifiers";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Keyboard";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Keyboard',member='Modifiers'";
		const REGISTRY_EVENT_STRING: &'static str = "Keyboard:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item, previous_modifiers: body.detail1, current_modifiers: body.detail2 })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for ModifiersEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Keyboard(KeyboardEvents::Modifiers(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl TryFrom<&zbus::Message> for KeyboardEvents {
		type Error = AtspiError;
		fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
			let member = ev
				.member()
				.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
			match member.as_str() {
				"Modifiers" => Ok(KeyboardEvents::Modifiers(ev.try_into()?)),
				_ => Err(AtspiError::MemberMatch("No matching member for Keyboard".into())),
			}
		}
	}

	impl_event_conversions!(
		ModifiersEvent,
		KeyboardEvents,
		KeyboardEvents::Modifiers,
		Event::Keyboard
	);
	event_test_cases!(ModifiersEvent);
	impl_to_dbus_message!(ModifiersEvent);
	impl_from_dbus_message!(ModifiersEvent);
	impl From<ModifiersEvent> for EventBodyOwned {
		fn from(event: ModifiersEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: event.previous_modifiers,
				detail2: event.current_modifiers,
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	/*impl HasMatchRule for ModifiersEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Keyboard',member='Modifiers'";
	}*/
	/*impl HasRegistryEventString for ModifiersEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Keyboard:Modifiers";
	}*/
	impl HasRegistryEventString for KeyboardEvents {
		const REGISTRY_EVENT_STRING: &'static str = "Keyboard:";
	}
}

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod terminal {
	use crate::{
		error::AtspiError,
		events::{Accessible, EventBodyOwned, GenericEvent, HasMatchRule, HasRegistryEventString},
		Event,
	};
	use zbus;
	use zbus::names::UniqueName;
	use zbus::zvariant::ObjectPath;

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that this example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// use atspi::identify::terminal::LineChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<LineChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let output = std::process::Command::new("busctl")
	/// #       .arg("--user")
	/// #       .arg("call")
	/// #       .arg("org.a11y.Bus")
	/// #       .arg("/org/a11y/bus")
	/// #       .arg("org.a11y.Bus")
	/// #       .arg("GetAddress")
	/// #       .output()
	/// #       .unwrap();
	/// #    let addr_string = String::from_utf8(output.stdout).unwrap();
	/// #    let addr_str = addr_string
	/// #        .strip_prefix("s \"")
	/// #        .unwrap()
	/// #        .trim()
	/// #        .strip_suffix('"')
	/// #        .unwrap();
	/// #   let mut base_cmd = std::process::Command::new("busctl");
	/// #   let thing = base_cmd
	/// #       .arg("--address")
	/// #       .arg(addr_str)
	/// #       .arg("emit")
	/// #       .arg("/org/a11y/atspi/accessible/null")
	/// #       .arg("org.a11y.atspi.Event.Terminal")
	/// #       .arg("LineChanged")
	/// #       .arg("siiva{sv}")
	/// #       .arg("")
	/// #       .arg("0")
	/// #       .arg("0")
	/// #       .arg("i")
	/// #       .arg("0")
	/// #       .arg("0")
	/// #       .output()
	/// #       .unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///          if let Event::Terminal(_event) = ev {
	/// #            break;
	///              // do things with your event here
	///          }
	/// #        else { panic!("Something went wrong receiving the event. Usually this means the wrong event was received.") };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
	pub enum TerminalEvents {
		LineChanged(LineChangedEvent),
		ColumnCountChanged(ColumnCountChangedEvent),
		LineCountChanged(LineCountChangedEvent),
		ApplicationChanged(ApplicationChangedEvent),
		CharWidthChanged(CharWidthChangedEvent),
	}
	impl_event_conversions!(TerminalEvents, Event::Terminal);
	event_wrapper_test_cases!(TerminalEvents, LineChangedEvent);

	impl HasMatchRule for TerminalEvents {
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Terminal'";
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::terminal::LineChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<LineChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = LineChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = LineChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct LineChangedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::terminal::ColumnCountChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ColumnCountChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = ColumnCountChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = ColumnCountChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct ColumnCountChangedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::terminal::LineCountChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<LineCountChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = LineCountChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = LineCountChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct LineCountChangedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::terminal::ApplicationChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ApplicationChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = ApplicationChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = ApplicationChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct ApplicationChangedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::terminal::CharWidthChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<CharWidthChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = CharWidthChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = CharWidthChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct CharWidthChangedEvent {
		pub item: crate::events::Accessible,
	}

	impl GenericEvent<'_> for LineChangedEvent {
		const DBUS_MEMBER: &'static str = "LineChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Terminal',member='LineChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for LineChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Terminal(TerminalEvents::LineChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for ColumnCountChangedEvent {
		const DBUS_MEMBER: &'static str = "ColumncountChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Terminal',member='ColumncountChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for ColumnCountChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Terminal(TerminalEvents::ColumnCountChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for LineCountChangedEvent {
		const DBUS_MEMBER: &'static str = "LinecountChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Terminal',member='LinecountChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for LineCountChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Terminal(TerminalEvents::LineCountChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for ApplicationChangedEvent {
		const DBUS_MEMBER: &'static str = "ApplicationChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Terminal',member='ApplicationChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for ApplicationChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Terminal(TerminalEvents::ApplicationChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for CharWidthChangedEvent {
		const DBUS_MEMBER: &'static str = "CharwidthChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Terminal',member='CharwidthChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for CharWidthChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Terminal(TerminalEvents::CharWidthChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl TryFrom<&zbus::Message> for TerminalEvents {
		type Error = AtspiError;
		fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
			let member = ev
				.member()
				.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
			match member.as_str() {
				"LineChanged" => Ok(TerminalEvents::LineChanged(ev.try_into()?)),
				"ColumncountChanged" => Ok(TerminalEvents::ColumnCountChanged(ev.try_into()?)),
				"LinecountChanged" => Ok(TerminalEvents::LineCountChanged(ev.try_into()?)),
				"ApplicationChanged" => Ok(TerminalEvents::ApplicationChanged(ev.try_into()?)),
				"CharwidthChanged" => Ok(TerminalEvents::CharWidthChanged(ev.try_into()?)),
				_ => Err(AtspiError::MemberMatch("No matching member for Terminal".into())),
			}
		}
	}

	impl_event_conversions!(
		LineChangedEvent,
		TerminalEvents,
		TerminalEvents::LineChanged,
		Event::Terminal
	);
	event_test_cases!(LineChangedEvent);
	impl_to_dbus_message!(LineChangedEvent);
	impl_from_dbus_message!(LineChangedEvent);
	impl From<LineChangedEvent> for EventBodyOwned {
		fn from(_event: LineChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		ColumnCountChangedEvent,
		TerminalEvents,
		TerminalEvents::ColumnCountChanged,
		Event::Terminal
	);
	event_test_cases!(ColumnCountChangedEvent);
	impl_to_dbus_message!(ColumnCountChangedEvent);
	impl_from_dbus_message!(ColumnCountChangedEvent);
	impl From<ColumnCountChangedEvent> for EventBodyOwned {
		fn from(_event: ColumnCountChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		LineCountChangedEvent,
		TerminalEvents,
		TerminalEvents::LineCountChanged,
		Event::Terminal
	);
	event_test_cases!(LineCountChangedEvent);
	impl_to_dbus_message!(LineCountChangedEvent);
	impl_from_dbus_message!(LineCountChangedEvent);
	impl From<LineCountChangedEvent> for EventBodyOwned {
		fn from(_event: LineCountChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		ApplicationChangedEvent,
		TerminalEvents,
		TerminalEvents::ApplicationChanged,
		Event::Terminal
	);
	event_test_cases!(ApplicationChangedEvent);
	impl_to_dbus_message!(ApplicationChangedEvent);
	impl_from_dbus_message!(ApplicationChangedEvent);
	impl From<ApplicationChangedEvent> for EventBodyOwned {
		fn from(_event: ApplicationChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		CharWidthChangedEvent,
		TerminalEvents,
		TerminalEvents::CharWidthChanged,
		Event::Terminal
	);
	event_test_cases!(CharWidthChangedEvent);
	impl_to_dbus_message!(CharWidthChangedEvent);
	impl_from_dbus_message!(CharWidthChangedEvent);
	impl From<CharWidthChangedEvent> for EventBodyOwned {
		fn from(_event: CharWidthChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	/*impl HasMatchRule for LineChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='LineChanged'";
	}*/
	/*impl HasMatchRule for ColumnCountChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='ColumncountChanged'";
	}*/
	/*impl HasMatchRule for LineCountChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='LinecountChanged'";
	}*/
	/*impl HasMatchRule for ApplicationChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='ApplicationChanged'";
	}*/
	/*impl HasMatchRule for CharWidthChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='CharwidthChanged'";
	}*/
	/*impl HasRegistryEventString for LineChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:LineChanged";
	}*/
	/*impl HasRegistryEventString for ColumnCountChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:ColumncountChanged";
	}*/
	/*impl HasRegistryEventString for LineCountChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:LinecountChanged";
	}*/
	/*impl HasRegistryEventString for ApplicationChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:ApplicationChanged";
	}*/
	/*impl HasRegistryEventString for CharWidthChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:CharwidthChanged";
	}*/
	impl HasRegistryEventString for TerminalEvents {
		const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
	}
}

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod document {
	use crate::{
		error::AtspiError,
		events::{Accessible, EventBodyOwned, GenericEvent, HasMatchRule, HasRegistryEventString},
		Event,
	};
	use zbus;
	use zbus::names::UniqueName;
	use zbus::zvariant::ObjectPath;

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that this example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// use atspi::identify::document::LoadCompleteEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<LoadCompleteEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let output = std::process::Command::new("busctl")
	/// #       .arg("--user")
	/// #       .arg("call")
	/// #       .arg("org.a11y.Bus")
	/// #       .arg("/org/a11y/bus")
	/// #       .arg("org.a11y.Bus")
	/// #       .arg("GetAddress")
	/// #       .output()
	/// #       .unwrap();
	/// #    let addr_string = String::from_utf8(output.stdout).unwrap();
	/// #    let addr_str = addr_string
	/// #        .strip_prefix("s \"")
	/// #        .unwrap()
	/// #        .trim()
	/// #        .strip_suffix('"')
	/// #        .unwrap();
	/// #   let mut base_cmd = std::process::Command::new("busctl");
	/// #   let thing = base_cmd
	/// #       .arg("--address")
	/// #       .arg(addr_str)
	/// #       .arg("emit")
	/// #       .arg("/org/a11y/atspi/accessible/null")
	/// #       .arg("org.a11y.atspi.Event.Document")
	/// #       .arg("LoadComplete")
	/// #       .arg("siiva{sv}")
	/// #       .arg("")
	/// #       .arg("0")
	/// #       .arg("0")
	/// #       .arg("i")
	/// #       .arg("0")
	/// #       .arg("0")
	/// #       .output()
	/// #       .unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///          if let Event::Document(_event) = ev {
	/// #            break;
	///              // do things with your event here
	///          }
	/// #        else { panic!("Something went wrong receiving the event. Usually this means the wrong event was received.") };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
	pub enum DocumentEvents {
		LoadComplete(LoadCompleteEvent),
		Reload(ReloadEvent),
		LoadStopped(LoadStoppedEvent),
		ContentChanged(ContentChangedEvent),
		AttributesChanged(AttributesChangedEvent),
		PageChanged(PageChangedEvent),
	}
	impl_event_conversions!(DocumentEvents, Event::Document);
	event_wrapper_test_cases!(DocumentEvents, LoadCompleteEvent);

	impl HasMatchRule for DocumentEvents {
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Document'";
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::document::LoadCompleteEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<LoadCompleteEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = LoadCompleteEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = LoadCompleteEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct LoadCompleteEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::document::ReloadEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ReloadEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = ReloadEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = ReloadEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct ReloadEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::document::LoadStoppedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<LoadStoppedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = LoadStoppedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = LoadStoppedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct LoadStoppedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::document::ContentChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<ContentChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = ContentChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = ContentChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct ContentChangedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::document::AttributesChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<AttributesChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = AttributesChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = AttributesChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct AttributesChangedEvent {
		pub item: crate::events::Accessible,
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::document::PageChangedEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<PageChangedEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = PageChangedEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = PageChangedEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct PageChangedEvent {
		pub item: crate::events::Accessible,
	}

	impl GenericEvent<'_> for LoadCompleteEvent {
		const DBUS_MEMBER: &'static str = "LoadComplete";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Document',member='LoadComplete'";
		const REGISTRY_EVENT_STRING: &'static str = "Document:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for LoadCompleteEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Document(DocumentEvents::LoadComplete(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for ReloadEvent {
		const DBUS_MEMBER: &'static str = "Reload";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Document',member='Reload'";
		const REGISTRY_EVENT_STRING: &'static str = "Document:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for ReloadEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Document(DocumentEvents::Reload(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for LoadStoppedEvent {
		const DBUS_MEMBER: &'static str = "LoadStopped";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Document',member='LoadStopped'";
		const REGISTRY_EVENT_STRING: &'static str = "Document:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for LoadStoppedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Document(DocumentEvents::LoadStopped(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for ContentChangedEvent {
		const DBUS_MEMBER: &'static str = "ContentChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Document',member='ContentChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Document:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for ContentChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Document(DocumentEvents::ContentChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for AttributesChangedEvent {
		const DBUS_MEMBER: &'static str = "AttributesChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Document',member='AttributesChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Document:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for AttributesChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Document(DocumentEvents::AttributesChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl GenericEvent<'_> for PageChangedEvent {
		const DBUS_MEMBER: &'static str = "PageChanged";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Document',member='PageChanged'";
		const REGISTRY_EVENT_STRING: &'static str = "Document:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for PageChangedEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Document(DocumentEvents::PageChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl TryFrom<&zbus::Message> for DocumentEvents {
		type Error = AtspiError;
		fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
			let member = ev
				.member()
				.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
			match member.as_str() {
				"LoadComplete" => Ok(DocumentEvents::LoadComplete(ev.try_into()?)),
				"Reload" => Ok(DocumentEvents::Reload(ev.try_into()?)),
				"LoadStopped" => Ok(DocumentEvents::LoadStopped(ev.try_into()?)),
				"ContentChanged" => Ok(DocumentEvents::ContentChanged(ev.try_into()?)),
				"AttributesChanged" => Ok(DocumentEvents::AttributesChanged(ev.try_into()?)),
				"PageChanged" => Ok(DocumentEvents::PageChanged(ev.try_into()?)),
				_ => Err(AtspiError::MemberMatch("No matching member for Document".into())),
			}
		}
	}

	impl_event_conversions!(
		LoadCompleteEvent,
		DocumentEvents,
		DocumentEvents::LoadComplete,
		Event::Document
	);
	event_test_cases!(LoadCompleteEvent);
	impl_to_dbus_message!(LoadCompleteEvent);
	impl_from_dbus_message!(LoadCompleteEvent);
	impl From<LoadCompleteEvent> for EventBodyOwned {
		fn from(_event: LoadCompleteEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(ReloadEvent, DocumentEvents, DocumentEvents::Reload, Event::Document);
	event_test_cases!(ReloadEvent);
	impl_to_dbus_message!(ReloadEvent);
	impl_from_dbus_message!(ReloadEvent);
	impl From<ReloadEvent> for EventBodyOwned {
		fn from(_event: ReloadEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		LoadStoppedEvent,
		DocumentEvents,
		DocumentEvents::LoadStopped,
		Event::Document
	);
	event_test_cases!(LoadStoppedEvent);
	impl_to_dbus_message!(LoadStoppedEvent);
	impl_from_dbus_message!(LoadStoppedEvent);
	impl From<LoadStoppedEvent> for EventBodyOwned {
		fn from(_event: LoadStoppedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		ContentChangedEvent,
		DocumentEvents,
		DocumentEvents::ContentChanged,
		Event::Document
	);
	event_test_cases!(ContentChangedEvent);
	impl_to_dbus_message!(ContentChangedEvent);
	impl_from_dbus_message!(ContentChangedEvent);
	impl From<ContentChangedEvent> for EventBodyOwned {
		fn from(_event: ContentChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		AttributesChangedEvent,
		DocumentEvents,
		DocumentEvents::AttributesChanged,
		Event::Document
	);
	event_test_cases!(AttributesChangedEvent);
	impl_to_dbus_message!(AttributesChangedEvent);
	impl_from_dbus_message!(AttributesChangedEvent);
	impl From<AttributesChangedEvent> for EventBodyOwned {
		fn from(_event: AttributesChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	impl_event_conversions!(
		PageChangedEvent,
		DocumentEvents,
		DocumentEvents::PageChanged,
		Event::Document
	);
	event_test_cases!(PageChangedEvent);
	impl_to_dbus_message!(PageChangedEvent);
	impl_from_dbus_message!(PageChangedEvent);
	impl From<PageChangedEvent> for EventBodyOwned {
		fn from(_event: PageChangedEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	/*impl HasMatchRule for LoadCompleteEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document',member='LoadComplete'";
	}*/
	/*impl HasMatchRule for ReloadEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document',member='Reload'";
	}*/
	/*impl HasMatchRule for LoadStoppedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document',member='LoadStopped'";
	}*/
	/*impl HasMatchRule for ContentChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document',member='ContentChanged'";
	}*/
	/*impl HasMatchRule for AttributesChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document',member='AttributesChanged'";
	}*/
	/*impl HasMatchRule for PageChangedEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document',member='PageChanged'";
	}*/
	/*impl HasRegistryEventString for LoadCompleteEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Document:LoadComplete";
	}*/
	/*impl HasRegistryEventString for ReloadEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Document:Reload";
	}*/
	/*impl HasRegistryEventString for LoadStoppedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Document:LoadStopped";
	}*/
	/*impl HasRegistryEventString for ContentChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Document:ContentChanged";
	}*/
	/*impl HasRegistryEventString for AttributesChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Document:AttributesChanged";
	}*/
	/*impl HasRegistryEventString for PageChangedEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Document:PageChanged";
	}*/
	impl HasRegistryEventString for DocumentEvents {
		const REGISTRY_EVENT_STRING: &'static str = "Document:";
	}
}

#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod focus {
	use crate::{
		error::AtspiError,
		events::{Accessible, EventBodyOwned, GenericEvent, HasMatchRule, HasRegistryEventString},
		Event,
	};
	use zbus;
	use zbus::names::UniqueName;
	use zbus::zvariant::ObjectPath;

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that this example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// use atspi::identify::focus::FocusEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<FocusEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let output = std::process::Command::new("busctl")
	/// #       .arg("--user")
	/// #       .arg("call")
	/// #       .arg("org.a11y.Bus")
	/// #       .arg("/org/a11y/bus")
	/// #       .arg("org.a11y.Bus")
	/// #       .arg("GetAddress")
	/// #       .output()
	/// #       .unwrap();
	/// #    let addr_string = String::from_utf8(output.stdout).unwrap();
	/// #    let addr_str = addr_string
	/// #        .strip_prefix("s \"")
	/// #        .unwrap()
	/// #        .trim()
	/// #        .strip_suffix('"')
	/// #        .unwrap();
	/// #   let mut base_cmd = std::process::Command::new("busctl");
	/// #   let thing = base_cmd
	/// #       .arg("--address")
	/// #       .arg(addr_str)
	/// #       .arg("emit")
	/// #       .arg("/org/a11y/atspi/accessible/null")
	/// #       .arg("org.a11y.atspi.Event.Focus")
	/// #       .arg("Focus")
	/// #       .arg("siiva{sv}")
	/// #       .arg("")
	/// #       .arg("0")
	/// #       .arg("0")
	/// #       .arg("i")
	/// #       .arg("0")
	/// #       .arg("0")
	/// #       .output()
	/// #       .unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///          if let Event::Focus(_event) = ev {
	/// #            break;
	///              // do things with your event here
	///          }
	/// #        else { panic!("Something went wrong receiving the event. Usually this means the wrong event was received.") };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
	pub enum FocusEvents {
		Focus(FocusEvent),
	}
	impl_event_conversions!(FocusEvents, Event::Focus);
	event_wrapper_test_cases!(FocusEvents, FocusEvent);

	impl HasMatchRule for FocusEvents {
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Focus'";
	}

	// IgnoreBlock start
	/// # Example
	///
	/// Even though this example employs `Tokio`, any runtime will do.
	///
	/// Note that the example is minimized for rhe sake of brevity.
	/// More complete examples may be found in the `examples/` directory.
	///
	/// ```
	/// use atspi::Event;
	/// # use atspi::events::GenericEvent;
	/// use atspi::identify::focus::FocusEvent;
	/// # use std::time::Duration;
	/// use tokio_stream::StreamExt;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let atspi = atspi::AccessibilityConnection::open().await.unwrap();
	///     let mut events = atspi.event_stream();
	/// #   atspi.register_event::<FocusEvent>().await.unwrap();
	///     std::pin::pin!(&mut events);
	/// #   let event_struct = FocusEvent::default();
	/// #   atspi.send_event(event_struct.clone()).await.unwrap();
	///
	///     while let Some(Ok(ev)) = events.next().await {
	///         if let Ok(event) = FocusEvent::try_from(ev) {
	/// #          assert_eq!(event.body(), event_struct.body());
	/// #          break;
	///            // do something with the specific event you've received
	///         } else { continue };
	///     }
	/// }
	/// ```
	// IgnoreBlock stop
	#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Eq, Hash, Default)]
	pub struct FocusEvent {
		pub item: crate::events::Accessible,
	}

	impl GenericEvent<'_> for FocusEvent {
		const DBUS_MEMBER: &'static str = "Focus";
		const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Focus";
		const MATCH_RULE_STRING: &'static str =
			"type='signal',interface='org.a11y.atspi.Event.Focus',member='Focus'";
		const REGISTRY_EVENT_STRING: &'static str = "Focus:";

		type Body = EventBodyOwned;

		fn build(item: Accessible, _body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self { item })
		}
		fn sender(&self) -> UniqueName<'_> {
			self.item.name.clone().into()
		}
		fn path<'a>(&self) -> ObjectPath<'_> {
			self.item.path.clone().into()
		}
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}

	/*
	impl TryFrom<Event> for FocusEvent {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
	   if let Event::Focus(FocusEvents::Focus(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}*/

	impl TryFrom<&zbus::Message> for FocusEvents {
		type Error = AtspiError;
		fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
			let member = ev
				.member()
				.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
			match member.as_str() {
				"Focus" => Ok(FocusEvents::Focus(ev.try_into()?)),
				_ => Err(AtspiError::MemberMatch("No matching member for Focus".into())),
			}
		}
	}

	impl_event_conversions!(FocusEvent, FocusEvents, FocusEvents::Focus, Event::Focus);
	event_test_cases!(FocusEvent);
	impl_to_dbus_message!(FocusEvent);
	impl_from_dbus_message!(FocusEvent);
	impl From<FocusEvent> for EventBodyOwned {
		fn from(_event: FocusEvent) -> Self {
			EventBodyOwned {
				properties: std::collections::HashMap::new(),
				kind: String::default(),
				detail1: i32::default(),
				detail2: i32::default(),
				any_data: zbus::zvariant::Value::U8(0).into(),
			}
		}
	}

	/*impl HasMatchRule for FocusEvent {
	  const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Focus',member='Focus'";
	}*/
	/*impl HasRegistryEventString for FocusEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Focus:Focus";
	}*/
	impl HasRegistryEventString for FocusEvents {
		const REGISTRY_EVENT_STRING: &'static str = "Focus:";
	}
}
