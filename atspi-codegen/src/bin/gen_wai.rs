use std::{
    vec,
};

use atspi_codegen::*;
use convert_case::{Case, Casing};
use zvariant::*;

pub fn iface_name(interface: &Interface) -> String {
	interface
		.name()
		.split('.')
		.next_back()
		.expect("An interface must have a period in the name")
		.to_string()
}

pub fn generate_wai_flag(iface: &Interface, signal: &Signal) -> String {
	format!("\t{}-{},", iface_name(iface).to_case(Case::Kebab), signal.name().to_case(Case::Kebab))
}
pub fn struct_name(signal: &Signal) -> String {
	signal.name().to_case(Case::Kebab)
}

pub fn  get_root_node_from_xml(file_name: &str) -> Node {
    let xml_file = std::fs::File::open(file_name).expect("Cannot read file");
    Node::from_reader(&xml_file).expect("Cannot deserialize file")
}

pub fn event_type_flags(nodes: &[Node]) -> String {
		let flags = nodes.iter()
			.map(|node| for_signals(node, generate_wai_flag))
			.collect::<Vec<String>>()
			.join("\n");
	format!(
r#"flags event-type {{
{flags}
}}"#)
}

// taken from zbus_xmlgen: https://gitlab.freedesktop.org/dbus/zbus/-/blob/main/zbus_xmlgen/src/gen.rs
fn _to_wasm_type(ty: &str, input: bool, as_ref: bool) -> String {
    // can't haz recursive closure, yet
    fn iter_to_rust_type(
        it: &mut std::iter::Peekable<std::slice::Iter<'_, u8>>,
        input: bool,
        as_ref: bool,
    ) -> String {
        let c = it.next().unwrap();
        match *c as char {
            u8::SIGNATURE_CHAR => "u8".into(),
            bool::SIGNATURE_CHAR => "bool".into(),
            i16::SIGNATURE_CHAR => "s16".into(),
            u16::SIGNATURE_CHAR => "u16".into(),
            i32::SIGNATURE_CHAR => "s32".into(),
            u32::SIGNATURE_CHAR => "u32".into(),
            i64::SIGNATURE_CHAR => "s64".into(),
            u64::SIGNATURE_CHAR => "u64".into(),
            f64::SIGNATURE_CHAR => "float64".into(),
            // xmlgen accepts 'h' on Windows, only for code generation
            'h' => "u32".into(),
            <&str>::SIGNATURE_CHAR => "string".into(),
            ObjectPath::SIGNATURE_CHAR => "string".into(),
            Signature::SIGNATURE_CHAR => "string".into(),
						// TODO: use a proper variant
            VARIANT_SIGNATURE_CHAR => "zbus-value".into(),
            ARRAY_SIGNATURE_CHAR => {
                let c = it.peek().unwrap();
                match **c as char {
                    '{' => format!(
                        "list<tuple<{}>>",
                        iter_to_rust_type(it, input, false)
                    ),
                    _ => {
                        let ty = iter_to_rust_type(it, input, false);
                        if input {
                            format!("&[{ty}]")
                        } else {
                            format!("{}list<{}>", if as_ref { "&" } else { "" }, ty)
                        }
                    }
                }
            }
            c @ STRUCT_SIG_START_CHAR | c @ DICT_ENTRY_SIG_START_CHAR => {
                let dict = c == '{';
                let mut vec = vec![];
                loop {
                    let c = it.peek().unwrap();
                    match **c as char {
                        STRUCT_SIG_END_CHAR | DICT_ENTRY_SIG_END_CHAR => break,
                        _ => vec.push(iter_to_rust_type(it, input, false)),
                    }
                }
                if dict {
                    vec.join(", ")
                } else if vec.len() > 1 {
                    format!("{}({})", if as_ref { "&" } else { "" }, vec.join(", "))
                } else {
                    vec[0].to_string()
                }
            }
            _ => unimplemented!(),
        }
    }

    let mut it = ty.as_bytes().iter().peekable();
    iter_to_rust_type(&mut it, input, as_ref)
}
pub fn to_wasm_type(dbus_type: &str) -> String {
	let orig_wasm_type = _to_wasm_type(dbus_type, false, true);
	// NOTE: There is a reasonable chance that this is the case, but is not gurenteed.
	orig_wasm_type.replace("(string, string)", "accessible")
}


pub fn generate_structs(_iface: &Interface, signal: &Signal) -> String {
	let tys = signal.args()
		.iter()
		.filter_map(|arg| {
			Some(format!("\t{}: {}", arg.name()?.to_case(Case::Kebab), to_wasm_type(arg.ty())))
		})
		.collect::<Vec<String>>()
		.join(",\n");
	format!("record {} {{\n\titem: accessible,\n{},\n}}", struct_name(signal), tys)
}

pub fn structs(nodes: &[Node]) -> String {
	nodes.iter()
		.map(|node| for_signals(node, generate_structs))
		.collect::<Vec<String>>()
		.join("\n")
}

const ACCESSIBLE_SIG: &str = r#"
record accessible {
	item: string,
	name: string,
}"#;
const ZVARIANT_VALUE_WASM_SIG: &str = r#"
variant zbus-value {
	U8(u8),
	Bool(bool),
	I16(s16),
	U16(u16),
	I32(s32),
	U32(u32),
	I64(s64),
	U64(u64),
	F64(float64),
	Str(string),
	Signature(string),
	ObjectPath(string),
	Fd(u32),
	// none of these values are valid in WASM
	// Value(Box<Value<'a>>),
	// Array(Array<'a>),
	// Dict(Dict<'a, 'a>),
	// Structure(Structure<'a>),
	// Maybe(Maybe<'a>),
	// this will be the value if the zvariant::Value is one of the above types
	Invalid,
}
"#;

pub fn main() {
		let nodes = vec![
			get_root_node_from_xml("/home/tait/Documents/atspi/xml/Event.xml"),
			get_root_node_from_xml("/home/tait/Documents/atspi/xml/Cache.xml"),
			get_root_node_from_xml("/home/tait/Documents/atspi/xml/Registry.xml"),
			get_root_node_from_xml("/home/tait/Documents/atspi/xml/Socket.xml"),
			get_root_node_from_xml("/home/tait/Documents/atspi/xml/DeviceEventListener.xml"),
		];
		println!("{}", ACCESSIBLE_SIG);
		println!("{}", ZVARIANT_VALUE_WASM_SIG);
		println!("{}", structs(&nodes));
		println!("{}", event_type_flags(&nodes));
}
