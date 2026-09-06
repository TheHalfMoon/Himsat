//! Foundational Himsat-owned identity, evidence, and event contracts.
//!
//! This crate intentionally defines logical values only. It does not allocate IDs,
//! persist events, choose a wire format, provide clocks, or perform legal provenance
//! checks.

use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// Error returned when a strongly typed Himsat identity cannot be parsed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdentityParseError {
    /// Canonical identities contain exactly 32 hexadecimal digits.
    InvalidLength,
    /// The supplied 32-character value contains a non-hexadecimal digit.
    InvalidHex,
}

impl fmt::Display for IdentityParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => formatter.write_str("identity must contain exactly 32 hex digits"),
            Self::InvalidHex => formatter.write_str("identity contains a non-hexadecimal digit"),
        }
    }
}

impl Error for IdentityParseError {}

macro_rules! define_identity {
    ($name:ident) => {
        #[doc = concat!("Strongly typed 128-bit `", stringify!($name), "` value.")]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(u128);

        impl $name {
            /// Constructs the identity from its logical 128-bit value.
            pub const fn new(value: u128) -> Self {
                Self(value)
            }

            /// Returns the underlying logical value.
            pub const fn get(self) -> u128 {
                self.0
            }
        }

        impl From<u128> for $name {
            fn from(value: u128) -> Self {
                Self::new(value)
            }
        }

        impl From<$name> for u128 {
            fn from(value: $name) -> Self {
                value.get()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "{:032x}", self.0)
            }
        }

        impl FromStr for $name {
            type Err = IdentityParseError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                if value.len() != 32 {
                    return Err(IdentityParseError::InvalidLength);
                }

                u128::from_str_radix(value, 16)
                    .map(Self::new)
                    .map_err(|_| IdentityParseError::InvalidHex)
            }
        }
    };
}

define_identity!(SessionId);
define_identity!(SourceId);
define_identity!(EventId);
define_identity!(ArtifactId);

/// Version of the logical core schema.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SchemaVersion {
    /// Breaking compatibility generation.
    pub major: u16,
    /// Backward-readable additive generation within the same major version.
    pub minor: u16,
}

impl SchemaVersion {
    /// Constructs a schema version.
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    /// Returns whether this reader version explicitly accepts an encoded version.
    ///
    /// A reader accepts only the same major version and encoded minor versions no
    /// newer than the reader itself. This predicate performs no migration.
    pub const fn can_read(self, encoded: Self) -> bool {
        self.major == encoded.major && encoded.minor <= self.minor
    }
}

/// Current logical schema version for foundational Himsat events.
pub const CORE_SCHEMA_VERSION: SchemaVersion = SchemaVersion::new(1, 0);

/// Per-session event ordinal. Allocation and global ordering are out of scope.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EventSequence(u64);

impl EventSequence {
    /// Constructs a sequence value.
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the ordinal value.
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Minimal logical session record.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Session {
    /// Session identity.
    pub id: SessionId,
}

impl Session {
    /// Constructs a session record.
    pub const fn new(id: SessionId) -> Self {
        Self { id }
    }
}

/// Minimal logical source record owned by a session.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Source {
    /// Source identity.
    pub id: SourceId,
    /// Owning session identity.
    pub session_id: SessionId,
}

impl Source {
    /// Constructs a source record.
    pub const fn new(id: SourceId, session_id: SessionId) -> Self {
        Self { id, session_id }
    }
}

/// Himsat-owned semantic artifact category without storage semantics.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ArtifactKind {
    /// Audio evidence or an audio-derived artifact.
    Audio,
    /// Transcript material.
    Transcript,
    /// A document artifact.
    Document,
    /// A captured screen frame.
    ScreenFrame,
    /// A note entered directly by a user.
    ManualNote,
    /// Imported media whose original type is not represented by another category.
    ImportedMedia,
    /// Derived material that does not fit a more specific foundational category.
    Derived,
}

/// Minimal logical artifact record.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Artifact {
    /// Artifact identity.
    pub id: ArtifactId,
    /// Owning session identity.
    pub session_id: SessionId,
    /// Optional source identity when the artifact belongs to one capture/import source.
    pub source_id: Option<SourceId>,
    /// Semantic artifact category.
    pub kind: ArtifactKind,
}

