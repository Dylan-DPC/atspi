#![deny(
	unsafe_code,
	clippy::all,
	clippy::pedantic,
)]

use serde::{Deserialize, Serialize};
use serde_xml_rs::{from_reader, from_str, to_writer, Error};
use std::{
    io::{Read, Write},
    result::Result,
};

// note: serde-xml-rs doesn't handle nicely interleaved elements, so we have to use enums:
// https://github.com/RReverser/serde-xml-rs/issues/55

macro_rules! get_vec {
    ($vec:expr, $kind:path) => {
        $vec.iter()
            .filter_map(|e| if let $kind(m) = e { Some(m) } else { None })
            .collect()
    };
}

macro_rules! get_doc {
    ($vec:expr, $kind:path) => {
        $vec.iter()
            .filter_map(|e| if let $kind(m) = e { Some(m.to_owned()) } else { None })
            .collect::<Vec<Doc>>()
            .get(0)
            .cloned()
    };
}

pub fn for_signals<F>(node: &Node, func: F) -> String 
	where F: Fn(&Interface, &Signal) -> String {
	node.interfaces()
		.iter()
		.map(|iface| iface.signals()
			.iter()
			.map(|signal| func(iface, signal))
			.collect::<Vec<String>>()
			.join("\n")
		)
		.collect::<Vec<String>>()
		.join("\n")
}

pub fn for_interfaces<F>(node: &Node, func: F) -> String 
	where F: Fn(&Interface) -> String {
	node.interfaces()
		.iter()
		.map(|iface| func(iface))
		.collect::<Vec<String>>()
		.join("\n")
}

pub fn for_signal_args<T, F>(signal: &Signal, func: F) -> Vec<T>
	where F: Fn(&Arg) -> T {
	signal.args()
		.iter()
		.map(|arg| func(arg))
		.collect::<Vec<T>>()
}

pub fn for_interface_signals<T, F>(iface: &Interface, func: F) -> Vec<T>
	where F: Fn(&Interface, &Signal) -> T {
	iface.signals()
		.iter()
		.map(|signal| func(iface, signal))
		.collect::<Vec<T>>()
}

/// Annotations are generic key/value pairs of metadata.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Annotation {
    name: String,
    value: String,
}

impl Annotation {
    /// Return the annotation name/key.
		#[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the annotation value.
		#[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    In,
    Out,
}

/// An argument
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Arg {
    name: Option<String>,
    r#type: String,
    direction: Option<Direction>,
    #[serde(rename = "annotation", default)]
    annotations: Vec<Annotation>,
}

impl Arg {
    /// Return the argument name, if any.
		#[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Return the argument type.
		#[must_use]
    pub fn ty(&self) -> &str {
        &self.r#type
    }

