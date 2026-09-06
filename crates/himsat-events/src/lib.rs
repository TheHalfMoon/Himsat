//! Foundational Himsat-owned identity, evidence, and event contracts.
//!
//! Logical contracts only: no ID allocation, persistence, wire format, clocks,
//! networking, or legal provenance decisions.

use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdentityParseError {
    InvalidLength,
    InvalidHex,
}

impl fmt::Display for IdentityParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => f.write_str("identity must contain exactly 32 hex digits"),
            Self::InvalidHex => f.write_str("identity contains a non-hexadecimal digit"),
        }
    }
}

impl Error for IdentityParseError {}

macro_rules! define_identity {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(u128);

        impl $name {
            pub const fn new(value: u128) -> Self {
                Self(value)
            }

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
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{:032x}", self.0)
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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SchemaVersion {
    pub major: u16,
    pub minor: u16,
}

impl SchemaVersion {
    pub const fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    pub const fn can_read(self, encoded: Self) -> bool {
        self.major == encoded.major && encoded.minor <= self.minor
    }
}

pub const CORE_SCHEMA_VERSION: SchemaVersion = SchemaVersion::new(1, 0);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EventSequence(u64);

impl EventSequence {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Session {
    pub id: SessionId,
}

impl Session {
    pub const fn new(id: SessionId) -> Self {
        Self { id }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Source {
    pub id: SourceId,
    pub session_id: SessionId,
}

impl Source {
    pub const fn new(id: SourceId, session_id: SessionId) -> Self {
        Self { id, session_id }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ArtifactKind {
    Audio,
    Transcript,
    Document,
    ScreenFrame,
    ManualNote,
    ImportedMedia,
    Derived,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Artifact {
    pub id: ArtifactId,
    pub session_id: SessionId,
    pub source_id: Option<SourceId>,
    pub kind: ArtifactKind,
}

impl Artifact {
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RangeError;

impl fmt::Display for RangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("range end cannot precede range start")
    }
}

impl Error for RangeError {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct TimeRangeMicros {
    start: u64,
    end: u64,
}

impl TimeRangeMicros {
    pub const fn new(start: u64, end: u64) -> Result<Self, RangeError> {
        if end < start {
            return Err(RangeError);
        }
        Ok(Self { start, end })
    }

    pub const fn start(self) -> u64 {
        self.start
    }

    pub const fn end(self) -> u64 {
        self.end
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct OrdinalRange {
    start: u64,
    end: u64,
}

impl OrdinalRange {
    pub const fn new(start: u64, end: u64) -> Result<Self, RangeError> {
        if end < start {
            return Err(RangeError);
        }
        Ok(Self { start, end })
    }

    pub const fn start(self) -> u64 {
        self.start
    }

    pub const fn end(self) -> u64 {
        self.end
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegionError {
    ComponentOutOfBounds,
    OutsideUnitSquare,
}

impl fmt::Display for RegionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ComponentOutOfBounds => {
                f.write_str("normalized region component exceeds 1,000,000")
            }
            Self::OutsideUnitSquare => f.write_str("normalized region leaves the unit square"),
        }
    }
}

impl Error for RegionError {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct NormalizedRegion {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl NormalizedRegion {
    pub const SCALE: u32 = 1_000_000;

    pub const fn new(x: u32, y: u32, width: u32, height: u32) -> Result<Self, RegionError> {
        if x > Self::SCALE || y > Self::SCALE || width > Self::SCALE || height > Self::SCALE {
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

    pub const fn x(self) -> u32 {
        self.x
    }

    pub const fn y(self) -> u32 {
        self.y
    }

    pub const fn width(self) -> u32 {
        self.width
    }

    pub const fn height(self) -> u32 {
        self.height
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EvidenceRef {
    WholeArtifact {
        artifact_id: ArtifactId,
    },
    AudioRange {
        artifact_id: ArtifactId,
        source_id: Option<SourceId>,
        range: TimeRangeMicros,
    },
    TranscriptRange {
        artifact_id: ArtifactId,
        range: OrdinalRange,
    },
    DocumentPage {
        artifact_id: ArtifactId,
        page_index: u32,
        region: Option<NormalizedRegion>,
    },
    ScreenFrame {
        artifact_id: ArtifactId,
        frame_ordinal: u64,
        region: Option<NormalizedRegion>,
    },
    ManualNote {
        artifact_id: ArtifactId,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProducerIdentityError {
    field: &'static str,
}

impl ProducerIdentityError {
    pub const fn field(self) -> &'static str {
        self.field
    }
}

impl fmt::Display for ProducerIdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "producer identity field '{}' cannot be empty",
            self.field
        )
    }
}

impl Error for ProducerIdentityError {}

fn validate_field(value: &str, field: &'static str) -> Result<(), ProducerIdentityError> {
    if value.trim().is_empty() {
        Err(ProducerIdentityError { field })
    } else {
        Ok(())
    }
}

macro_rules! define_producer_identity {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name {
            name: String,
            version: String,
            revision: String,
            digest: Option<[u8; 32]>,
        }

        impl $name {
            pub fn new(
                name: impl Into<String>,
                version: impl Into<String>,
                revision: impl Into<String>,
                digest: Option<[u8; 32]>,
            ) -> Result<Self, ProducerIdentityError> {
                let name = name.into();
                let version = version.into();
                let revision = revision.into();
                validate_field(&name, "name")?;
                validate_field(&version, "version")?;
                validate_field(&revision, "revision")?;
                Ok(Self {
                    name,
                    version,
                    revision,
                    digest,
                })
            }

            pub fn name(&self) -> &str {
                &self.name
            }

            pub fn version(&self) -> &str {
                &self.version
            }

            pub fn revision(&self) -> &str {
                &self.revision
            }

            pub const fn digest(&self) -> Option<[u8; 32]> {
                self.digest
            }
        }
    };
}

define_producer_identity!(ModelIdentity);
define_producer_identity!(ParserIdentity);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CoreEvent {
    SessionCreated,
    SourceRegistered {
        source_id: SourceId,
    },
    ArtifactRegistered {
        artifact_id: ArtifactId,
        source_id: Option<SourceId>,
        kind: ArtifactKind,
    },
    EvidenceLinked {
        evidence: EvidenceRef,
    },
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EventEnvelope {
    schema_version: SchemaVersion,
    event_id: EventId,
    session_id: SessionId,
    sequence: EventSequence,
    event: CoreEvent,
}

impl EventEnvelope {
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

    pub const fn schema_version(self) -> SchemaVersion {
        self.schema_version
    }

    pub const fn event_id(self) -> EventId {
        self.event_id
    }

    pub const fn session_id(self) -> SessionId {
        self.session_id
    }

    pub const fn sequence(self) -> EventSequence {
        self.sequence
    }

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
    fn identities_round_trip_and_display_canonically() {
        assert_id_round_trip::<SessionId>();
        assert_id_round_trip::<SourceId>();
        assert_id_round_trip::<EventId>();
        assert_id_round_trip::<ArtifactId>();
        assert_eq!(
            SessionId::new(0xAB).to_string(),
            "000000000000000000000000000000ab"
        );
    }

    #[test]
    fn malformed_identities_fail() {
        assert_eq!(
            "0".parse::<SessionId>(),
            Err(IdentityParseError::InvalidLength)
        );
        assert_eq!(
            "000000000000000000000000000000000".parse::<SessionId>(),
            Err(IdentityParseError::InvalidLength)
        );
        assert_eq!(
            "gggggggggggggggggggggggggggggggg".parse::<EventId>(),
            Err(IdentityParseError::InvalidHex)
        );
    }

    #[test]
    fn schema_compatibility_is_explicit() {
        let reader = SchemaVersion::new(1, 2);
        assert!(reader.can_read(SchemaVersion::new(1, 0)));
        assert!(reader.can_read(SchemaVersion::new(1, 2)));
        assert!(!reader.can_read(SchemaVersion::new(1, 3)));
        assert!(!reader.can_read(SchemaVersion::new(2, 0)));
    }

    #[test]
    fn checked_ranges_reject_reversal() {
        assert_eq!(TimeRangeMicros::new(9, 8), Err(RangeError));
        assert_eq!(OrdinalRange::new(4, 3), Err(RangeError));
        let time = TimeRangeMicros::new(9, 9).expect("equal endpoints are valid");
        let ordinals = OrdinalRange::new(3, 4).expect("forward range is valid");
        assert_eq!((time.start(), time.end()), (9, 9));
        assert_eq!((ordinals.start(), ordinals.end()), (3, 4));
    }

    #[test]
    fn normalized_region_checks_bounds() {
        let region = NormalizedRegion::new(250_000, 100_000, 750_000, 900_000)
            .expect("boundary-ending region is valid");
        assert_eq!(
            (region.x(), region.y(), region.width(), region.height()),
            (250_000, 100_000, 750_000, 900_000)
        );
        assert_eq!(
            NormalizedRegion::new(1_000_001, 0, 0, 0),
            Err(RegionError::ComponentOutOfBounds)
        );
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
    fn evidence_ref_preserves_locator_type() {
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
    fn producer_identities_require_non_empty_fields() {
        assert_eq!(
            ModelIdentity::new("model", " ", "rev", None)
                .expect_err("blank version must fail")
                .field(),
            "version"
        );
        assert_eq!(
            ParserIdentity::new("", "1", "rev", None)
                .expect_err("blank name must fail")
                .field(),
            "name"
        );
        let digest = [7_u8; 32];
        let model =
            ModelIdentity::new("engine", "1", "abc", Some(digest)).expect("valid model identity");
        assert_eq!(
            (
                model.name(),
                model.version(),
                model.revision(),
                model.digest()
            ),
            ("engine", "1", "abc", Some(digest))
        );
    }

    #[test]
    fn source_event_inherits_session_relationship() {
        let session_id = SessionId::new(1);
        let source = Source::new(SourceId::new(2), session_id);
        let envelope =
            EventEnvelope::source_registered(EventId::new(3), EventSequence::new(4), source);
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
        let artifact = Artifact::new(
            ArtifactId::new(12),
            SessionId::new(10),
            Some(SourceId::new(11)),
            ArtifactKind::Transcript,
        );
        let envelope =
            EventEnvelope::artifact_registered(EventId::new(13), EventSequence::new(14), artifact);
        assert_eq!(envelope.session_id(), artifact.session_id);
        assert_eq!(
            envelope.event(),
            CoreEvent::ArtifactRegistered {
                artifact_id: artifact.id,
                source_id: artifact.source_id,
                kind: artifact.kind
            }
        );
    }

    #[test]
    fn session_and_evidence_events_preserve_explicit_session() {
        let session = Session::new(SessionId::new(21));
        let created =
            EventEnvelope::session_created(EventId::new(22), EventSequence::new(0), session);
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