impl Artifact {
    /// Constructs an artifact record.
    pub const fn new(
        id: ArtifactId,
        session_id: SessionId,
        source_id: Option<SourceId>,
        kind: ArtifactKind,
    ) -> Self {
        Self {
            id,
            session_id,
            source_id,
            kind,
        }
    }
}

/// Error returned when an ordered range is reversed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RangeError;

impl fmt::Display for RangeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("range end cannot precede range start")
    }
}

impl Error for RangeError {}

/// Inclusive microsecond range within an audio timeline.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TimeRangeMicros {
    start: u64,
    end: u64,
}

impl TimeRangeMicros {
    /// Constructs a checked inclusive range.
    pub const fn new(start: u64, end: u64) -> Result<Self, RangeError> {
        if end < start {
            return Err(RangeError);
        }
        Ok(Self { start, end })
    }

    /// Returns the inclusive start position in microseconds.
    pub const fn start(self) -> u64 {
        self.start
    }

    /// Returns the inclusive end position in microseconds.
    pub const fn end(self) -> u64 {
        self.end
    }
}

/// Inclusive ordinal range for transcript or other ordered units.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OrdinalRange {
    start: u64,
    end: u64,
}

impl OrdinalRange {
    /// Constructs a checked inclusive ordinal range.
    pub const fn new(start: u64, end: u64) -> Result<Self, RangeError> {
        if end < start {
            return Err(RangeError);
        }
        Ok(Self { start, end })
    }

    /// Returns the inclusive first ordinal.
    pub const fn start(self) -> u64 {
        self.start
    }

    /// Returns the inclusive last ordinal.
    pub const fn end(self) -> u64 {
        self.end
    }
}

/// Error returned when a normalized region is not inside the unit square.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegionError {
    /// At least one coordinate or extent exceeds one million millionths.
    ComponentOutOfBounds,
    /// Origin plus extent leaves the normalized unit square.
    OutsideUnitSquare,
}

impl fmt::Display for RegionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ComponentOutOfBounds => {
                formatter.write_str("normalized region component exceeds 1,000,000")
            }
            Self::OutsideUnitSquare => formatter.write_str("normalized region leaves the unit square"),
        }
    }
}

impl Error for RegionError {}

/// Integer-millionth normalized region with origin and extent inside a unit square.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NormalizedRegion {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl NormalizedRegion {
    /// One normalized unit expressed in integer millionths.
    pub const SCALE: u32 = 1_000_000;

    /// Constructs a checked normalized region.
    pub const fn new(x: u32, y: u32, width: u32, height: u32) -> Result<Self, RegionError> {
        if x > Self::SCALE
            || y > Self::SCALE
            || width > Self::SCALE
            || height > Self::SCALE
        {
            return Err(RegionError::ComponentOutOfBounds);
        }

        let Some(right) = x.checked_add(width) else {
            return Err(RegionError::OutsideUnitSquare);
        };
        let Some(bottom) = y.checked_add(height) else {
            return Err(RegionError::OutsideUnitSquare);
        };

        if right > Self::SCALE || bottom > Self::SCALE {
            return Err(RegionError::OutsideUnitSquare);
        }

        Ok(Self {
            x,
            y,
            width,
            height,
        })
    }

    /// Horizontal origin in integer millionths.
    pub const fn x(self) -> u32 {
        self.x
    }

    /// Vertical origin in integer millionths.
    pub const fn y(self) -> u32 {
        self.y
    }

    /// Width in integer millionths.
    pub const fn width(self) -> u32 {
        self.width
    }

    /// Height in integer millionths.
    pub const fn height(self) -> u32 {
        self.height
    }
}

/// Typed locator for evidence. It carries addressability, not semantic sufficiency.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EvidenceRef {
    /// The complete artifact is the evidence locator.
    WholeArtifact { artifact_id: ArtifactId },
    /// Inclusive audio timeline range, optionally tied to one source.
    AudioRange {
        artifact_id: ArtifactId,
        source_id: Option<SourceId>,
        range: TimeRangeMicros,
    },
    /// Inclusive transcript-unit ordinal range.
    TranscriptRange {
        artifact_id: ArtifactId,
        range: OrdinalRange,
    },
    /// Zero-based document page with an optional normalized region.
    DocumentPage {
        artifact_id: ArtifactId,
        page_index: u32,
        region: Option<NormalizedRegion>,
    },
    /// Zero-based screen-frame ordinal with an optional normalized region.
    ScreenFrame {
        artifact_id: ArtifactId,
        frame_ordinal: u64,
        region: Option<NormalizedRegion>,
    },
    /// A user-authored note artifact.
    ManualNote { artifact_id: ArtifactId },
}

