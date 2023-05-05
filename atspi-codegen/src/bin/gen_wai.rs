use std::{
    vec,
};

use atspi_codegen::*;
use convert_case::{Case, Casing};
use zvariant::*;

const AUTO_IMPL_HELP: [(&'static str, &'static str, &'static str, &'static str); 6] = [
	("org.a11y.atspi.Event.Object", "PropertyChange", "value", "string"),
	("org.a11y.atspi.Event.Object", "ChildrenChanged", "child", "accessible"),
	("org.a11y.atspi.Event.Object", "ActiveDescendantChanged", "child", "accessible"),
	("org.a11y.atspi.Event.Object", "TextChanged", "text", "string"),
	("org.a11y.atspi.Event.Cache", "AccessibleRemoved", "nodeRemoved", "accessible"),
	("org.a11y.atspi.Event.Cache", "AccessibleAdd", "nodeAdded", "cache-item"),
];

fn auto_impl_type_override(iface: &Interface, signal: &Signal, field: &Arg) -> Option<&'static str> {
	for (m_iface, m_member, m_field_name, m_type) in AUTO_IMPL_HELP {
		if (m_iface, m_member, m_field_name) == (iface.name(), signal.name(), field.name()?) {
			return Some(m_type);
		}
	}
	None
}

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
pub fn struct_name(iface: &Interface, signal: &Signal) -> String {
	format!("{}-{}-{}", iface_name(iface).to_lowercase(), signal.name().to_case(Case::Kebab), "event")
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

pub fn event_enum_variant(iface: &Interface) -> String {
	let enum_name = format!("{}Events", iface_name(iface)).to_case(Case::Kebab);
	format!("\t{}({}),", iface_name(iface).to_case(Case::Kebab), enum_name)
}

pub fn enum_variant(iface: &Interface, signal: &Signal) -> String {
		format!("\t{}({}),", signal.name().to_case(Case::Kebab), struct_name(iface, signal))
}

pub fn enum_container(iface: &Interface) -> String {
	let enum_name = format!("{}Events", iface_name(iface)).to_case(Case::Kebab);
	let variants = iface.signals()
		.iter()
		.map(|signal| enum_variant(iface, signal))
		.collect::<Vec<String>>()
		.join("\n");
	format!(
r#"variant {enum_name} {{
{variants}
}}"#)
}

pub fn containers(node: &Node) -> String {
	node.interfaces()
		.iter()
		.map(|iface| enum_container(iface))
		.collect::<Vec<String>>()
		.join("\n")
}

pub fn enum_containers(nodes: &[Node]) -> String {
	nodes.iter()
		.map(containers)
		.collect::<Vec<String>>()
		.join("\n")
}
pub fn container_variants(node: &Node) -> String {
	node.interfaces()
		.iter()
		.map(|iface| event_enum_variant(iface))
		.collect::<Vec<String>>()
		.join("\n")
}
pub fn event_enum(nodes: &[Node]) -> String {
	let variants = nodes.iter()
		.map(container_variants)
		.collect::<Vec<String>>()
		.join("\n");
	format!(
r#"variant event {{
{variants}
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
                    format!("{}tuple<{}>", if as_ref { "&" } else { "" }, vec.join(", "))
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
pub fn to_wasm_type(iface: &Interface, signal: &Signal, arg: &Arg) -> String {
  if let Some(type_override) = auto_impl_type_override(iface, signal, arg) {
		return type_override.to_string();
	}
	let orig_wasm_type = _to_wasm_type(arg.ty(), false, false);
	// NOTE: There is a reasonable chance that this is the case, but is not gurenteed.
	orig_wasm_type.replace("(string, string)", "accessible")
}


pub fn generate_structs(iface: &Interface, signal: &Signal) -> String {
	let tys = signal.args()
		.iter()
		.filter_map(|arg| {
			if arg.name()? == "properties" { None } else {
				Some(format!("\t{}: {}", arg.name()?.to_case(Case::Kebab), to_wasm_type(iface, signal, arg)))
			}
		})
		.collect::<Vec<String>>()
		.join(",\n");
	format!("record {} {{\n\titem: accessible,\n{}}}", struct_name(iface, signal), tys)
}

pub fn structs(nodes: &[Node]) -> String {
	nodes.iter()
		.map(|node| for_signals(node, generate_structs))
		.collect::<Vec<String>>()
		.join("\n")
}

pub fn main() {
		let nodes = vec![
			get_root_node_from_xml("/home/tait/Documents/atspi/xml/Event.xml"),
			get_root_node_from_xml("/home/tait/Documents/atspi/xml/Cache.xml"),
			get_root_node_from_xml("/home/tait/Documents/atspi/xml/Registry.xml"),
			get_root_node_from_xml("/home/tait/Documents/atspi/xml/Socket.xml"),
			get_root_node_from_xml("/home/tait/Documents/atspi/xml/DeviceEventListener.xml"),
		];
		println!("use * from initial-types");
		println!("{}", structs(&nodes));
		println!("{}", event_type_flags(&nodes));
		println!("{}", enum_containers(&nodes));
		println!("{}", event_enum(&nodes));
}
