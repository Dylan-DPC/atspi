use atspi::{
	events::GenericEvent,
	identify::object::{ObjectEvents, StateChangedEvent},
	signify::Signified,
};
use std::error::Error;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let atspi = atspi::AccessibilityConnection::open().await?;
	atspi.register_event::<ObjectEvents>().await?;

	let events = atspi.event_stream();
	tokio::pin!(events);

	while let Some(Ok(ev)) = events.next().await {
		let Ok(change)  = <StateChangedEvent>::try_from(ev) else { continue };

		if change.kind() == "focused" && change.enabled() == 1 {
			let Some(bus_name) = change.inner().sender()? else { continue };
			println!("Accessible belonging to {bus_name}  focused!");
		}
	}
	Ok(())
}