/// Error returned when a required producer identity field is empty or whitespace-only.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProducerIdentityError {
    field: &'static str,
}

impl ProducerIdentityError {
    /// Returns the invalid field name.
    pub const fn field(self) -> &'static str {
        self.field
    }
}

impl fmt::Display for ProducerIdentityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "producer identity field '{}' cannot be empty", self.field)
    }
}

impl Error for ProducerIdentityError {}

fn validate_identity_field(value: &str, field: &'static str) -> Result<(), ProducerIdentityError> {
    if value.trim().is_empty() {
        Err(ProducerIdentityError { field })
    } else {
        Ok(())
    }
}

/// Operational identity for a model implementation that produced an artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelIdentity {
    name: String,
    version: String,
    revision: String,
    digest: Option<[u8; 32]>,
}

impl ModelIdentity {
    /// Constructs a validated model identity.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        revision: impl Into<String>,
        digest: Option<[u8; 32]>,
    ) -> Result<Self, ProducerIdentityError> {
        let name = name.into();
        let version = version.into();
        let revision = revision.into();
        validate_identity_field(&name, "name")?;
        validate_identity_field(&version, "version")?;
        validate_identity_field(&revision, "revision")?;
        Ok(Self {
            name,
            version,
            revision,
            digest,
        })
    }

    /// Producer name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Producer version.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Producer revision identity.
    pub fn revision(&self) -> &str {
        &self.revision
    }

    /// Optional exact 32-byte digest identity.
    pub const fn digest(&self) -> Option<[u8; 32]> {
        self.digest
    }
}

/// Operational identity for a parser implementation that produced an artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParserIdentity {
    name: String,
    version: String,
    revision: String,
    digest: Option<[u8; 32]>,
}

impl ParserIdentity {
    /// Constructs a validated parser identity.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        revision: impl Into<String>,
        digest: Option<[u8; 32]>,
    ) -> Result<Self, ProducerIdentityError> {
        let name = name.into();
        let version = version.into();
        let revision = revision.into();
        validate_identity_field(&name, "name")?;
        validate_identity_field(&version, "version")?;
        validate_identity_field(&revision, "revision")?;
        Ok(Self {
            name,
            version,
            revision,
            digest,
        })
    }

    /// Producer name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Producer version.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Producer revision identity.
    pub fn revision(&self) -> &str {
        &self.revision
    }

    /// Optional exact 32-byte digest identity.
    pub const fn digest(&self) -> Option<[u8; 32]> {
        self.digest
    }
}

/// Foundational event relationships only. Domain event payloads belong to later specs.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CoreEvent {
    /// Establishes a session identity.
    SessionCreated,
    /// Registers a source under the envelope's session.
    SourceRegistered { source_id: SourceId },
    /// Registers an artifact under the envelope's session.
    ArtifactRegistered {
        artifact_id: ArtifactId,
        source_id: Option<SourceId>,
        kind: ArtifactKind,
    },
    /// Links typed evidence under the envelope's session.
    EvidenceLinked { evidence: EvidenceRef },
}

/// Minimal logical, versioned event envelope.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EventEnvelope {
    schema_version: SchemaVersion,
    event_id: EventId,
    session_id: SessionId,
    sequence: EventSequence,
    event: CoreEvent,
}

impl EventEnvelope {
    /// Constructs a current-schema `SessionCreated` event from the session record.
    pub const fn session_created(
        event_id: EventId,
        sequence: EventSequence,
        session: Session,
    ) -> Self {
        Self {
            schema_version: CORE_SCHEMA_VERSION,
            event_id,
            session_id: session.id,
            sequence,
            event: CoreEvent::SessionCreated,
        }
    }

    /// Constructs a current-schema `SourceRegistered` event from the source record.
    pub const fn source_registered(
        event_id: EventId,
        sequence: EventSequence,
        source: Source,
    ) -> Self {
        Self {
            schema_version: CORE_SCHEMA_VERSION,
            event_id,
            session_id: source.session_id,
            sequence,
            event: CoreEvent::SourceRegistered {
                source_id: source.id,
            },
        }
    }

