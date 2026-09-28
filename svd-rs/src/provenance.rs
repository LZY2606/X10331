//! Provenance information for model elements.
//!
//! SVD models can be built either programmatically (with the builders) or by
//! parsing an XML document. Parsers can attach a [`Provenance`] to the elements
//! they create so that later diagnostics can point back to the exact source
//! location.
//!
//! The model itself never stores any source text: only an opaque identifier
//! produced by the parser is kept. Programmatic builders do not set any
//! provenance, which is represented by the default value.

/// Opaque identifier of a source element assigned by the frontend (e.g. the
/// XML parser).
///
/// The identifier is interpreted by the frontend which produced it; the model
/// treats it as an opaque value. `None` means that the element was created
/// without source information (for instance through a builder).
pub type SourceId = Option<usize>;

/// Origin of a model element.
///
/// The default value (`None`) describes elements without source information.
/// It intentionally uses a private inner type so that construction outside of
/// the parser-facing API is explicit.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(transparent)
)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Provenance(Option<SourceElement>);

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct SourceElement {
    id: usize,
}

impl Provenance {
    /// Create provenance referencing a source element by its identifier.
    pub fn new(id: usize) -> Self {
        Self(Some(SourceElement { id }))
    }

    /// Returns the opaque source element identifier, if any.
    pub fn source_id(self) -> SourceId {
        self.0.map(|s| s.id)
    }

    /// Returns `true` if no source information is attached.
    pub fn is_empty(self) -> bool {
        self.0.is_none()
    }
}

/// Types that can report their [`Provenance`].
pub trait WithProvenance {
    /// Returns the provenance of this element.
    fn provenance(&self) -> Provenance;
    /// Returns the opaque source identifier of this element, if any.
    fn source_id(&self) -> SourceId {
        self.provenance().source_id()
    }
}
