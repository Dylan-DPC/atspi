
    use crate::error::AtspiError;
    

    
#[allow(clippy::module_name_repetitions)]
// IgnoreBlock start
// this is to stop clippy from complaining about the copying of module names in the types; since this is more organizational than logical, we're ok leaving it in
// IgnoreBlock stop
pub mod object {
	use crate::{
		error::AtspiError,
		events::*,
		events::{
			object::*,
		},
		traits::{GenericEvent, HasMatchRule, HasRegistryEventString, EventBodyOwned, EventBody},
	};
	use zvariant::ObjectPath;
	/*
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
	#[repr(C)]
	#[derive(Clone, Debug)]
	pub enum ObjectEvents {
		PropertyChange(PropertyChangeEvent<'a>),		BoundsChanged(BoundsChangedEvent<'a>),		LinkSelected(LinkSelectedEvent<'a>),		StateChanged(StateChangedEvent<'a>),		ChildrenChanged(ChildrenChangedEvent<'a>),		VisibleDataChanged(VisibleDataChangedEvent<'a>),		SelectionChanged(SelectionChangedEvent<'a>),		ModelChanged(ModelChangedEvent<'a>),		ActiveDescendantChanged(ActiveDescendantChangedEvent<'a>),		Announcement(AnnouncementEvent<'a>),		AttributesChanged(AttributesChangedEvent<'a>),		RowInserted(RowInsertedEvent<'a>),		RowReordered(RowReorderedEvent<'a>),		RowDeleted(RowDeletedEvent<'a>),		ColumnInserted(ColumnInsertedEvent<'a>),		ColumnReordered(ColumnReorderedEvent<'a>),		ColumnDeleted(ColumnDeletedEvent<'a>),		TextBoundsChanged(TextBoundsChangedEvent<'a>),		TextSelectionChanged(TextSelectionChangedEvent<'a>),		TextChanged(TextChangedEvent<'a>),		TextAttributesChanged(TextAttributesChangedEvent<'a>),		TextCaretMoved(TextCaretMovedEvent<'a>),
	}
	*/
		impl HasMatchRule for ObjectEvents<'_> {
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object'";
	}
	/*
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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = PropertyChangeEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct PropertyChangeEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,
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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = BoundsChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct BoundsChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = LinkSelectedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct LinkSelectedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = StateChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct StateChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,
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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = ChildrenChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct ChildrenChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,
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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = VisibleDataChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct VisibleDataChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = SelectionChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct SelectionChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = ModelChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct ModelChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = ActiveDescendantChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct ActiveDescendantChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,
   pub child: String,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = AnnouncementEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct AnnouncementEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,
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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = AttributesChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct AttributesChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = RowInsertedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct RowInsertedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = RowReorderedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct RowReorderedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = RowDeletedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct RowDeletedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = ColumnInsertedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct ColumnInsertedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = ColumnReorderedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct ColumnReorderedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = ColumnDeletedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct ColumnDeletedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = TextBoundsChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct TextBoundsChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = TextSelectionChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct TextSelectionChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = TextChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct TextChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,
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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = TextAttributesChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct TextAttributesChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = TextCaretMovedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct TextCaretMovedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,
   pub position: i32,

}
	*/
	
    	impl GenericEvent<'_> for PropertyChangeEvent<'_> {
      const DBUS_MEMBER: &'static str = "PropertyChange";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='PropertyChange'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
				property: body.kind, value: body.any_data.try_into()?	
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for PropertyChangeEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::PropertyChange(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for BoundsChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "BoundsChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='BoundsChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for BoundsChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::BoundsChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for LinkSelectedEvent<'_> {
      const DBUS_MEMBER: &'static str = "LinkSelected";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='LinkSelected'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for LinkSelectedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::LinkSelected(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for StateChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "StateChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='StateChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
				state: body.kind, enabled: body.detail1	
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for StateChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::StateChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for ChildrenChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "ChildrenChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ChildrenChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
				operation: body.kind, index_in_parent: body.detail1, child: body.any_data.try_into()?	
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for ChildrenChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::ChildrenChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for VisibleDataChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "VisibleDataChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='VisibleDataChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for VisibleDataChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::VisibleDataChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for SelectionChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "SelectionChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='SelectionChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for SelectionChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::SelectionChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for ModelChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "ModelChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ModelChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for ModelChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::ModelChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for ActiveDescendantChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "ActiveDescendantChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ActiveDescendantChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
				child: body.any_data.try_into()?	
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for ActiveDescendantChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::ActiveDescendantChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for AnnouncementEvent<'_> {
      const DBUS_MEMBER: &'static str = "Announcement";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='Announcement'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
				text: body.kind	
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for AnnouncementEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::Announcement(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for AttributesChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "AttributesChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='AttributesChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for AttributesChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::AttributesChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for RowInsertedEvent<'_> {
      const DBUS_MEMBER: &'static str = "RowInserted";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='RowInserted'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for RowInsertedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::RowInserted(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for RowReorderedEvent<'_> {
      const DBUS_MEMBER: &'static str = "RowReordered";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='RowReordered'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for RowReorderedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::RowReordered(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for RowDeletedEvent<'_> {
      const DBUS_MEMBER: &'static str = "RowDeleted";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='RowDeleted'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for RowDeletedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::RowDeleted(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for ColumnInsertedEvent<'_> {
      const DBUS_MEMBER: &'static str = "ColumnInserted";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnInserted'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for ColumnInsertedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::ColumnInserted(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for ColumnReorderedEvent<'_> {
      const DBUS_MEMBER: &'static str = "ColumnReordered";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnReordered'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for ColumnReorderedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::ColumnReordered(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for ColumnDeletedEvent<'_> {
      const DBUS_MEMBER: &'static str = "ColumnDeleted";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='ColumnDeleted'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for ColumnDeletedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::ColumnDeleted(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for TextBoundsChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "TextBoundsChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='TextBoundsChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for TextBoundsChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::TextBoundsChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for TextSelectionChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "TextSelectionChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='TextSelectionChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for TextSelectionChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::TextSelectionChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for TextChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "TextChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='TextChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
				detail: body.kind, start_pos: body.detail1, length: body.detail2, text: body.any_data.try_into()?	
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for TextChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::TextChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for TextAttributesChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "TextAttributesChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='TextAttributesChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for TextAttributesChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::TextAttributesChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for TextCaretMovedEvent<'_> {
      const DBUS_MEMBER: &'static str = "TextCaretMoved";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Object";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Object',member='TextCaretMoved'";
      const REGISTRY_EVENT_STRING: &'static str = "Object:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
				position: body.detail1	
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for TextCaretMovedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Object(ObjectEvents::TextCaretMoved(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    
	
	
	impl From<ObjectEvents<'_>> for Event<'_> {
		fn from(event_enum: ObjectEvents) -> Self {
        Event::Object(event_enum)
		}
	}
	#[cfg(feature = "zbus")]
	impl TryFrom<&zbus::Message> for ObjectEvents<'_> {
		type Error = AtspiError;
		fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
			let member = ev.member()
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
				"ActiveDescendantChanged" => Ok(ObjectEvents::ActiveDescendantChanged(ev.try_into()?)),
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
	
  
	impl From<PropertyChangeEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: PropertyChangeEvent) -> Self {
			ObjectEvents::PropertyChange(specific_event)
		}
	}
	impl From<PropertyChangeEvent<'_>> for Event<'_> {
		fn from(specific_event: PropertyChangeEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(PropertyChangeEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(PropertyChangeEvent);
	impl From<PropertyChangeEvent<'_>> for EventBodyOwned {
		fn from(event: PropertyChangeEvent) -> Self {
			EventBodyOwned {
	properties: std::collections::HashMap::new(),
	kind: event.property, detail1: i32::default(), detail2: i32::default(), any_data: zvariant::Value::from(event.value).into()
}
		}
	}
	

	impl From<BoundsChangedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: BoundsChangedEvent) -> Self {
			ObjectEvents::BoundsChanged(specific_event)
		}
	}
	impl From<BoundsChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: BoundsChangedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(BoundsChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(BoundsChangedEvent);
	impl From<BoundsChangedEvent<'_>> for EventBodyOwned {
		fn from(_: BoundsChangedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<LinkSelectedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: LinkSelectedEvent) -> Self {
			ObjectEvents::LinkSelected(specific_event)
		}
	}
	impl From<LinkSelectedEvent<'_>> for Event<'_> {
		fn from(specific_event: LinkSelectedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(LinkSelectedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(LinkSelectedEvent);
	impl From<LinkSelectedEvent<'_>> for EventBodyOwned {
		fn from(_: LinkSelectedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<StateChangedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: StateChangedEvent) -> Self {
			ObjectEvents::StateChanged(specific_event)
		}
	}
	impl From<StateChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: StateChangedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(StateChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(StateChangedEvent);
	impl From<StateChangedEvent<'_>> for EventBodyOwned {
		fn from(event: StateChangedEvent) -> Self {
			EventBodyOwned {
	properties: std::collections::HashMap::new(),
	kind: event.state, detail1: event.enabled, detail2: i32::default(), any_data: zvariant::OwnedValue::default()
}
		}
	}
	

	impl From<ChildrenChangedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: ChildrenChangedEvent) -> Self {
			ObjectEvents::ChildrenChanged(specific_event)
		}
	}
	impl From<ChildrenChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: ChildrenChangedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(ChildrenChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(ChildrenChangedEvent);
	impl From<ChildrenChangedEvent<'_>> for EventBodyOwned {
		fn from(event: ChildrenChangedEvent) -> Self {
			EventBodyOwned {
	properties: std::collections::HashMap::new(),
	kind: event.operation, detail1: event.index_in_parent, detail2: i32::default(), any_data: zvariant::Value::from(event.child).into()
}
		}
	}
	

	impl From<VisibleDataChangedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: VisibleDataChangedEvent) -> Self {
			ObjectEvents::VisibleDataChanged(specific_event)
		}
	}
	impl From<VisibleDataChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: VisibleDataChangedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(VisibleDataChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(VisibleDataChangedEvent);
	impl From<VisibleDataChangedEvent<'_>> for EventBodyOwned {
		fn from(_: VisibleDataChangedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<SelectionChangedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: SelectionChangedEvent) -> Self {
			ObjectEvents::SelectionChanged(specific_event)
		}
	}
	impl From<SelectionChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: SelectionChangedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(SelectionChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(SelectionChangedEvent);
	impl From<SelectionChangedEvent<'_>> for EventBodyOwned {
		fn from(_: SelectionChangedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<ModelChangedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: ModelChangedEvent) -> Self {
			ObjectEvents::ModelChanged(specific_event)
		}
	}
	impl From<ModelChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: ModelChangedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(ModelChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(ModelChangedEvent);
	impl From<ModelChangedEvent<'_>> for EventBodyOwned {
		fn from(_: ModelChangedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<ActiveDescendantChangedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: ActiveDescendantChangedEvent) -> Self {
			ObjectEvents::ActiveDescendantChanged(specific_event)
		}
	}
	impl From<ActiveDescendantChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: ActiveDescendantChangedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(ActiveDescendantChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(ActiveDescendantChangedEvent);
	impl From<ActiveDescendantChangedEvent<'_>> for EventBodyOwned {
		fn from(event: ActiveDescendantChangedEvent) -> Self {
			EventBodyOwned {
	properties: std::collections::HashMap::new(),
	kind: String::default(), detail1: i32::default(), detail2: i32::default(), any_data: zvariant::Value::from(event.child).into()
}
		}
	}
	

	impl From<AnnouncementEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: AnnouncementEvent) -> Self {
			ObjectEvents::Announcement(specific_event)
		}
	}
	impl From<AnnouncementEvent<'_>> for Event<'_> {
		fn from(specific_event: AnnouncementEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(AnnouncementEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(AnnouncementEvent);
	impl From<AnnouncementEvent<'_>> for EventBodyOwned {
		fn from(event: AnnouncementEvent) -> Self {
			EventBodyOwned {
	properties: std::collections::HashMap::new(),
	kind: event.text, detail1: i32::default(), detail2: i32::default(), any_data: zvariant::OwnedValue::default()
}
		}
	}
	

	impl From<AttributesChangedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: AttributesChangedEvent) -> Self {
			ObjectEvents::AttributesChanged(specific_event)
		}
	}
	impl From<AttributesChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: AttributesChangedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(AttributesChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(AttributesChangedEvent);
	impl From<AttributesChangedEvent<'_>> for EventBodyOwned {
		fn from(_: AttributesChangedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<RowInsertedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: RowInsertedEvent) -> Self {
			ObjectEvents::RowInserted(specific_event)
		}
	}
	impl From<RowInsertedEvent<'_>> for Event<'_> {
		fn from(specific_event: RowInsertedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(RowInsertedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(RowInsertedEvent);
	impl From<RowInsertedEvent<'_>> for EventBodyOwned {
		fn from(_: RowInsertedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<RowReorderedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: RowReorderedEvent) -> Self {
			ObjectEvents::RowReordered(specific_event)
		}
	}
	impl From<RowReorderedEvent<'_>> for Event<'_> {
		fn from(specific_event: RowReorderedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(RowReorderedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(RowReorderedEvent);
	impl From<RowReorderedEvent<'_>> for EventBodyOwned {
		fn from(_: RowReorderedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<RowDeletedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: RowDeletedEvent) -> Self {
			ObjectEvents::RowDeleted(specific_event)
		}
	}
	impl From<RowDeletedEvent<'_>> for Event<'_> {
		fn from(specific_event: RowDeletedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(RowDeletedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(RowDeletedEvent);
	impl From<RowDeletedEvent<'_>> for EventBodyOwned {
		fn from(_: RowDeletedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<ColumnInsertedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: ColumnInsertedEvent) -> Self {
			ObjectEvents::ColumnInserted(specific_event)
		}
	}
	impl From<ColumnInsertedEvent<'_>> for Event<'_> {
		fn from(specific_event: ColumnInsertedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(ColumnInsertedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(ColumnInsertedEvent);
	impl From<ColumnInsertedEvent<'_>> for EventBodyOwned {
		fn from(_: ColumnInsertedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<ColumnReorderedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: ColumnReorderedEvent) -> Self {
			ObjectEvents::ColumnReordered(specific_event)
		}
	}
	impl From<ColumnReorderedEvent<'_>> for Event<'_> {
		fn from(specific_event: ColumnReorderedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(ColumnReorderedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(ColumnReorderedEvent);
	impl From<ColumnReorderedEvent<'_>> for EventBodyOwned {
		fn from(_: ColumnReorderedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<ColumnDeletedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: ColumnDeletedEvent) -> Self {
			ObjectEvents::ColumnDeleted(specific_event)
		}
	}
	impl From<ColumnDeletedEvent<'_>> for Event<'_> {
		fn from(specific_event: ColumnDeletedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(ColumnDeletedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(ColumnDeletedEvent);
	impl From<ColumnDeletedEvent<'_>> for EventBodyOwned {
		fn from(_: ColumnDeletedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<TextBoundsChangedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: TextBoundsChangedEvent) -> Self {
			ObjectEvents::TextBoundsChanged(specific_event)
		}
	}
	impl From<TextBoundsChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: TextBoundsChangedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(TextBoundsChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(TextBoundsChangedEvent);
	impl From<TextBoundsChangedEvent<'_>> for EventBodyOwned {
		fn from(_: TextBoundsChangedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<TextSelectionChangedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: TextSelectionChangedEvent) -> Self {
			ObjectEvents::TextSelectionChanged(specific_event)
		}
	}
	impl From<TextSelectionChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: TextSelectionChangedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(TextSelectionChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(TextSelectionChangedEvent);
	impl From<TextSelectionChangedEvent<'_>> for EventBodyOwned {
		fn from(_: TextSelectionChangedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<TextChangedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: TextChangedEvent) -> Self {
			ObjectEvents::TextChanged(specific_event)
		}
	}
	impl From<TextChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: TextChangedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(TextChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(TextChangedEvent);
	impl From<TextChangedEvent<'_>> for EventBodyOwned {
		fn from(event: TextChangedEvent) -> Self {
			EventBodyOwned {
	properties: std::collections::HashMap::new(),
	kind: event.detail, detail1: event.start_pos, detail2: event.length, any_data: zvariant::Value::from(event.text).into()
}
		}
	}
	

	impl From<TextAttributesChangedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: TextAttributesChangedEvent) -> Self {
			ObjectEvents::TextAttributesChanged(specific_event)
		}
	}
	impl From<TextAttributesChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: TextAttributesChangedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(TextAttributesChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(TextAttributesChangedEvent);
	impl From<TextAttributesChangedEvent<'_>> for EventBodyOwned {
		fn from(_: TextAttributesChangedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<TextCaretMovedEvent<'_>> for ObjectEvents<'_> {
		fn from(specific_event: TextCaretMovedEvent) -> Self {
			ObjectEvents::TextCaretMoved(specific_event)
		}
	}
	impl From<TextCaretMovedEvent<'_>> for Event<'_> {
		fn from(specific_event: TextCaretMovedEvent) -> Self {
			Event::Object(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(TextCaretMovedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(TextCaretMovedEvent);
	impl From<TextCaretMovedEvent<'_>> for EventBodyOwned {
		fn from(event: TextCaretMovedEvent) -> Self {
			EventBodyOwned {
	properties: std::collections::HashMap::new(),
	kind: String::default(), detail1: event.position, detail2: i32::default(), any_data: zvariant::OwnedValue::default()
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
  	impl HasRegistryEventString for ObjectEvents<'_> {
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
		events::*,
		events::{
			window::*,
		},
		traits::{GenericEvent, HasMatchRule, HasRegistryEventString, EventBodyOwned, EventBody},
	};
	use zvariant::ObjectPath;
	/*
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
	#[repr(C)]
	#[derive(Clone, Debug)]
	pub enum WindowEvents {
		PropertyChange(PropertyChangeEvent<'a>),		Minimize(MinimizeEvent<'a>),		Maximize(MaximizeEvent<'a>),		Restore(RestoreEvent<'a>),		Close(CloseEvent<'a>),		Create(CreateEvent<'a>),		Reparent(ReparentEvent<'a>),		DesktopCreate(DesktopCreateEvent<'a>),		DesktopDestroy(DesktopDestroyEvent<'a>),		Destroy(DestroyEvent<'a>),		Activate(ActivateEvent<'a>),		Deactivate(DeactivateEvent<'a>),		Raise(RaiseEvent<'a>),		Lower(LowerEvent<'a>),		Move(MoveEvent<'a>),		Resize(ResizeEvent<'a>),		Shade(ShadeEvent<'a>),		UUshade(UUshadeEvent<'a>),		Restyle(RestyleEvent<'a>),
	}
	*/
		impl HasMatchRule for WindowEvents<'_> {
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window'";
	}
	/*
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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = PropertyChangeEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct PropertyChangeEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,
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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = MinimizeEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct MinimizeEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = MaximizeEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct MaximizeEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = RestoreEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct RestoreEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = CloseEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct CloseEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = CreateEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct CreateEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = ReparentEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct ReparentEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = DesktopCreateEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct DesktopCreateEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = DesktopDestroyEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct DesktopDestroyEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = DestroyEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct DestroyEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = ActivateEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct ActivateEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = DeactivateEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct DeactivateEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = RaiseEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct RaiseEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = LowerEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct LowerEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = MoveEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct MoveEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = ResizeEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct ResizeEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = ShadeEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct ShadeEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = UUshadeEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct UUshadeEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = RestyleEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct RestyleEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

}
	*/
	
    	impl GenericEvent<'_> for PropertyChangeEvent<'_> {
      const DBUS_MEMBER: &'static str = "PropertyChange";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='PropertyChange'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
				property: body.kind	
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for PropertyChangeEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::PropertyChange(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for MinimizeEvent<'_> {
      const DBUS_MEMBER: &'static str = "Minimize";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Minimize'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for MinimizeEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::Minimize(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for MaximizeEvent<'_> {
      const DBUS_MEMBER: &'static str = "Maximize";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Maximize'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for MaximizeEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::Maximize(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for RestoreEvent<'_> {
      const DBUS_MEMBER: &'static str = "Restore";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Restore'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for RestoreEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::Restore(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for CloseEvent<'_> {
      const DBUS_MEMBER: &'static str = "Close";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Close'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for CloseEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::Close(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for CreateEvent<'_> {
      const DBUS_MEMBER: &'static str = "Create";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Create'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for CreateEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::Create(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for ReparentEvent<'_> {
      const DBUS_MEMBER: &'static str = "Reparent";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Reparent'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for ReparentEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::Reparent(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for DesktopCreateEvent<'_> {
      const DBUS_MEMBER: &'static str = "DesktopCreate";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopCreate'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for DesktopCreateEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::DesktopCreate(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for DesktopDestroyEvent<'_> {
      const DBUS_MEMBER: &'static str = "DesktopDestroy";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='DesktopDestroy'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for DesktopDestroyEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::DesktopDestroy(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for DestroyEvent<'_> {
      const DBUS_MEMBER: &'static str = "Destroy";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Destroy'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for DestroyEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::Destroy(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for ActivateEvent<'_> {
      const DBUS_MEMBER: &'static str = "Activate";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Activate'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for ActivateEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::Activate(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for DeactivateEvent<'_> {
      const DBUS_MEMBER: &'static str = "Deactivate";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Deactivate'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for DeactivateEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::Deactivate(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for RaiseEvent<'_> {
      const DBUS_MEMBER: &'static str = "Raise";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Raise'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for RaiseEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::Raise(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for LowerEvent<'_> {
      const DBUS_MEMBER: &'static str = "Lower";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Lower'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for LowerEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::Lower(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for MoveEvent<'_> {
      const DBUS_MEMBER: &'static str = "Move";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Move'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for MoveEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::Move(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for ResizeEvent<'_> {
      const DBUS_MEMBER: &'static str = "Resize";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Resize'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for ResizeEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::Resize(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for ShadeEvent<'_> {
      const DBUS_MEMBER: &'static str = "Shade";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Shade'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for ShadeEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::Shade(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for UUshadeEvent<'_> {
      const DBUS_MEMBER: &'static str = "uUshade";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='uUshade'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for UUshadeEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::UUshade(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for RestyleEvent<'_> {
      const DBUS_MEMBER: &'static str = "Restyle";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Window";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Window',member='Restyle'";
      const REGISTRY_EVENT_STRING: &'static str = "Window:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for RestyleEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Window(WindowEvents::Restyle(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    
	
	
	impl From<WindowEvents<'_>> for Event<'_> {
		fn from(event_enum: WindowEvents) -> Self {
        Event::Window(event_enum)
		}
	}
	#[cfg(feature = "zbus")]
	impl TryFrom<&zbus::Message> for WindowEvents<'_> {
		type Error = AtspiError;
		fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
			let member = ev.member()
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
	
  
	impl From<PropertyChangeEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: PropertyChangeEvent) -> Self {
			WindowEvents::PropertyChange(specific_event)
		}
	}
	impl From<PropertyChangeEvent<'_>> for Event<'_> {
		fn from(specific_event: PropertyChangeEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(PropertyChangeEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(PropertyChangeEvent);
	impl From<PropertyChangeEvent<'_>> for EventBodyOwned {
		fn from(event: PropertyChangeEvent) -> Self {
			EventBodyOwned {
	properties: std::collections::HashMap::new(),
	kind: event.property, detail1: i32::default(), detail2: i32::default(), any_data: zvariant::OwnedValue::default()
}
		}
	}
	

	impl From<MinimizeEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: MinimizeEvent) -> Self {
			WindowEvents::Minimize(specific_event)
		}
	}
	impl From<MinimizeEvent<'_>> for Event<'_> {
		fn from(specific_event: MinimizeEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(MinimizeEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(MinimizeEvent);
	impl From<MinimizeEvent<'_>> for EventBodyOwned {
		fn from(_: MinimizeEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<MaximizeEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: MaximizeEvent) -> Self {
			WindowEvents::Maximize(specific_event)
		}
	}
	impl From<MaximizeEvent<'_>> for Event<'_> {
		fn from(specific_event: MaximizeEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(MaximizeEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(MaximizeEvent);
	impl From<MaximizeEvent<'_>> for EventBodyOwned {
		fn from(_: MaximizeEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<RestoreEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: RestoreEvent) -> Self {
			WindowEvents::Restore(specific_event)
		}
	}
	impl From<RestoreEvent<'_>> for Event<'_> {
		fn from(specific_event: RestoreEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(RestoreEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(RestoreEvent);
	impl From<RestoreEvent<'_>> for EventBodyOwned {
		fn from(_: RestoreEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<CloseEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: CloseEvent) -> Self {
			WindowEvents::Close(specific_event)
		}
	}
	impl From<CloseEvent<'_>> for Event<'_> {
		fn from(specific_event: CloseEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(CloseEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(CloseEvent);
	impl From<CloseEvent<'_>> for EventBodyOwned {
		fn from(_: CloseEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<CreateEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: CreateEvent) -> Self {
			WindowEvents::Create(specific_event)
		}
	}
	impl From<CreateEvent<'_>> for Event<'_> {
		fn from(specific_event: CreateEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(CreateEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(CreateEvent);
	impl From<CreateEvent<'_>> for EventBodyOwned {
		fn from(_: CreateEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<ReparentEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: ReparentEvent) -> Self {
			WindowEvents::Reparent(specific_event)
		}
	}
	impl From<ReparentEvent<'_>> for Event<'_> {
		fn from(specific_event: ReparentEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(ReparentEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(ReparentEvent);
	impl From<ReparentEvent<'_>> for EventBodyOwned {
		fn from(_: ReparentEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<DesktopCreateEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: DesktopCreateEvent) -> Self {
			WindowEvents::DesktopCreate(specific_event)
		}
	}
	impl From<DesktopCreateEvent<'_>> for Event<'_> {
		fn from(specific_event: DesktopCreateEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(DesktopCreateEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(DesktopCreateEvent);
	impl From<DesktopCreateEvent<'_>> for EventBodyOwned {
		fn from(_: DesktopCreateEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<DesktopDestroyEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: DesktopDestroyEvent) -> Self {
			WindowEvents::DesktopDestroy(specific_event)
		}
	}
	impl From<DesktopDestroyEvent<'_>> for Event<'_> {
		fn from(specific_event: DesktopDestroyEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(DesktopDestroyEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(DesktopDestroyEvent);
	impl From<DesktopDestroyEvent<'_>> for EventBodyOwned {
		fn from(_: DesktopDestroyEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<DestroyEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: DestroyEvent) -> Self {
			WindowEvents::Destroy(specific_event)
		}
	}
	impl From<DestroyEvent<'_>> for Event<'_> {
		fn from(specific_event: DestroyEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(DestroyEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(DestroyEvent);
	impl From<DestroyEvent<'_>> for EventBodyOwned {
		fn from(_: DestroyEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<ActivateEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: ActivateEvent) -> Self {
			WindowEvents::Activate(specific_event)
		}
	}
	impl From<ActivateEvent<'_>> for Event<'_> {
		fn from(specific_event: ActivateEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(ActivateEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(ActivateEvent);
	impl From<ActivateEvent<'_>> for EventBodyOwned {
		fn from(_: ActivateEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<DeactivateEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: DeactivateEvent) -> Self {
			WindowEvents::Deactivate(specific_event)
		}
	}
	impl From<DeactivateEvent<'_>> for Event<'_> {
		fn from(specific_event: DeactivateEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(DeactivateEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(DeactivateEvent);
	impl From<DeactivateEvent<'_>> for EventBodyOwned {
		fn from(_: DeactivateEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<RaiseEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: RaiseEvent) -> Self {
			WindowEvents::Raise(specific_event)
		}
	}
	impl From<RaiseEvent<'_>> for Event<'_> {
		fn from(specific_event: RaiseEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(RaiseEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(RaiseEvent);
	impl From<RaiseEvent<'_>> for EventBodyOwned {
		fn from(_: RaiseEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<LowerEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: LowerEvent) -> Self {
			WindowEvents::Lower(specific_event)
		}
	}
	impl From<LowerEvent<'_>> for Event<'_> {
		fn from(specific_event: LowerEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(LowerEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(LowerEvent);
	impl From<LowerEvent<'_>> for EventBodyOwned {
		fn from(_: LowerEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<MoveEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: MoveEvent) -> Self {
			WindowEvents::Move(specific_event)
		}
	}
	impl From<MoveEvent<'_>> for Event<'_> {
		fn from(specific_event: MoveEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(MoveEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(MoveEvent);
	impl From<MoveEvent<'_>> for EventBodyOwned {
		fn from(_: MoveEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<ResizeEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: ResizeEvent) -> Self {
			WindowEvents::Resize(specific_event)
		}
	}
	impl From<ResizeEvent<'_>> for Event<'_> {
		fn from(specific_event: ResizeEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(ResizeEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(ResizeEvent);
	impl From<ResizeEvent<'_>> for EventBodyOwned {
		fn from(_: ResizeEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<ShadeEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: ShadeEvent) -> Self {
			WindowEvents::Shade(specific_event)
		}
	}
	impl From<ShadeEvent<'_>> for Event<'_> {
		fn from(specific_event: ShadeEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(ShadeEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(ShadeEvent);
	impl From<ShadeEvent<'_>> for EventBodyOwned {
		fn from(_: ShadeEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<UUshadeEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: UUshadeEvent) -> Self {
			WindowEvents::UUshade(specific_event)
		}
	}
	impl From<UUshadeEvent<'_>> for Event<'_> {
		fn from(specific_event: UUshadeEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(UUshadeEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(UUshadeEvent);
	impl From<UUshadeEvent<'_>> for EventBodyOwned {
		fn from(_: UUshadeEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<RestyleEvent<'_>> for WindowEvents<'_> {
		fn from(specific_event: RestyleEvent) -> Self {
			WindowEvents::Restyle(specific_event)
		}
	}
	impl From<RestyleEvent<'_>> for Event<'_> {
		fn from(specific_event: RestyleEvent) -> Self {
			Event::Window(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(RestyleEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(RestyleEvent);
	impl From<RestyleEvent<'_>> for EventBodyOwned {
		fn from(_: RestyleEvent) -> Self {
			EventBodyOwned::default()
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
  	impl HasRegistryEventString for WindowEvents<'_> {
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
		events::*,
		events::{
			mouse::*,
		},
		traits::{GenericEvent, HasMatchRule, HasRegistryEventString, EventBodyOwned, EventBody},
	};
	use zvariant::ObjectPath;
	/*
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
	#[repr(C)]
	#[derive(Clone, Debug)]
	pub enum MouseEvents {
		Abs(AbsEvent<'a>),		Rel(RelEvent<'a>),		Button(ButtonEvent<'a>),
	}
	*/
		impl HasMatchRule for MouseEvents<'_> {
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Mouse'";
	}
	/*
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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = AbsEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct AbsEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,
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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = RelEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct RelEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,
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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = ButtonEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct ButtonEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,
   pub detail: String,
   pub mouse_x: i32,
   pub mouse_y: i32,

}
	*/
	
    	impl GenericEvent<'_> for AbsEvent<'_> {
      const DBUS_MEMBER: &'static str = "Abs";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Mouse',member='Abs'";
      const REGISTRY_EVENT_STRING: &'static str = "Mouse:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
				x: body.detail1, y: body.detail2	
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for AbsEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Mouse(MouseEvents::Abs(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for RelEvent<'_> {
      const DBUS_MEMBER: &'static str = "Rel";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Mouse',member='Rel'";
      const REGISTRY_EVENT_STRING: &'static str = "Mouse:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
				x: body.detail1, y: body.detail2	
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for RelEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Mouse(MouseEvents::Rel(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for ButtonEvent<'_> {
      const DBUS_MEMBER: &'static str = "Button";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Mouse";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Mouse',member='Button'";
      const REGISTRY_EVENT_STRING: &'static str = "Mouse:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
				detail: body.kind, mouse_x: body.detail1, mouse_y: body.detail2	
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for ButtonEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Mouse(MouseEvents::Button(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    
	
	
	impl From<MouseEvents<'_>> for Event<'_> {
		fn from(event_enum: MouseEvents) -> Self {
        Event::Mouse(event_enum)
		}
	}
	#[cfg(feature = "zbus")]
	impl TryFrom<&zbus::Message> for MouseEvents<'_> {
		type Error = AtspiError;
		fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
			let member = ev.member()
				.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
			match member.as_str() {
				"Abs" => Ok(MouseEvents::Abs(ev.try_into()?)),
				"Rel" => Ok(MouseEvents::Rel(ev.try_into()?)),
				"Button" => Ok(MouseEvents::Button(ev.try_into()?)),
				_ => Err(AtspiError::MemberMatch("No matching member for Mouse".into())),
			}
		}
	}
	
  
	impl From<AbsEvent<'_>> for MouseEvents<'_> {
		fn from(specific_event: AbsEvent) -> Self {
			MouseEvents::Abs(specific_event)
		}
	}
	impl From<AbsEvent<'_>> for Event<'_> {
		fn from(specific_event: AbsEvent) -> Self {
			Event::Mouse(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(AbsEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(AbsEvent);
	impl From<AbsEvent<'_>> for EventBodyOwned {
		fn from(event: AbsEvent) -> Self {
			EventBodyOwned {
	properties: std::collections::HashMap::new(),
	kind: String::default(), detail1: event.x, detail2: event.y, any_data: zvariant::OwnedValue::default()
}
		}
	}
	

	impl From<RelEvent<'_>> for MouseEvents<'_> {
		fn from(specific_event: RelEvent) -> Self {
			MouseEvents::Rel(specific_event)
		}
	}
	impl From<RelEvent<'_>> for Event<'_> {
		fn from(specific_event: RelEvent) -> Self {
			Event::Mouse(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(RelEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(RelEvent);
	impl From<RelEvent<'_>> for EventBodyOwned {
		fn from(event: RelEvent) -> Self {
			EventBodyOwned {
	properties: std::collections::HashMap::new(),
	kind: String::default(), detail1: event.x, detail2: event.y, any_data: zvariant::OwnedValue::default()
}
		}
	}
	

	impl From<ButtonEvent<'_>> for MouseEvents<'_> {
		fn from(specific_event: ButtonEvent) -> Self {
			MouseEvents::Button(specific_event)
		}
	}
	impl From<ButtonEvent<'_>> for Event<'_> {
		fn from(specific_event: ButtonEvent) -> Self {
			Event::Mouse(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(ButtonEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(ButtonEvent);
	impl From<ButtonEvent<'_>> for EventBodyOwned {
		fn from(event: ButtonEvent) -> Self {
			EventBodyOwned {
	properties: std::collections::HashMap::new(),
	kind: event.detail.to_string(), detail1: event.mouse_x, detail2: event.mouse_y, any_data: zvariant::OwnedValue::default()
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
  	impl HasRegistryEventString for MouseEvents<'_> {
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
		events::*,
		events::{
			keyboard::*,
		},
		traits::{GenericEvent, HasMatchRule, HasRegistryEventString, EventBodyOwned, EventBody},
	};
	use zvariant::ObjectPath;
	/*
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
	#[repr(C)]
	#[derive(Clone, Debug)]
	pub enum KeyboardEvents {
		Modifiers(ModifiersEvent<'a>),
	}
	*/
		impl HasMatchRule for KeyboardEvents<'_> {
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Keyboard'";
	}
	/*
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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = ModifiersEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct ModifiersEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,
   pub previous_modifiers: i32,
   pub current_modifiers: i32,

}
	*/
	
    	impl GenericEvent<'_> for ModifiersEvent<'_> {
      const DBUS_MEMBER: &'static str = "Modifiers";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Keyboard";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Keyboard',member='Modifiers'";
      const REGISTRY_EVENT_STRING: &'static str = "Keyboard:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, body: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
				previous_modifiers: body.detail1, current_modifiers: body.detail2	
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for ModifiersEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Keyboard(KeyboardEvents::Modifiers(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    
	
	
	impl From<KeyboardEvents<'_>> for Event<'_> {
		fn from(event_enum: KeyboardEvents) -> Self {
        Event::Keyboard(event_enum)
		}
	}
	#[cfg(feature = "zbus")]
	impl TryFrom<&zbus::Message> for KeyboardEvents<'_> {
		type Error = AtspiError;
		fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
			let member = ev.member()
				.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
			match member.as_str() {
				"Modifiers" => Ok(KeyboardEvents::Modifiers(ev.try_into()?)),
				_ => Err(AtspiError::MemberMatch("No matching member for Keyboard".into())),
			}
		}
	}
	
  
	impl From<ModifiersEvent<'_>> for KeyboardEvents<'_> {
		fn from(specific_event: ModifiersEvent) -> Self {
			KeyboardEvents::Modifiers(specific_event)
		}
	}
	impl From<ModifiersEvent<'_>> for Event<'_> {
		fn from(specific_event: ModifiersEvent) -> Self {
			Event::Keyboard(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(ModifiersEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(ModifiersEvent);
	impl From<ModifiersEvent<'_>> for EventBodyOwned {
		fn from(event: ModifiersEvent) -> Self {
			EventBodyOwned {
	properties: std::collections::HashMap::new(),
	kind: String::default(), detail1: event.previous_modifiers, detail2: event.current_modifiers, any_data: zvariant::OwnedValue::default()
}
		}
	}
	
		/*impl HasMatchRule for ModifiersEvent {
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Keyboard',member='Modifiers'";
	}*/
  	/*impl HasRegistryEventString for ModifiersEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Keyboard:Modifiers";
	}*/
  	impl HasRegistryEventString for KeyboardEvents<'_> {
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
		events::*,
		events::{
			terminal::*,
		},
		traits::{GenericEvent, HasMatchRule, HasRegistryEventString, EventBodyOwned, EventBody},
	};
	use zvariant::ObjectPath;
	/*
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
	#[repr(C)]
	#[derive(Clone, Debug)]
	pub enum TerminalEvents {
		LineChanged(LineChangedEvent<'a>),		ColumnCountChanged(ColumnCountChangedEvent<'a>),		LineCountChanged(LineCountChangedEvent<'a>),		ApplicationChanged(ApplicationChangedEvent<'a>),		CharWidthChanged(CharWidthChangedEvent<'a>),
	}
	*/
		impl HasMatchRule for TerminalEvents<'_> {
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal'";
	}
	/*
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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = LineChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct LineChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = ColumnCountChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct ColumnCountChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = LineCountChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct LineCountChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = ApplicationChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct ApplicationChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = CharWidthChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct CharWidthChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

}
	*/
	
    	impl GenericEvent<'_> for LineChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "LineChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='LineChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for LineChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Terminal(TerminalEvents::LineChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for ColumnCountChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "ColumncountChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='ColumncountChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for ColumnCountChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Terminal(TerminalEvents::ColumnCountChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for LineCountChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "LinecountChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='LinecountChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for LineCountChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Terminal(TerminalEvents::LineCountChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for ApplicationChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "ApplicationChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='ApplicationChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for ApplicationChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Terminal(TerminalEvents::ApplicationChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for CharWidthChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "CharwidthChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Terminal";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Terminal',member='CharwidthChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Terminal:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for CharWidthChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Terminal(TerminalEvents::CharWidthChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    
	
	
	impl From<TerminalEvents<'_>> for Event<'_> {
		fn from(event_enum: TerminalEvents) -> Self {
        Event::Terminal(event_enum)
		}
	}
	#[cfg(feature = "zbus")]
	impl TryFrom<&zbus::Message> for TerminalEvents<'_> {
		type Error = AtspiError;
		fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
			let member = ev.member()
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
	
  
	impl From<LineChangedEvent<'_>> for TerminalEvents<'_> {
		fn from(specific_event: LineChangedEvent) -> Self {
			TerminalEvents::LineChanged(specific_event)
		}
	}
	impl From<LineChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: LineChangedEvent) -> Self {
			Event::Terminal(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(LineChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(LineChangedEvent);
	impl From<LineChangedEvent<'_>> for EventBodyOwned {
		fn from(_: LineChangedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<ColumnCountChangedEvent<'_>> for TerminalEvents<'_> {
		fn from(specific_event: ColumnCountChangedEvent) -> Self {
			TerminalEvents::ColumnCountChanged(specific_event)
		}
	}
	impl From<ColumnCountChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: ColumnCountChangedEvent) -> Self {
			Event::Terminal(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(ColumnCountChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(ColumnCountChangedEvent);
	impl From<ColumnCountChangedEvent<'_>> for EventBodyOwned {
		fn from(_: ColumnCountChangedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<LineCountChangedEvent<'_>> for TerminalEvents<'_> {
		fn from(specific_event: LineCountChangedEvent) -> Self {
			TerminalEvents::LineCountChanged(specific_event)
		}
	}
	impl From<LineCountChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: LineCountChangedEvent) -> Self {
			Event::Terminal(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(LineCountChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(LineCountChangedEvent);
	impl From<LineCountChangedEvent<'_>> for EventBodyOwned {
		fn from(_: LineCountChangedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<ApplicationChangedEvent<'_>> for TerminalEvents<'_> {
		fn from(specific_event: ApplicationChangedEvent) -> Self {
			TerminalEvents::ApplicationChanged(specific_event)
		}
	}
	impl From<ApplicationChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: ApplicationChangedEvent) -> Self {
			Event::Terminal(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(ApplicationChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(ApplicationChangedEvent);
	impl From<ApplicationChangedEvent<'_>> for EventBodyOwned {
		fn from(_: ApplicationChangedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<CharWidthChangedEvent<'_>> for TerminalEvents<'_> {
		fn from(specific_event: CharWidthChangedEvent) -> Self {
			TerminalEvents::CharWidthChanged(specific_event)
		}
	}
	impl From<CharWidthChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: CharWidthChangedEvent) -> Self {
			Event::Terminal(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(CharWidthChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(CharWidthChangedEvent);
	impl From<CharWidthChangedEvent<'_>> for EventBodyOwned {
		fn from(_: CharWidthChangedEvent) -> Self {
			EventBodyOwned::default()
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
  	impl HasRegistryEventString for TerminalEvents<'_> {
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
		events::*,
		events::{
			document::*,
		},
		traits::{GenericEvent, HasMatchRule, HasRegistryEventString, EventBodyOwned, EventBody},
	};
	use zvariant::ObjectPath;
	/*
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
	#[repr(C)]
	#[derive(Clone, Debug)]
	pub enum DocumentEvents {
		LoadComplete(LoadCompleteEvent<'a>),		Reload(ReloadEvent<'a>),		LoadStopped(LoadStoppedEvent<'a>),		ContentChanged(ContentChangedEvent<'a>),		AttributesChanged(AttributesChangedEvent<'a>),		PageChanged(PageChangedEvent<'a>),
	}
	*/
		impl HasMatchRule for DocumentEvents<'_> {
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document'";
	}
	/*
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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = LoadCompleteEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct LoadCompleteEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = ReloadEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct ReloadEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = LoadStoppedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct LoadStoppedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = ContentChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct ContentChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = AttributesChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct AttributesChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = PageChangedEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct PageChangedEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

}
	*/
	
    	impl GenericEvent<'_> for LoadCompleteEvent<'_> {
      const DBUS_MEMBER: &'static str = "LoadComplete";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document',member='LoadComplete'";
      const REGISTRY_EVENT_STRING: &'static str = "Document:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for LoadCompleteEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Document(DocumentEvents::LoadComplete(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for ReloadEvent<'_> {
      const DBUS_MEMBER: &'static str = "Reload";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document',member='Reload'";
      const REGISTRY_EVENT_STRING: &'static str = "Document:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for ReloadEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Document(DocumentEvents::Reload(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for LoadStoppedEvent<'_> {
      const DBUS_MEMBER: &'static str = "LoadStopped";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document',member='LoadStopped'";
      const REGISTRY_EVENT_STRING: &'static str = "Document:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for LoadStoppedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Document(DocumentEvents::LoadStopped(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for ContentChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "ContentChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document',member='ContentChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Document:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for ContentChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Document(DocumentEvents::ContentChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for AttributesChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "AttributesChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document',member='AttributesChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Document:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for AttributesChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Document(DocumentEvents::AttributesChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    

    	impl GenericEvent<'_> for PageChangedEvent<'_> {
      const DBUS_MEMBER: &'static str = "PageChanged";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Document";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Document',member='PageChanged'";
      const REGISTRY_EVENT_STRING: &'static str = "Document:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for PageChangedEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Document(DocumentEvents::PageChanged(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    
	
	
	impl From<DocumentEvents<'_>> for Event<'_> {
		fn from(event_enum: DocumentEvents) -> Self {
        Event::Document(event_enum)
		}
	}
	#[cfg(feature = "zbus")]
	impl TryFrom<&zbus::Message> for DocumentEvents<'_> {
		type Error = AtspiError;
		fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
			let member = ev.member()
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
	
  
	impl From<LoadCompleteEvent<'_>> for DocumentEvents<'_> {
		fn from(specific_event: LoadCompleteEvent) -> Self {
			DocumentEvents::LoadComplete(specific_event)
		}
	}
	impl From<LoadCompleteEvent<'_>> for Event<'_> {
		fn from(specific_event: LoadCompleteEvent) -> Self {
			Event::Document(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(LoadCompleteEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(LoadCompleteEvent);
	impl From<LoadCompleteEvent<'_>> for EventBodyOwned {
		fn from(_: LoadCompleteEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<ReloadEvent<'_>> for DocumentEvents<'_> {
		fn from(specific_event: ReloadEvent) -> Self {
			DocumentEvents::Reload(specific_event)
		}
	}
	impl From<ReloadEvent<'_>> for Event<'_> {
		fn from(specific_event: ReloadEvent) -> Self {
			Event::Document(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(ReloadEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(ReloadEvent);
	impl From<ReloadEvent<'_>> for EventBodyOwned {
		fn from(_: ReloadEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<LoadStoppedEvent<'_>> for DocumentEvents<'_> {
		fn from(specific_event: LoadStoppedEvent) -> Self {
			DocumentEvents::LoadStopped(specific_event)
		}
	}
	impl From<LoadStoppedEvent<'_>> for Event<'_> {
		fn from(specific_event: LoadStoppedEvent) -> Self {
			Event::Document(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(LoadStoppedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(LoadStoppedEvent);
	impl From<LoadStoppedEvent<'_>> for EventBodyOwned {
		fn from(_: LoadStoppedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<ContentChangedEvent<'_>> for DocumentEvents<'_> {
		fn from(specific_event: ContentChangedEvent) -> Self {
			DocumentEvents::ContentChanged(specific_event)
		}
	}
	impl From<ContentChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: ContentChangedEvent) -> Self {
			Event::Document(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(ContentChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(ContentChangedEvent);
	impl From<ContentChangedEvent<'_>> for EventBodyOwned {
		fn from(_: ContentChangedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<AttributesChangedEvent<'_>> for DocumentEvents<'_> {
		fn from(specific_event: AttributesChangedEvent) -> Self {
			DocumentEvents::AttributesChanged(specific_event)
		}
	}
	impl From<AttributesChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: AttributesChangedEvent) -> Self {
			Event::Document(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(AttributesChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(AttributesChangedEvent);
	impl From<AttributesChangedEvent<'_>> for EventBodyOwned {
		fn from(_: AttributesChangedEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	

	impl From<PageChangedEvent<'_>> for DocumentEvents<'_> {
		fn from(specific_event: PageChangedEvent) -> Self {
			DocumentEvents::PageChanged(specific_event)
		}
	}
	impl From<PageChangedEvent<'_>> for Event<'_> {
		fn from(specific_event: PageChangedEvent) -> Self {
			Event::Document(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(PageChangedEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(PageChangedEvent);
	impl From<PageChangedEvent<'_>> for EventBodyOwned {
		fn from(_: PageChangedEvent) -> Self {
			EventBodyOwned::default()
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
  	impl HasRegistryEventString for DocumentEvents<'_> {
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
		events::*,
		events::{
			focus::*,
		},
		traits::{GenericEvent, HasMatchRule, HasRegistryEventString, EventBodyOwned, EventBody},
	};
	use zvariant::ObjectPath;
	/*
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
	#[repr(C)]
	#[derive(Clone, Debug)]
	pub enum FocusEvents {
		Focus(FocusEvent<'a>),
	}
	*/
		impl HasMatchRule for FocusEvents<'_> {
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Focus'";
	}
	/*
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
		/// #   atspi.send_event(event_struct).await.unwrap();
    ///
    ///     while let Some(Ok(ev)) = events.next().await {
    ///         if let Ok(event) = FocusEvent::try_from(ev) {
		/// #          break;
		///            // do something with the specific event you've received
		///         } else { continue };
    ///     }
    /// }
    /// ```
    // IgnoreBlock stop
	#[repr(C)]
	#[derive(Debug, PartialEq, Clone, Default)]
	pub struct FocusEvent<'a> {
    pub item: crate::accessible::Accessible<'a>,

}
	*/
	
    	impl GenericEvent<'_> for FocusEvent<'_> {
      const DBUS_MEMBER: &'static str = "Focus";
      const DBUS_INTERFACE: &'static str = "org.a11y.atspi.Event.Focus";
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Focus',member='Focus'";
      const REGISTRY_EVENT_STRING: &'static str = "Focus:";
			
			type Body = EventBodyOwned;

		fn build(item: Accessible, _: Self::Body) -> Result<Self, AtspiError> {
			Ok(Self {
				item,
					
			})
		}
    fn sender(&self) -> zbus_names::UniqueName<'_> {
      zbus_names::UniqueName::try_from(self.item.name.clone()).unwrap()
    }
    fn path<'a>(&self) -> zvariant::ObjectPath<'_> {
      zvariant::ObjectPath::try_from(self.item.path.clone()).unwrap()
    }
		fn body(&self) -> Self::Body {
			let copy = self.clone();
			copy.into()
		}
	}
	
    #[rustfmt::skip]
    impl TryFrom<Event<'_>> for FocusEvent<'_> {
	type Error = AtspiError;
	fn try_from(event: Event) -> Result<Self, Self::Error> {
       if let Event::Focus(FocusEvents::Focus(inner_event)) = event {
				Ok(inner_event)
			} else {
				Err(AtspiError::Conversion("Invalid type"))
			}
		}
	}
    
    
	
	
	impl From<FocusEvents<'_>> for Event<'_> {
		fn from(event_enum: FocusEvents) -> Self {
        Event::Focus(event_enum)
		}
	}
	#[cfg(feature = "zbus")]
	impl TryFrom<&zbus::Message> for FocusEvents<'_> {
		type Error = AtspiError;
		fn try_from(ev: &zbus::Message) -> Result<Self, Self::Error> {
			let member = ev.member()
				.ok_or(AtspiError::MemberMatch("Event without member".into()))?;
			match member.as_str() {
				"Focus" => Ok(FocusEvents::Focus(ev.try_into()?)),
				_ => Err(AtspiError::MemberMatch("No matching member for Focus".into())),
			}
		}
	}
	
  
	impl From<FocusEvent<'_>> for FocusEvents<'_> {
		fn from(specific_event: FocusEvent) -> Self {
			FocusEvents::Focus(specific_event)
		}
	}
	impl From<FocusEvent<'_>> for Event<'_> {
		fn from(specific_event: FocusEvent) -> Self {
			Event::Focus(specific_event.into())
		}
	}
	#[cfg(feature = "zbus")]
	crate::macros::impl_to_dbus_message!(FocusEvent);
	#[cfg(feature = "zbus")]
	crate::macros::impl_from_dbus_message!(FocusEvent);
	impl From<FocusEvent<'_>> for EventBodyOwned {
		fn from(_: FocusEvent) -> Self {
			EventBodyOwned::default()
		}
	}
	
		/*impl HasMatchRule for FocusEvent {
      const MATCH_RULE_STRING: &'static str = "type='signal',interface='org.a11y.atspi.Event.Focus',member='Focus'";
	}*/
  	/*impl HasRegistryEventString for FocusEvent {
		const REGISTRY_EVENT_STRING: &'static str = "Focus:Focus";
	}*/
  	impl HasRegistryEventString for FocusEvents<'_> {
		const REGISTRY_EVENT_STRING: &'static str = "Focus:";
	}
}
	use crate::events::Event;
