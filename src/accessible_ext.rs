use crate::AtspiError;
use crate::{
	accessible::{
		Accessible, AccessibleBlocking, AccessibleProxy, AccessibleProxyBlocking, RelationType,
		Role,
	},
	collection::MatchRule,
	convertable::{Convertable, ConvertableBlocking},
	hyperlink::Hyperlink,
	state::StateSet,
	text::{Text, TextBlocking},
};
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait AccessibleExt {
	type Error: std::error::Error;

	async fn get_indexed_children<'a>(&self) -> Result<Vec<(i32, Self)>, Self::Error>
	where
		Self: Sized;

	async fn get_indexed_siblings<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized;

	async fn get_siblings_before<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized;

	async fn get_siblings_after<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized;

	async fn get_hyperlinks_adjacent_to_caret<'a>(&self) -> Result<[Vec<Self>; 3], Self::Error>
	where
		Self: Sized;

	async fn get_next_hyperlink<'a>(
		&self,
		rule: &MatchRule,
		backward: bool,
		with_unknown: bool,
	) -> Result<Option<Self>, Self::Error>
	where
		Self: Sized;

	async fn get_relation_set_ext<'a>(
		&self,
	) -> Result<HashMap<RelationType, Vec<Self>>, Self::Error>
	where
		Self: Sized;

	async fn find_inner<'a>(
		&self,
		before_or_after: i32,
		rule: &MatchRule,
		backward: bool,
		recur: bool,
	) -> Result<Option<Self>, Self::Error>
	where
		Self: Sized;

	async fn matches_rule(&self, rule: &MatchRule) -> Result<bool, Self::Error>;
}

// TODO: implement AccessibleExt
pub trait AccessibleBlockingExt {}

#[allow(clippy::module_name_repetitions)]
pub trait AccessibleExtError: Accessible + Convertable {
	type Error: std::error::Error
		+ From<<Self as Accessible>::Error>
		+ From<<Self as Convertable>::Error>
		// TODO: add all convertable error types
		+ From<<<Self as Convertable>::Text as Text>::Error>
		+ From<std::num::TryFromIntError>
		+ Send;
}

#[allow(clippy::module_name_repetitions)]
pub trait AccessibleBlockingExtError: AccessibleBlocking + ConvertableBlocking {
	type Error: std::error::Error
		+ From<<Self as AccessibleBlocking>::Error>
		+ From<<Self as ConvertableBlocking>::Error>
		// TODO: add all convertable error types
		+ From<<<Self as ConvertableBlocking>::Text as TextBlocking>::Error>
		+ From<std::num::TryFromIntError>;
}

#[async_trait]
impl<T: Accessible + Convertable> AccessibleExt for T {
	type Error = crate::AtspiError;