    /// Constructs a current-schema `ArtifactRegistered` event from the artifact record.
    pub const fn artifact_registered(
        event_id: EventId,
        sequence: EventSequence,
        artifact: Artifact,
    ) -> Self {
        Self {
            schema_version: CORE_SCHEMA_VERSION,
            event_id,
            session_id: artifact.session_id,
            sequence,
            event: CoreEvent::ArtifactRegistered {
                artifact_id: artifact.id,
                source_id: artifact.source_id,
                kind: artifact.kind,
            },
        }
    }

    /// Constructs a current-schema evidence-link event for an explicitly supplied session.
    pub const fn evidence_linked(
        event_id: EventId,
        session_id: SessionId,
        sequence: EventSequence,
        evidence: EvidenceRef,
    ) -> Self {
        Self {
            schema_version: CORE_SCHEMA_VERSION,
            event_id,
            session_id,
            sequence,
            event: CoreEvent::EvidenceLinked { evidence },
        }
    }

    /// Logical schema version carried by this envelope.
    pub const fn schema_version(self) -> SchemaVersion {
        self.schema_version
    }

    /// Event identity.
    pub const fn event_id(self) -> EventId {
        self.event_id
    }

    /// Owning session identity.
    pub const fn session_id(self) -> SessionId {
        self.session_id
    }

    /// Per-session event ordinal.
    pub const fn sequence(self) -> EventSequence {
        self.sequence
    }