    /// Return the signature type.
		#[must_use]
    pub fn signature(&self) -> zbus::zvariant::Signature<'static> {
        zbus::zvariant::Signature::try_from(self.ty())
            .expect("Could not create DBus signature from {self.type}")
            .to_owned()
    }

    /// Return the argument direction (should be "in" or "out"), if any.
		#[must_use]
    pub fn direction(&self) -> Option<Direction> {
        self.direction
    }

    /// Return the associated annotations.
		#[must_use]
    pub fn annotations(&self) -> Vec<&Annotation> {
        self.annotations.iter().collect()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
enum MethodElement {
    Arg(Arg),
    Annotation(Annotation),
}

/// A method
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Method {
    name: String,

    #[serde(rename = "$value", default)]
    elems: Vec<MethodElement>,
}

impl Method {
    /// Return the method name.
		#[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the method arguments.
		#[must_use]
    pub fn args(&self) -> Vec<&Arg> {
        get_vec!(self.elems, MethodElement::Arg)
    }

    /// Return the method annotations.
		#[must_use]
    pub fn annotations(&self) -> Vec<&Annotation> {
        get_vec!(self.elems, MethodElement::Annotation)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
enum SignalElement {
    Arg(Arg),
    Annotation(Annotation),
}

/// A signal
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Signal {
    name: String,

    #[serde(rename = "$value", default)]
    elems: Vec<SignalElement>,
}

impl Signal {
    /// Return the signal name.
		#[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the signal arguments.
		#[must_use]
    pub fn args(&self) -> Vec<&Arg> {
        get_vec!(self.elems, SignalElement::Arg)
    }

    /// Return the signal annotations.
		#[must_use]
    pub fn annotations(&self) -> Vec<&Annotation> {
        get_vec!(self.elems, SignalElement::Annotation)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Access {
    Read,
    Write,
    ReadWrite,
}

/// A property
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Property {
    name: String,
    r#type: String,
    access: Access,

    #[serde(rename = "annotation", default)]
    annotations: Vec<Annotation>,
}

impl Property {
    /// Returns the property name.
		#[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the property type.
		#[must_use]
    pub fn ty(&self) -> &str {
        &self.r#type
    }

    /// Return the signature type.
		#[must_use]
    pub fn signature(&self) -> zbus::zvariant::Signature<'static> {
        zbus::zvariant::Signature::try_from(self.ty())
            .expect("Could not create DBus signature from {self.type}")
            .to_owned()
    }

    /// Returns the property access flags (should be "read", "write" or "readwrite").
		#[must_use]
    pub fn access(&self) -> Access {
        self.access
    }

    /// Return the associated annotations.
		#[must_use]
    pub fn annotations(&self) -> Vec<&Annotation> {
        self.annotations.iter().collect()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
enum InterfaceElement {
    Method(Method),
    Signal(Signal),
    Property(Property),
    Annotation(Annotation),
}

/// An interface
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Interface {
    name: String,

    #[serde(rename = "$value", default)]
    elems: Vec<InterfaceElement>,
}

impl Interface {
    /// Returns the interface name.
		#[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the interface methods.
		#[must_use]
    pub fn methods(&self) -> Vec<&Method> {
        get_vec!(self.elems, InterfaceElement::Method)
    }

    /// Returns the interface signals.
		#[must_use]
    pub fn signals(&self) -> Vec<&Signal> {
        get_vec!(self.elems, InterfaceElement::Signal)
    }

    /// Returns the interface properties.
		#[must_use]
    pub fn properties(&self) -> Vec<&Property> {
        get_vec!(self.elems, InterfaceElement::Property)
    }

    /// Return the associated annotations.
		#[must_use]
    pub fn annotations(&self) -> Vec<&Annotation> {
        get_vec!(self.elems, InterfaceElement::Annotation)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub struct Doc {
    #[serde(rename = "$value")]
    pub data: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
enum NodeElement {
    Node(Node),
    Interface(Interface),
    Annotation(Annotation),
    Doc(Doc),
}

/// An introspection tree node (typically the root of the XML document).
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Node {
    name: Option<String>,

    #[serde(rename = "$value", default)]
    elems: Vec<NodeElement>,
}

impl Node {
    /// Parse the introspection XML document from reader.
		/// # Errors
		/// Fails if there is failure to read the file, or deserialization files.
    pub fn from_reader<R: Read>(reader: R) -> Result<Node, Error> {
        from_reader(reader)
    }

    /// Write the XML document to writer.
		/// # Errors
		/// Fails if there is failure to write the file out to disk.
    pub fn to_writer<W: Write>(&self, writer: W) -> Result<(), Error> {
        to_writer(writer, &self)
    }

    /// Returns the node name, if any.
		#[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Return the node documentation, if any.
		#[must_use]
    pub fn doc(&self) -> Option<Doc> {
        get_doc!(self.elems, NodeElement::Doc)
    }

    /// Returns the children nodes.
		#[must_use]
    pub fn nodes(&self) -> Vec<&Node> {
        get_vec!(self.elems, NodeElement::Node)
    }

    /// Returns the interfaces on this node.
		#[must_use]
    pub fn interfaces(&self) -> Vec<&Interface> {
        get_vec!(self.elems, NodeElement::Interface)
    }

    /// Return the associated annotations.
		#[must_use]
    pub fn annotations(&self) -> Vec<&Annotation> {
        get_vec!(self.elems, NodeElement::Annotation)
    }
}

impl std::str::FromStr for Node {
    type Err = Error;

    /// Parse the introspection XML document from `s`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, str::FromStr};
    use test_log::test;

    use super::Node;

    static EXAMPLE: &str = r##"
<!DOCTYPE node PUBLIC "-//freedesktop//DTD D-BUS Object Introspection 1.0//EN"
  "http://www.freedesktop.org/standards/dbus/1.0/introspect.dtd">
 <node name="/com/example/sample_object0">
   <node name="first"/>
   <interface name="com.example.SampleInterface0">
     <method name="Frobate">
       <arg name="foo" type="i" direction="in"/>
       <arg name="bar" type="s" direction="out"/>
       <arg name="baz" type="a{us}" direction="out"/>
       <annotation name="org.freedesktop.DBus.Deprecated" value="true"/>
     </method>
     <method name="Bazify">
       <arg name="bar" type="(iiu)" direction="in"/>
       <arg name="bar" type="v" direction="out"/>
     </method>
     <method name="Mogrify">
       <arg name="bar" type="(iiav)" direction="in"/>
     </method>
     <signal name="Changed">
       <arg name="new_value" type="b"/>
     </signal>
     <property name="Bar" type="y" access="readwrite"/>
   </interface>
   <node name="child_of_sample_object"/>
   <node name="another_child_of_sample_object"/>
</node>
"##;

    #[test]
    fn serde() -> Result<(), Box<dyn Error>> {
        let node = Node::from_reader(EXAMPLE.as_bytes())?;
        assert_eq!(node.interfaces().len(), 1);
        assert_eq!(node.nodes().len(), 3);

        let node_str = Node::from_str(EXAMPLE)?;
        assert_eq!(node_str.interfaces().len(), 1);
        assert_eq!(node_str.nodes().len(), 3);

        // TODO: Fails at the moment, this seems fresh & related:
        // https://github.com/RReverser/serde-xml-rs/pull/129
        //let mut writer = Vec::with_capacity(128);
        //node.to_writer(&mut writer).unwrap();
        Ok(())
    }
}