	/// Retrieves children with their respective indexes in parent, the current `Accessible`.
	async fn get_indexed_children<'a>(&self) -> Result<Vec<(i32, Self)>, AtspiError> {
		let children = self.get_children().await?;
		let mut indexed: Vec<(i32, Self)> = Vec::with_capacity(children.len());
		for child in children {
			let idx = child.get_index_in_parent().await?;
			indexed.push((idx, child));
		}
		Ok(indexed)
	}

	/// Retrieves all siblings with their respective indexes in the shared parent, the current `Accessible`
	async fn get_indexed_siblings<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized,
	{
		let parent = self.parent().await?;
		let mut siblings = parent.get_indexed_children().await?;
		let index = self.get_index_in_parent().await?;
		siblings.retain(|(&idx, _)| idx != index);
		Ok(siblings)
	}

	/// Retrieve sibling objects in parent which index is smaller than this `Accessible`
	///
	/// # Efficiency note:
	/// This method is provided for convenience.
	///
	/// This function calls `get_indexed_siblings()` and discards unwanted siblings,
	/// If  you have a use for all siblings, it may be more efficient to keep those around,
	/// and call `get_indexed_siblings()` instead.
	async fn get_siblings_before<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized,
	{
		let index = self.get_index_in_parent().await?;
		let siblings = self.get_indexed_siblings().await?;
		siblings.retain(|(idx, _)| idx < index);
		Ok(siblings)
	}

	/// Retrieve sibling objects in parent which index is smaller than this `Accessible`
	///
	/// # Efficiency note:
	/// This method is provided for convenience.
	///
	/// This function calls `get_indexed_siblings()` and discards unwanted siblings,
	/// If  you have a use for all siblings, it may be more efficient to keep those around,
	/// and call `get_indexed_siblings()` instead.
	async fn get_siblings_after<'a>(&self) -> Result<Vec<Self>, Self::Error>
	where
		Self: Sized,
	{
		let index = self.get_index_in_parent().await?;
		let siblings = self.get_indexed_siblings().await?;
		siblings.retain(|(idx, _)| idx > index);
		Ok(siblings)
	}

	/// Return 'hyperlink children' before and after the caret position, if hyperlink child objects exist.
	///
	/// If hyperlink objects'  'IndexAtStart' property does not yield a value, then these children are found in a third vector.
	async fn get_hyperlinks_adjacent_to_caret<'a>(&self) -> Result<[Vec<Self>; 3], Self::Error>
	where
		Self: Sized,
	{
		// If 'self' is text-object, get caret-offset, or bail.
		let text_iface = self.to_text().await?;
		let caret_pos = text_iface.caret_offset().await?;

		let text_children = self.get_children.await?;
		let length = text_children.len();

		// Collect the text-object's children.
		// Collect in either of three categories:
		let mut lhs = Vec::with_capacity(length);
		let mut rhs = Vec::with_capacity(length);
		let mut unknown = Vec::with_capacity(length);

		for child in text_children {
			// Presumably all `Text` children are `Hyperlink`'s..
			let Ok(hyperlink) = child.to_hyperlink().await else { continue };
			match hyperlink.start_index().await {
				Ok(start_idx) if start_idx <= caret_pos => lhs.push(child),
				Ok(start_idx) if start_idx >= caret_pos => rhs.push(child),
				_ => unknown.push(child),
			}
		}
		Ok([lhs, rhs, unknown])
	}

	// Should this be replaced with a hyperlink Stream?
	async fn get_next_hyperlink<'a>(
		&self,
		rule: &MatchRule,
		backward: bool,
		with_unknown: bool,
	) -> Result<Option<Self>, Self::Error>
	where
		Self: Sized,
	{
		let [lhs, rhs, unknown] = self.get_hyperlinks_adjacent_to_caret().await?;
		let hyperlinks = if backward { lhs.into_iter() } else { rhs.into_iter() };

		if with_unknown {
			let hyperlinks = hyperlinks.chain(unknown.into_iter());
		}

		// For each link in the list of caret adjacent hyperlink-children,
		// if the link matches the provided rule, return it

		for link in hyperlinks {
			if link.match_(rule).await? {
				return Ok(Some(link));
			} else if let Some(found_sub) = link.find_inner(0, rule, backward, true).await? {
				return Ok(Some(found_sub));
			}
		}

		// No hyperlink matching 'rule' was found
		let mut last_parent_index = self.get_index_in_parent().await?;

		if let Ok(mut parent) = self.get_parent().await {
			while parent.get_role().await? != Role::InternalFrame {
				let found_inner_child =
					parent.find_inner(last_parent_index, rule, backward, false).await?;
				if found_inner_child.is_some() {
					return Ok(found_inner_child);
				}
				last_parent_index = parent.get_index_in_parent().await?;
				parent = parent.get_parent_ext().await?;
			}
		}
		Ok(None)
	}

	async fn get_relation_set_ext<'a>(
		&self,
	) -> Result<HashMap<RelationType, Vec<Self>>, Self::Error>
	where
		Self: Sized,
	{
		let raw_relations = self.get_relation_set().await?;
		Ok(HashMap::from_iter(raw_relations))
	}

	/// Find a descendant by matching against `MatcherArgs`.
	/// before_or_after
	async fn find_inner<'a>(
		&self,
		before_or_after: i32,
		rule: &MatchRule,
		backward: bool,
		recur: bool,
	) -> Result<Option<Self>, Self::Error>
	where
		Self: Sized,
	{
		let children = if backward {
			let mut vec = self.get_children_ext().await?;
			vec.reverse();
			vec
		} else {
			self.get_children_ext().await?
		};

		for child in children {
			let child_index = child.get_index_in_parent().await?;
			if !recur
				&& ((child_index <= before_or_after && !backward)
					|| (child_index >= before_or_after && backward))
			{
				continue;
			}

			// If the child's role corresponds with the first and sole role of MatcherArgs
			if child.match_(rule).await? {
				return Ok(Some(child));
			}

			/* 0 here is ignored because we are recursive; see the line starting with if !recur */
			if let Some(found_descendant) = child.find_inner(0, rule, backward, true).await? {
				return Ok(Some(found_descendant));
			}
		}
		Ok(None)
	}

	async fn matches_rule(&self, rule: &MatchRule) -> Result<bool, Self::Error> {
		let mut result = false;
		if !(*rule).states.is_empty() {}

		// Invert the result if rule inversion was selected:
		return if rule.invert == &true { Ok(!result) } else { Ok(result) };
	}
}

impl<T: AccessibleBlocking + ConvertableBlocking> AccessibleBlockingExt for T {}

assert_impl_all!(AccessibleProxy: Accessible, AccessibleExt);
assert_impl_all!(AccessibleProxyBlocking: AccessibleBlocking, AccessibleBlockingExt);
