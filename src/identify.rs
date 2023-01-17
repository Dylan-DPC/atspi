//! ## Signified signal types
//!
//! The generic `AtspiEvent` has a specific meaning depending on its origin.
//! This module offers the signified types and their conversions from a generic `AtpiEvent`.
//!
//! The `TrySignify` macro implements a `TryFrom<Event>` on a per-name and member basis
//!
pub mod object {
	use atspi_macros::TrySignify;
	use crate::{
		error::AtspiError,
		events::{AtspiEvent, GenericEvent},
		signify::Signified,
	};
	use zbus;
	use zbus::zvariant::OwnedValue;
	
	#[derive(Clone, Debug)]
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
	
	
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct PropertyChangeEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct BoundsChangedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct LinkSelectedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct StateChangedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ChildrenChangedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct VisibleDataChangedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct SelectionChangedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ModelChangedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ActiveDescendantChangedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct AnnouncementEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct AttributesChangedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct RowInsertedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct RowReorderedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct RowDeletedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ColumnInsertedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ColumnReorderedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ColumnDeletedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct TextBoundsChangedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct TextSelectionChangedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct TextChangedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct TextAttributesChangedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct TextCaretMovedEvent(pub(crate) AtspiEvent);
	
	
	impl PropertyChangeEvent {
		
		#[must_use]
		pub fn value(&self) -> &zbus::zvariant::Value<'_> {
			self.0.any_data()
		}
	
	}
	

	impl BoundsChangedEvent {
		
	}
	

	impl LinkSelectedEvent {
		
	}
	

	impl StateChangedEvent {
		
		#[must_use]
		pub fn enabled(&self) -> i32 {
			self.0.detail1()
		}
	
	}
	

	impl ChildrenChangedEvent {
		
		#[must_use]
		pub fn index_in_parent(&self) -> i32 {
			self.0.detail1()
		}
	

		#[must_use]
		pub fn child(&self) -> &zbus::zvariant::Value<'_> {
			self.0.any_data()
		}
	
	}
	

	impl VisibleDataChangedEvent {
		
	}
	

	impl SelectionChangedEvent {
		
	}
	

	impl ModelChangedEvent {
		
	}
	

	impl ActiveDescendantChangedEvent {
		
		#[must_use]
		pub fn child(&self) -> &zbus::zvariant::Value<'_> {
			self.0.any_data()
		}
	
	}
	

	impl AnnouncementEvent {
		
	}
	

	impl AttributesChangedEvent {
		
	}
	

	impl RowInsertedEvent {
		
	}
	

	impl RowReorderedEvent {
		
	}
	

	impl RowDeletedEvent {
		
	}
	

	impl ColumnInsertedEvent {
		
	}
	

	impl ColumnReorderedEvent {
		
	}
	

	impl ColumnDeletedEvent {
		
	}
	

	impl TextBoundsChangedEvent {
		
	}
	

	impl TextSelectionChangedEvent {
		
	}
	

	impl TextChangedEvent {
		
		#[must_use]
		pub fn start_pos(&self) -> i32 {
			self.0.detail1()
		}
	

		#[must_use]
		pub fn end_pos(&self) -> i32 {
			self.0.detail2()
		}
	

		#[must_use]
		pub fn text(&self) -> &zbus::zvariant::Value<'_> {
			self.0.any_data()
		}
	
	}
	

	impl TextAttributesChangedEvent {
		
	}
	

	impl TextCaretMovedEvent {
		
		#[must_use]
		pub fn position(&self) -> i32 {
			self.0.detail1()
		}
	
	}
	
	
	impl TryFrom<AtspiEvent> for ObjectEvents {
		type Error = AtspiError;

		fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"PropertyChange" => Ok(ObjectEvents::PropertyChange(PropertyChangeEvent(ev))),
				"BoundsChanged" => Ok(ObjectEvents::BoundsChanged(BoundsChangedEvent(ev))),
				"LinkSelected" => Ok(ObjectEvents::LinkSelected(LinkSelectedEvent(ev))),
				"StateChanged" => Ok(ObjectEvents::StateChanged(StateChangedEvent(ev))),
				"ChildrenChanged" => Ok(ObjectEvents::ChildrenChanged(ChildrenChangedEvent(ev))),
				"VisibleDataChanged" => Ok(ObjectEvents::VisibleDataChanged(VisibleDataChangedEvent(ev))),
				"SelectionChanged" => Ok(ObjectEvents::SelectionChanged(SelectionChangedEvent(ev))),
				"ModelChanged" => Ok(ObjectEvents::ModelChanged(ModelChangedEvent(ev))),
				"ActiveDescendantChanged" => Ok(ObjectEvents::ActiveDescendantChanged(ActiveDescendantChangedEvent(ev))),
				"Announcement" => Ok(ObjectEvents::Announcement(AnnouncementEvent(ev))),
				"AttributesChanged" => Ok(ObjectEvents::AttributesChanged(AttributesChangedEvent(ev))),
				"RowInserted" => Ok(ObjectEvents::RowInserted(RowInsertedEvent(ev))),
				"RowReordered" => Ok(ObjectEvents::RowReordered(RowReorderedEvent(ev))),
				"RowDeleted" => Ok(ObjectEvents::RowDeleted(RowDeletedEvent(ev))),
				"ColumnInserted" => Ok(ObjectEvents::ColumnInserted(ColumnInsertedEvent(ev))),
				"ColumnReordered" => Ok(ObjectEvents::ColumnReordered(ColumnReorderedEvent(ev))),
				"ColumnDeleted" => Ok(ObjectEvents::ColumnDeleted(ColumnDeletedEvent(ev))),
				"TextBoundsChanged" => Ok(ObjectEvents::TextBoundsChanged(TextBoundsChangedEvent(ev))),
				"TextSelectionChanged" => Ok(ObjectEvents::TextSelectionChanged(TextSelectionChangedEvent(ev))),
				"TextChanged" => Ok(ObjectEvents::TextChanged(TextChangedEvent(ev))),
				"TextAttributesChanged" => Ok(ObjectEvents::TextAttributesChanged(TextAttributesChangedEvent(ev))),
				"TextCaretMoved" => Ok(ObjectEvents::TextCaretMoved(TextCaretMovedEvent(ev))),
				_ => Err(AtspiError::MemberMatch("No matching member for Object".into())),
			}
		}
	}
	
}
	

