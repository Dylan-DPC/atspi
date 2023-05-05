pub use crate::events::{Accessible, Role, Iface, State};

use zvariant::{Signature, Type};

macro_rules! impl_type {
	($for_type:ty, $sig_type:ty) => {
		impl Type for $for_type {
			fn signature() -> Signature<'static> {
				<$sig_type>::signature()
			}
		}
	}
}

impl_type!(State, u64);
assert_eq_size!(State, u64);
assert_impl_all!(State: Clone, Copy, std::fmt::Debug, Eq, PartialEq);
impl_type!(Iface, u32);
assert_eq_size!(Iface, u32);
assert_impl_all!(Iface: Clone, Copy, std::fmt::Debug, Eq, PartialEq);
impl_type!(Role, u32);
assert_eq_size!(Role, u8);
assert_impl_all!(Role: Clone, Copy, std::fmt::Debug, Eq, PartialEq);


