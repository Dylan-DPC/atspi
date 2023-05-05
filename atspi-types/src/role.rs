use serde::{Deserialize, Serialize};
use zvariant::Type;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Type, Hash)]
/// An accessible object role.
/// To think of it in terms of HTML, any semantic element likely has a corollary in this enum.
/// For example: `<button>`, `<input>`, `<form>` or `<h4>`.
/// Non-semantic elements like `<span>`, `<div>` and `<b>` will not be represented here, and this information is not passed through via the atspi library.
/// TODO: add examples for GTK/Qt libraries in addition to HTML examples.
#[repr(u32)]
pub enum Role {
	/// An invalid role used for either an invalid deserialization, or when trying to match for any possible role. TODO: confirm
	Invalid,
	/// Unknown role TODO
	AcceleratorLabel,
	/// Alert: this is triggered in a web browser through the alert(...); function.
	Alert,
	/// Animation: unknown use TODO
	Animation,
	/// Arrow: unknown use TODO
	Arrow,
	/// Calendar: a calendar widget, or in HTML terms, `<input type="datetime-local">`
	Calendar,
	/// A canvas on which any direct rendering may be called. In web terms, this would be the `<canvas>` element.
	Canvas,
	/// A (multiple) checkbox. In HTML terms, `<input type="checkbox">`, note that there is a different role for radio buttons and for multiple select dropdowns.
	CheckBox,
	/// CheckMenuItem: unknown use. TODO
	CheckMenuItem,
	/// ColorChooser: a color picker input. In HTML terms, `<input type="color">`
	ColorChooser,
	/// Column header: in HTML terms, a `<th>`.
	ColumnHeader,
	/// A multiple select dropdown menu.
	ComboBox,
	/// Date editor: unknown use case. TODO
	DateEditor,
	/// A desktop icon: on Windows this would be the "Recycle Bin", or "My Computer" on your desktop. On Linux this would be similar to any applications showing on your desktop.
	DesktopIcon,
	/// The frame within all windows live. A DesktopFrame will generally share siblings with others of the same type if you use multiple desktops.
	DesktopFrame,
	/// Dial: unknown use case. TODO
	Dial,
	/// Dialog: a pop-up dialog. In HTML terms, the `<dialog>` tag.
	Dialog,
	/// Directory pane: unknown use case.
	DirectoryPane,
	DrawingArea,
	/// File chooser: this is a window which will display when  
	FileChooser,
	/// Filter: unknown use: TODO.
	Filler,
	/// Focus traversable: TODO
	FocusTraversable,
	/// Font chooser: TODO, but obvious?
	FontChooser,
	/// Frame: generally the parent of InternalFrame.
	Frame,
	/// Glass pane? TODO
	GlassPane,
	/// Constraining container of which only HTML resides in. This is useful during structural navigation calls to bound the search area to only HTML elements of the page.
	HTMLContainer,
	/// TODO: icon
	Icon,
	/// An image. In HTML, this would be the <img> tag, or it could be an image embedded within a non-web application.
	Image,
	/// Internal frame: this is the constraining role for a graphical window. This is a good bounding role for finding things from within an application.
	InternalFrame,
	/// A label, which is generally associated with an item with a different role. In HTML terms, this would be a `<label for="X">` being attached to whatever `<Y id="X">` is.
	Label,
	/// Layered pane? TODO
	LayeredPane,
	/// List: a list with [`Self::ListItem`] contained within it. In HTML, this would be the same as the `<ul>` or `<ol>` elements.
	List,
	/// ListItem: a list's item. This would be the same as an `<li>` in HTML terms.
	ListItem,
	Menu,
	MenuBar,
	MenuItem,
	OptionPane,
	PageTab,
	PageTabList,
	Panel,
	/// A password input, like `<input type="password">`.
	PasswordText,
	PopupMenu,
	/// Progress bar: this indicates progress of some process, and generally is indicated by successively higher-pitched beeps on a screen reader as it is updated to a more and more highly completed state. In HTML this would be the same as `<progress>` tag.
	ProgressBar,
	/// PushButton: this is what everybody else calls a button. In HTML, `<button>`
	PushButton,
	/// Radio button: a multiple-choice, single-selection option. In HTML: `<input type="radio">`.
	RadioButton,
	RadioMenuItem,
	/// Root pane: the mother of *ALL* panes. This is the pane from which all other panes descend. If you wanted to, for some reason, search within a bound of the entire active desktop, this would be your bounding pane.
	RootPane,
	/// Row header: a heading to a row. In HTML this would be the same as `<th role="rowheader">` without the additional role="..." attribute, the header will still be recognized as a column header.
	RowHeader,
	/// A scroll bar itself: the item you may click on and scroll up and down.
	ScrollBar,
	/// A scroll pane: the pane in which the scrollable content is contained within.
	ScrollPane,
	/// Separator: commonly used in desktop applications to pad out the interface. This also is the same as the `<hr>` element in HTML.
	Separator,
	/// Slider: a slider to control a granular value like volume, pitch, or speed.
	Slider,
	/// spin button: ? TODO
	SpinButton,
	/// Split pane: ? TODO
	SplitPane,
	/// Status bar: ? TODO
	StatusBar,
	/// Table: a table. This may hold any tabular data with rows and columns. This would be the same as the `<table>` element in HTML.
	Table,
	/// A table cell: this may hold a singular piece of data at a row+column combo. This is the same as `<td>` in HTML.
	TableCell,
	/// The column header of a table, represented in HTML as a `<th>`
	TableColumnHeader,
	/// The row heading of a table, represented in HTML as a `<th scope="row">`.
	TableRowHeader,
	TearoffMenuItem,
	/// A virtual terminal like MATE Terminal, Foot, or `st`.
	Terminal,
	Text,
	ToggleButton,
	ToolBar,
	ToolTip,
	/// The root of a tree, which may have many sub trees and tree items (leafs).
	Tree,
	TreeTable,
	/// When the role cannot be accurately reported, this role will be set.
	Unknown,
	Viewport,
	/// A window itself, not the same thing as a Pane or a Frame, which are both contained within a
	/// Window.
	Window,
	Extended,
	/// A header with upfront information about a document, site, or application. The same as `<header>` in HTML.
	Header,
	/// A footer with additional (usually optional) information about a web page, document, or application. The same as `<footer>` in HTML.
	Footer,
	/// A paragraph of text: the same as `<p>` in HTML.
	Paragraph,
	/// A horizontal line between two items. Usually a `<hr>` in HTML.
	Ruler,
	Application,
	Autocomplete,
	Editbar,
	Embedded,
	Entry,
	CHART,
	Caption,
	DocumentFrame,
	/// Heading: this is a heading with a level (usually 1-6). This is represented by `<h1>` through `<h6>` in HTML.
	Heading,
	Page,
	/// Section: pieces of grouped content for semantic purposes. This is the same as the `<section>` tag in HTML.
	Section,
	RedundantObject,
	/// Form: a form where a user will input information and send the form out (usually to an online service). The same as the `<form>` element in HTML.
	Form,
	/// Link: a hyperlink that leads to a new destination. The same as the `<a>` tag in HTML.
	Link,
	InputMethodWindow,
	/// Table row: a row of table data. This is the same as the `<tr>` tag from HTML.
	TableRow,
	/// A leaf or node within a tree.
	TreeItem,
	/// A spreadsheet document (almost exclusively used by LibreofficeCalc).
	DocumentSpreadsheet,
	/// A presentation document (almost exclusively used by LibreofficePresent).
	DocumentPresentation,
	/// A text document (almost exclusively used by LibreofficeWriter).
	DocumentText,
	/// A web document, used for any web browser.
	DocumentWeb,
	/// An email document, used primarily by Thunderbird.
	DocumentEmail,
	Comment,
	ListBox,
	Grouping,
	ImageMap,
	/// Notification: this is generally displayed and made accessible by a notification daemon. For example `dunst`.
	Notification,
	InfoBar,
	LevelBar,
	TitleBar,
	/// Block quote: when a quote is longer than around one full sentence, a block-style quote often make more sense. This is the same as the `<blockquote>` HTML tag.
	BlockQuote,
	/// Audio: a role which can play sound. In HTML: `<audio>`
	Audio,
	/// Video: a role which can play a video (with optional sound). In HTML: `<video>`
	Video,
	Definition,
	Article,
	Landmark,
	Log,
	Marquee,
	/// Math: a special role for when math equations appear. This is the same as the `<math>` tag in HTML, indicating embedded MathML.
	Math,
	/// A rating system, generally out of five stars, but it does not need to be that way. There is no tag nor role for this in HTML, however.
	Rating,
	Timer,
	Static,
	MathFraction,
	MathRoot,
	Subscript,
	Superscript,
	/// A list with Term/Value subitems. This is the same as `<dl>` in HTML.
	DescriptionList,
	/// An item (usually inside a [`Self::DescriptionList`]) that has a term as its content.
	/// The same as the `<dt>` tag in HTML.
	DescriptionTerm,
	/// An item (usually inside a [`Self::DescriptionList`]) that has a term's value as its
	/// content. This is the same as a `<dd>` tag in HTML.
	DescriptionValue,
	Footnote,
	ContentDeletion,
	ContentInsertion,
	Mark,
	Suggestion,
	PushButtonMenu,
}