    /// Foundational event payload.
    pub const fn event(self) -> CoreEvent {
        self.event
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEX: &str = "0123456789abcdef0123456789abcdef";
    const VALUE: u128 = 0x0123_4567_89ab_cdef_0123_4567_89ab_cdef;

    fn assert_id_round_trip<T>()
    where
        T: FromStr<Err = IdentityParseError> + fmt::Display + Into<u128> + Copy,
    {
        let parsed = HEX.parse::<T>().expect("canonical identity should parse");
        assert_eq!(parsed.to_string(), HEX);
        assert_eq!(Into::<u128>::into(parsed), VALUE);
    }

    #[test]
    fn identity_classes_round_trip_canonically() {
        assert_id_round_trip::<SessionId>();
        assert_id_round_trip::<SourceId>();
        assert_id_round_trip::<EventId>();
        assert_id_round_trip::<ArtifactId>();
    }

    #[test]
    fn identity_display_is_zero_padded_lowercase_hex() {
        assert_eq!(SessionId::new(0xAB).to_string(), "000000000000000000000000000000ab");
    }

    #[test]
    fn identity_parse_rejects_wrong_length() {
        assert_eq!("0".parse::<SessionId>(), Err(IdentityParseError::InvalidLength));
        assert_eq!(
            "000000000000000000000000000000000".parse::<SessionId>(),
            Err(IdentityParseError::InvalidLength)
        );
    }

    #[test]
    fn identity_parse_rejects_non_hexadecimal_input() {
        assert_eq!(
            "gggggggggggggggggggggggggggggggg".parse::<EventId>(),
            Err(IdentityParseError::InvalidHex)
        );
    }

    #[test]
    fn schema_reader_accepts_only_same_major_and_known_minor() {
        let reader = SchemaVersion::new(1, 2);
        assert!(reader.can_read(SchemaVersion::new(1, 0)));
        assert!(reader.can_read(SchemaVersion::new(1, 2)));
        assert!(!reader.can_read(SchemaVersion::new(1, 3)));
        assert!(!reader.can_read(SchemaVersion::new(2, 0)));
    }

    #[test]
    fn time_range_rejects_reversal_and_accepts_equal_boundary() {
        assert_eq!(TimeRangeMicros::new(9, 8), Err(RangeError));
        let range = TimeRangeMicros::new(9, 9).expect("equal endpoints are valid");
        assert_eq!((range.start(), range.end()), (9, 9));
    }

    #[test]
    fn ordinal_range_rejects_reversal() {
        assert_eq!(OrdinalRange::new(4, 3), Err(RangeError));
        assert_eq!(
            OrdinalRange::new(3, 4).expect("forward range should be valid").end(),
            4
        );
    }

    #[test]
    fn normalized_region_accepts_unit_square_boundaries() {
        let region = NormalizedRegion::new(250_000, 100_000, 750_000, 900_000)
            .expect("region ending on unit-square boundary should be valid");
        assert_eq!(region.x(), 250_000);
        assert_eq!(region.y(), 100_000);
        assert_eq!(region.width(), 750_000);
        assert_eq!(region.height(), 900_000);
    }

    #[test]
    fn normalized_region_rejects_component_out_of_bounds() {
        assert_eq!(
            NormalizedRegion::new(1_000_001, 0, 0, 0),
            Err(RegionError::ComponentOutOfBounds)
        );
    }

    #[test]
    fn normalized_region_rejects_origin_plus_extent_outside_square() {
        assert_eq!(
            NormalizedRegion::new(900_000, 0, 100_001, 0),
            Err(RegionError::OutsideUnitSquare)
        );
        assert_eq!(
            NormalizedRegion::new(0, 900_000, 0, 100_001),
            Err(RegionError::OutsideUnitSquare)
        );
    }

    #[test]
    fn evidence_ref_preserves_locator_kind_and_values() {
        let artifact_id = ArtifactId::new(11);
        let source_id = SourceId::new(12);
        let range = TimeRangeMicros::new(100, 200).expect("valid range");
        let evidence = EvidenceRef::AudioRange {
            artifact_id,
            source_id: Some(source_id),
            range,
        };
        assert_eq!(
            evidence,
            EvidenceRef::AudioRange {
                artifact_id,
                source_id: Some(source_id),
                range
            }
        );
    }

    #[test]
    fn model_identity_rejects_empty_required_fields() {
        let error = ModelIdentity::new("model", " ", "rev", None)
            .expect_err("whitespace-only version must fail");
        assert_eq!(error.field(), "version");
    }

    #[test]
    fn parser_identity_rejects_empty_required_fields() {
        let error = ParserIdentity::new("", "1", "rev", None)
            .expect_err("empty name must fail");
        assert_eq!(error.field(), "name");
    }

    #[test]
    fn producer_identity_preserves_optional_digest() {
        let digest = [7_u8; 32];
        let identity = ModelIdentity::new("engine", "1", "abc", Some(digest))
            .expect("valid producer identity");
        assert_eq!(identity.name(), "engine");
        assert_eq!(identity.version(), "1");
        assert_eq!(identity.revision(), "abc");
        assert_eq!(identity.digest(), Some(digest));
    }

    #[test]
    fn source_event_inherits_session_relationship_from_source_record() {
        let session_id = SessionId::new(1);
        let source = Source::new(SourceId::new(2), session_id);
        let envelope = EventEnvelope::source_registered(
            EventId::new(3),
            EventSequence::new(4),
            source,
        );
        assert_eq!(envelope.schema_version(), CORE_SCHEMA_VERSION);
        assert_eq!(envelope.session_id(), session_id);
        assert_eq!(
            envelope.event(),
            CoreEvent::SourceRegistered {
                source_id: source.id
            }
        );
    }

    #[test]
    fn artifact_event_inherits_artifact_relationships() {
        let session_id = SessionId::new(10);
        let source_id = SourceId::new(11);
        let artifact = Artifact::new(
            ArtifactId::new(12),
            session_id,
            Some(source_id),
            ArtifactKind::Transcript,
        );
        let envelope = EventEnvelope::artifact_registered(
            EventId::new(13),
            EventSequence::new(14),
            artifact,
        );
        assert_eq!(envelope.session_id(), session_id);
        assert_eq!(
            envelope.event(),
            CoreEvent::ArtifactRegistered {
                artifact_id: artifact.id,
                source_id: Some(source_id),
                kind: ArtifactKind::Transcript
            }
        );
    }

    #[test]
    fn session_created_and_evidence_linked_use_explicit_session_identity() {
        let session = Session::new(SessionId::new(21));
        let created = EventEnvelope::session_created(
            EventId::new(22),
            EventSequence::new(0),
            session,
        );
        assert_eq!(created.session_id(), session.id);
        assert_eq!(created.event(), CoreEvent::SessionCreated);

        let evidence = EvidenceRef::ManualNote {
            artifact_id: ArtifactId::new(23),
        };
        let linked = EventEnvelope::evidence_linked(
            EventId::new(24),
            session.id,
            EventSequence::new(1),
            evidence,
        );
        assert_eq!(linked.session_id(), session.id);
        assert_eq!(linked.event(), CoreEvent::EvidenceLinked { evidence });
    }
}
