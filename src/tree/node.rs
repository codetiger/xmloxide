//! Node type definitions.
//!
//! The `NodeKind` enum represents all node types in an XML document tree,
//! corresponding to libxml2's `xmlElementType`. Each variant carries the
//! node-type-specific payload (e.g., element name and attributes, text content).

use std::borrow::Cow;

use super::{cow_into_static, Attribute};

/// The kind of an XML node and its associated data.
///
/// This enum carries the payload for each node type. Navigation links
/// (parent, children, siblings) are stored in `NodeData`, not here.
#[derive(Debug, Clone)]
pub enum NodeKind<'a> {
    /// The document node — there is exactly one per `Document`.
    Document,

    /// An element node, e.g., `<div class="x">`.
    Element {
        /// The element's local name (or full `QName` before namespace resolution).
        name: Cow<'a, str>,
        /// Namespace prefix (e.g., `"svg"` in `svg:rect`), if any.
        prefix: Option<Cow<'a, str>>,
        /// Namespace URI after resolution, if any.
        namespace: Option<Cow<'a, str>>,
        /// Attributes on this element.
        attributes: Vec<Attribute<'a>>,
    },

    /// A text node containing character data.
    Text {
        /// The text content (already decoded — character references resolved).
        content: Cow<'a, str>,
    },

    /// A CDATA section, e.g., `<![CDATA[...]]>`.
    CData {
        /// The CDATA content (no escaping applied).
        content: Cow<'a, str>,
    },

    /// A comment node, e.g., `<!-- ... -->`.
    Comment {
        /// The comment text (without the `<!--` and `-->` delimiters).
        content: Cow<'a, str>,
    },

    /// A processing instruction, e.g., `<?target data?>`.
    ProcessingInstruction {
        /// The PI target (e.g., `"xml-stylesheet"`).
        target: Cow<'a, str>,
        /// The PI data, if any.
        data: Option<Cow<'a, str>>,
    },

    /// An entity reference node (e.g., `&amp;` when not expanded).
    EntityRef {
        /// The entity name (without `&` and `;`).
        name: Cow<'a, str>,
        /// The expanded value of the entity (used for `text_content()`).
        value: Option<Cow<'a, str>>,
    },

    /// A document type declaration node, e.g., `<!DOCTYPE html>`.
    ///
    /// See XML 1.0 §2.8: `[28]` doctypedecl
    DocumentType {
        /// The root element name declared in the DOCTYPE.
        name: Cow<'a, str>,
        /// The SYSTEM identifier (URI), if any.
        system_id: Option<Cow<'a, str>>,
        /// The PUBLIC identifier, if any.
        public_id: Option<Cow<'a, str>>,
        /// The serialized internal subset content (between `[` and `]`), if any.
        internal_subset: Option<Cow<'a, str>>,
    },
}

impl NodeKind<'_> {
    /// Converts all borrowed strings to owned, producing a `'static` lifetime.
    #[must_use]
    pub fn into_static(self) -> NodeKind<'static> {
        match self {
            Self::Document => NodeKind::Document,
            Self::Element {
                name,
                prefix,
                namespace,
                attributes,
            } => NodeKind::Element {
                name: cow_into_static(name),
                prefix: prefix.map(cow_into_static),
                namespace: namespace.map(cow_into_static),
                attributes: attributes.into_iter().map(Attribute::into_static).collect(),
            },
            Self::Text { content } => NodeKind::Text {
                content: cow_into_static(content),
            },
            Self::CData { content } => NodeKind::CData {
                content: cow_into_static(content),
            },
            Self::Comment { content } => NodeKind::Comment {
                content: cow_into_static(content),
            },
            Self::ProcessingInstruction { target, data } => NodeKind::ProcessingInstruction {
                target: cow_into_static(target),
                data: data.map(cow_into_static),
            },
            Self::EntityRef { name, value } => NodeKind::EntityRef {
                name: cow_into_static(name),
                value: value.map(cow_into_static),
            },
            Self::DocumentType {
                name,
                system_id,
                public_id,
                internal_subset,
            } => NodeKind::DocumentType {
                name: cow_into_static(name),
                system_id: system_id.map(cow_into_static),
                public_id: public_id.map(cow_into_static),
                internal_subset: internal_subset.map(cow_into_static),
            },
        }
    }
}