const ROLE_NAMES: &[&str] = &[
	"invalid",
	"accelerator label",
	"alert",
	"animation",
	"arrow",
	"calendar",
	"canvas",
	"check box",
	"check menu item",
	"color chooser",
	"column header",
	"combo box",
	"date editor",
	"desktop icon",
	"desktop frame",
	"dial",
	"dialog",
	"directory pane",
	"drawing area",
	"file chooser",
	"filler",
	"focus traversable",
	"font chooser",
	"frame",
	"glass pane",
	"html container",
	"icon",
	"image",
	"internal frame",
	"label",
	"layered pane",
	"list",
	"list item",
	"menu",
	"menu bar",
	"menu item",
	"option pane",
	"page tab",
	"page tab list",
	"panel",
	"password text",
	"popup menu",
	"progress bar",
	"push button",
	"radio button",
	"radio menu item",
	"root pane",
	"row header",
	"scroll bar",
	"scroll pane",
	"separator",
	"slider",
	"spin button",
	"split pane",
	"status bar",
	"table",
	"table cell",
	"table column header",
	"table row header",
	"tearoff menu item",
	"terminal",
	"text",
	"toggle button",
	"tool bar",
	"tool tip",
	"tree",
	"tree table",
	"unknown",
	"viewport",
	"window",
	"extended",
	"header",
	"footer",
	"paragraph",
	"ruler",
	"application",
	"autocomplete",
	"editbar",
	"embedded",
	"entry",
	"chart",
	"caption",
	"document frame",
	"heading",
	"page",
	"section",
	"redundant object",
	"form",
	"link",
	"input method window",
	"table row",
	"tree item",
	"document spreadsheet",
	"document presentation",
	"document text",
	"document web",
	"document email",
	"comment",
	"list box",
	"grouping",
	"image map",
	"notification",
	"info bar",
	"level bar",
	"title bar",
	"block quote",
	"audio",
	"video",
	"definition",
	"article",
	"landmark",
	"log",
	"marquee",
	"math",
	"rating",
	"timer",
	"static",
	"math fraction",
	"math root",
	"subscript",
	"superscript",
	"description list",
	"description term",
	"description value",
	"footnote",
	"content deletion",
	"content insertion",
	"mark",
	"suggestion",
	"push button menu",
];

impl Role {
	#[must_use]
	pub fn name(&self) -> &'static str {
		ROLE_NAMES[*self as usize]
	}
}
