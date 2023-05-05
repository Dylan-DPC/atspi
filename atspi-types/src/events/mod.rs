use wai_bindgen_rust::*;
import!("./src/events/events.wai");
pub use events::*;
pub mod object {
pub use super::ObjectEvents;pub use super::ObjectPropertyChangeEvent as PropertyChangeEvent;
pub use super::ObjectBoundsChangedEvent as BoundsChangedEvent;
pub use super::ObjectLinkSelectedEvent as LinkSelectedEvent;
pub use super::ObjectStateChangedEvent as StateChangedEvent;
pub use super::ObjectChildrenChangedEvent as ChildrenChangedEvent;
pub use super::ObjectVisibleDataChangedEvent as VisibleDataChangedEvent;
pub use super::ObjectSelectionChangedEvent as SelectionChangedEvent;
pub use super::ObjectModelChangedEvent as ModelChangedEvent;
pub use super::ObjectActiveDescendantChangedEvent as ActiveDescendantChangedEvent;
pub use super::ObjectAnnouncementEvent as AnnouncementEvent;
pub use super::ObjectAttributesChangedEvent as AttributesChangedEvent;
pub use super::ObjectRowInsertedEvent as RowInsertedEvent;
pub use super::ObjectRowReorderedEvent as RowReorderedEvent;
pub use super::ObjectRowDeletedEvent as RowDeletedEvent;
pub use super::ObjectColumnInsertedEvent as ColumnInsertedEvent;
pub use super::ObjectColumnReorderedEvent as ColumnReorderedEvent;
pub use super::ObjectColumnDeletedEvent as ColumnDeletedEvent;
pub use super::ObjectTextBoundsChangedEvent as TextBoundsChangedEvent;
pub use super::ObjectTextSelectionChangedEvent as TextSelectionChangedEvent;
pub use super::ObjectTextChangedEvent as TextChangedEvent;
pub use super::ObjectTextAttributesChangedEvent as TextAttributesChangedEvent;
pub use super::ObjectTextCaretMovedEvent as TextCaretMovedEvent;
}
pub mod window {
pub use super::WindowEvents;pub use super::WindowPropertyChangeEvent as PropertyChangeEvent;
pub use super::WindowMinimizeEvent as MinimizeEvent;
pub use super::WindowMaximizeEvent as MaximizeEvent;
pub use super::WindowRestoreEvent as RestoreEvent;
pub use super::WindowCloseEvent as CloseEvent;
pub use super::WindowCreateEvent as CreateEvent;
pub use super::WindowReparentEvent as ReparentEvent;
pub use super::WindowDesktopCreateEvent as DesktopCreateEvent;
pub use super::WindowDesktopDestroyEvent as DesktopDestroyEvent;
pub use super::WindowDestroyEvent as DestroyEvent;
pub use super::WindowActivateEvent as ActivateEvent;
pub use super::WindowDeactivateEvent as DeactivateEvent;
pub use super::WindowRaiseEvent as RaiseEvent;
pub use super::WindowLowerEvent as LowerEvent;
pub use super::WindowMoveEvent as MoveEvent;
pub use super::WindowResizeEvent as ResizeEvent;
pub use super::WindowShadeEvent as ShadeEvent;
pub use super::WindowUUshadeEvent as UUshadeEvent;
pub use super::WindowRestyleEvent as RestyleEvent;
}
pub mod mouse {
pub use super::MouseEvents;pub use super::MouseAbsEvent as AbsEvent;
pub use super::MouseRelEvent as RelEvent;
pub use super::MouseButtonEvent as ButtonEvent;
}
pub mod keyboard {
pub use super::KeyboardEvents;pub use super::KeyboardModifiersEvent as ModifiersEvent;
}
pub mod terminal {
pub use super::TerminalEvents;pub use super::TerminalLineChangedEvent as LineChangedEvent;
pub use super::TerminalColumnCountChangedEvent as ColumnCountChangedEvent;
pub use super::TerminalLineCountChangedEvent as LineCountChangedEvent;
pub use super::TerminalApplicationChangedEvent as ApplicationChangedEvent;
pub use super::TerminalCharWidthChangedEvent as CharWidthChangedEvent;
}
pub mod document {
pub use super::DocumentEvents;pub use super::DocumentLoadCompleteEvent as LoadCompleteEvent;
pub use super::DocumentReloadEvent as ReloadEvent;
pub use super::DocumentLoadStoppedEvent as LoadStoppedEvent;
pub use super::DocumentContentChangedEvent as ContentChangedEvent;
pub use super::DocumentAttributesChangedEvent as AttributesChangedEvent;
pub use super::DocumentPageChangedEvent as PageChangedEvent;
}
pub mod focus {
pub use super::FocusEvents;pub use super::FocusFocusEvent as FocusEvent;
}
pub mod cache {
pub use super::CacheEvents;pub use super::CacheAddEvent as AddEvent;
pub use super::CacheRemoveEvent as RemoveEvent;
}
pub mod registry {
pub use super::RegistryEvents;pub use super::RegistryEventListenerRegisteredEvent as EventListenerRegisteredEvent;
pub use super::RegistryEventListenerDeregisteredEvent as EventListenerDeregisteredEvent;
}
pub mod socket {
pub use super::SocketEvents;pub use super::SocketAvailableEvent as AvailableEvent;
}
pub mod device_event_listener {
pub use super::DeviceEventListenerEvents;pub use super::DeviceEventListenerKeystrokeListenerRegisteredEvent as KeystrokeListenerRegisteredEvent;
pub use super::DeviceEventListenerKeystrokeListenerDeregisteredEvent as KeystrokeListenerDeregisteredEvent;
}
pub use events::Event;