pub mod window {
	use atspi_macros::TrySignify;
	use crate::{
		error::AtspiError,
		events::{AtspiEvent, GenericEvent},
		signify::Signified,
	};
	use zbus;
	use zbus::zvariant::OwnedValue;
	
	#[derive(Clone, Debug)]
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
	
	
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct PropertyChangeEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct MinimizeEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct MaximizeEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct RestoreEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct CloseEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct CreateEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ReparentEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct DesktopCreateEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct DesktopDestroyEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct DestroyEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ActivateEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct DeactivateEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct RaiseEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct LowerEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct MoveEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ResizeEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ShadeEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct UUshadeEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct RestyleEvent(pub(crate) AtspiEvent);
	
	
	impl PropertyChangeEvent {
		
	}
	

	impl MinimizeEvent {
		
	}
	

	impl MaximizeEvent {
		
	}
	

	impl RestoreEvent {
		
	}
	

	impl CloseEvent {
		
	}
	

	impl CreateEvent {
		
	}
	

	impl ReparentEvent {
		
	}
	

	impl DesktopCreateEvent {
		
	}
	

	impl DesktopDestroyEvent {
		
	}
	

	impl DestroyEvent {
		
	}
	

	impl ActivateEvent {
		
	}
	

	impl DeactivateEvent {
		
	}
	

	impl RaiseEvent {
		
	}
	

	impl LowerEvent {
		
	}
	

	impl MoveEvent {
		
	}
	

	impl ResizeEvent {
		
	}
	

	impl ShadeEvent {
		
	}
	

	impl UUshadeEvent {
		
	}
	

	impl RestyleEvent {
		
	}
	
	
	impl TryFrom<AtspiEvent> for WindowEvents {
		type Error = AtspiError;

		fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"PropertyChange" => Ok(WindowEvents::PropertyChange(PropertyChangeEvent(ev))),
				"Minimize" => Ok(WindowEvents::Minimize(MinimizeEvent(ev))),
				"Maximize" => Ok(WindowEvents::Maximize(MaximizeEvent(ev))),
				"Restore" => Ok(WindowEvents::Restore(RestoreEvent(ev))),
				"Close" => Ok(WindowEvents::Close(CloseEvent(ev))),
				"Create" => Ok(WindowEvents::Create(CreateEvent(ev))),
				"Reparent" => Ok(WindowEvents::Reparent(ReparentEvent(ev))),
				"DesktopCreate" => Ok(WindowEvents::DesktopCreate(DesktopCreateEvent(ev))),
				"DesktopDestroy" => Ok(WindowEvents::DesktopDestroy(DesktopDestroyEvent(ev))),
				"Destroy" => Ok(WindowEvents::Destroy(DestroyEvent(ev))),
				"Activate" => Ok(WindowEvents::Activate(ActivateEvent(ev))),
				"Deactivate" => Ok(WindowEvents::Deactivate(DeactivateEvent(ev))),
				"Raise" => Ok(WindowEvents::Raise(RaiseEvent(ev))),
				"Lower" => Ok(WindowEvents::Lower(LowerEvent(ev))),
				"Move" => Ok(WindowEvents::Move(MoveEvent(ev))),
				"Resize" => Ok(WindowEvents::Resize(ResizeEvent(ev))),
				"Shade" => Ok(WindowEvents::Shade(ShadeEvent(ev))),
				"uUshade" => Ok(WindowEvents::UUshade(UUshadeEvent(ev))),
				"Restyle" => Ok(WindowEvents::Restyle(RestyleEvent(ev))),
				_ => Err(AtspiError::MemberMatch("No matching member for Window".into())),
			}
		}
	}
	
}
	

pub mod mouse {
	use atspi_macros::TrySignify;
	use crate::{
		error::AtspiError,
		events::{AtspiEvent, GenericEvent},
		signify::Signified,
	};
	use zbus;
	use zbus::zvariant::OwnedValue;
	
	#[derive(Clone, Debug)]
	pub enum MouseEvents {
		Abs(AbsEvent),
		Rel(RelEvent),
		Button(ButtonEvent),
	}
	
	
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct AbsEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct RelEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ButtonEvent(pub(crate) AtspiEvent);
	
	
	impl AbsEvent {
		
		#[must_use]
		pub fn x(&self) -> i32 {
			self.0.detail1()
		}
	

		#[must_use]
		pub fn y(&self) -> i32 {
			self.0.detail2()
		}
	
	}
	

	impl RelEvent {
		
		#[must_use]
		pub fn x(&self) -> i32 {
			self.0.detail1()
		}
	

		#[must_use]
		pub fn y(&self) -> i32 {
			self.0.detail2()
		}
	
	}
	

	impl ButtonEvent {
		
		#[must_use]
		pub fn mouse_x(&self) -> i32 {
			self.0.detail1()
		}
	

		#[must_use]
		pub fn mouse_y(&self) -> i32 {
			self.0.detail2()
		}
	
	}
	
	
	impl TryFrom<AtspiEvent> for MouseEvents {
		type Error = AtspiError;

		fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"Abs" => Ok(MouseEvents::Abs(AbsEvent(ev))),
				"Rel" => Ok(MouseEvents::Rel(RelEvent(ev))),
				"Button" => Ok(MouseEvents::Button(ButtonEvent(ev))),
				_ => Err(AtspiError::MemberMatch("No matching member for Mouse".into())),
			}
		}
	}
	
}
	

pub mod keyboard {
	use atspi_macros::TrySignify;
	use crate::{
		error::AtspiError,
		events::{AtspiEvent, GenericEvent},
		signify::Signified,
	};
	use zbus;
	use zbus::zvariant::OwnedValue;
	
	#[derive(Clone, Debug)]
	pub enum KeyboardEvents {
		Modifiers(ModifiersEvent),
	}
	
	
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ModifiersEvent(pub(crate) AtspiEvent);
	
	
	impl ModifiersEvent {
		
		#[must_use]
		pub fn previous_modifiers(&self) -> i32 {
			self.0.detail1()
		}
	

		#[must_use]
		pub fn current_modifiers(&self) -> i32 {
			self.0.detail2()
		}
	
	}
	
	
	impl TryFrom<AtspiEvent> for KeyboardEvents {
		type Error = AtspiError;

		fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"Modifiers" => Ok(KeyboardEvents::Modifiers(ModifiersEvent(ev))),
				_ => Err(AtspiError::MemberMatch("No matching member for Keyboard".into())),
			}
		}
	}
	
}
	

pub mod terminal {
	use atspi_macros::TrySignify;
	use crate::{
		error::AtspiError,
		events::{AtspiEvent, GenericEvent},
		signify::Signified,
	};
	use zbus;
	use zbus::zvariant::OwnedValue;
	
	#[derive(Clone, Debug)]
	pub enum TerminalEvents {
		LineChanged(LineChangedEvent),
		ColumnCountChanged(ColumnCountChangedEvent),
		LineCountChanged(LineCountChangedEvent),
		ApplicationChanged(ApplicationChangedEvent),
		CharWidthChanged(CharWidthChangedEvent),
	}
	
	
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct LineChangedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ColumnCountChangedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct LineCountChangedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ApplicationChangedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct CharWidthChangedEvent(pub(crate) AtspiEvent);
	
	
	impl LineChangedEvent {
		
	}
	

	impl ColumnCountChangedEvent {
		
	}
	

	impl LineCountChangedEvent {
		
	}
	

	impl ApplicationChangedEvent {
		
	}
	

	impl CharWidthChangedEvent {
		
	}
	
	
	impl TryFrom<AtspiEvent> for TerminalEvents {
		type Error = AtspiError;

		fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"LineChanged" => Ok(TerminalEvents::LineChanged(LineChangedEvent(ev))),
				"ColumncountChanged" => Ok(TerminalEvents::ColumnCountChanged(ColumnCountChangedEvent(ev))),
				"LinecountChanged" => Ok(TerminalEvents::LineCountChanged(LineCountChangedEvent(ev))),
				"ApplicationChanged" => Ok(TerminalEvents::ApplicationChanged(ApplicationChangedEvent(ev))),
				"CharwidthChanged" => Ok(TerminalEvents::CharWidthChanged(CharWidthChangedEvent(ev))),
				_ => Err(AtspiError::MemberMatch("No matching member for Terminal".into())),
			}
		}
	}
	
}
	

pub mod document {
	use atspi_macros::TrySignify;
	use crate::{
		error::AtspiError,
		events::{AtspiEvent, GenericEvent},
		signify::Signified,
	};
	use zbus;
	use zbus::zvariant::OwnedValue;
	
	#[derive(Clone, Debug)]
	pub enum DocumentEvents {
		LoadComplete(LoadCompleteEvent),
		Reload(ReloadEvent),
		LoadStopped(LoadStoppedEvent),
		ContentChanged(ContentChangedEvent),
		AttributesChanged(AttributesChangedEvent),
		PageChanged(PageChangedEvent),
	}
	
	
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct LoadCompleteEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ReloadEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct LoadStoppedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct ContentChangedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct AttributesChangedEvent(pub(crate) AtspiEvent);
	

	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct PageChangedEvent(pub(crate) AtspiEvent);
	
	
	impl LoadCompleteEvent {
		
	}
	

	impl ReloadEvent {
		
	}
	

	impl LoadStoppedEvent {
		
	}
	

	impl ContentChangedEvent {
		
	}
	

	impl AttributesChangedEvent {
		
	}
	

	impl PageChangedEvent {
		
	}
	
	
	impl TryFrom<AtspiEvent> for DocumentEvents {
		type Error = AtspiError;

		fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"LoadComplete" => Ok(DocumentEvents::LoadComplete(LoadCompleteEvent(ev))),
				"Reload" => Ok(DocumentEvents::Reload(ReloadEvent(ev))),
				"LoadStopped" => Ok(DocumentEvents::LoadStopped(LoadStoppedEvent(ev))),
				"ContentChanged" => Ok(DocumentEvents::ContentChanged(ContentChangedEvent(ev))),
				"AttributesChanged" => Ok(DocumentEvents::AttributesChanged(AttributesChangedEvent(ev))),
				"PageChanged" => Ok(DocumentEvents::PageChanged(PageChangedEvent(ev))),
				_ => Err(AtspiError::MemberMatch("No matching member for Document".into())),
			}
		}
	}
	
}
	

pub mod focus {
	use atspi_macros::TrySignify;
	use crate::{
		error::AtspiError,
		events::{AtspiEvent, GenericEvent},
		signify::Signified,
	};
	use zbus;
	use zbus::zvariant::OwnedValue;
	
	#[derive(Clone, Debug)]
	pub enum FocusEvents {
		Focus(FocusEvent),
	}
	
	
	#[derive(Debug, PartialEq, Eq, Clone, TrySignify)]
	pub struct FocusEvent(pub(crate) AtspiEvent);
	
	
	impl FocusEvent {
		
	}
	
	
	impl TryFrom<AtspiEvent> for FocusEvents {
		type Error = AtspiError;

		fn try_from(ev: AtspiEvent) -> Result<Self, Self::Error> {
			let Some(member) = ev.member() else { return Err(AtspiError::MemberMatch("Event w/o member".into())); };
			match member.as_str() {
				"Focus" => Ok(FocusEvents::Focus(FocusEvent(ev))),
				_ => Err(AtspiError::MemberMatch("No matching member for Focus".into())),
			}
		}
	}
	
}
	
