//! 006A capture source lifecycle and session state machine.
//!
//! Portable contract over the closed 003/004/005 substrate: sources
//! attach to sessions, the session moves through explicit health states
//! on typed events, and every transition is total (accepted or
//! explicitly refused). Platform adapters (007+) bind native conditions
//! to these events; telemetry (006B) records the reasons the machine
//! classifies; checkpoints (006C) seal the durable states. Donor
//! recorder states (OpenSuperWhisper `AudioRecorder.swift`, reviewed
//! master `bef6bc0`) couple the machine to AppKit dictation surfaces;
//! this machine is chosen Himsat-native so all platforms share one
//! portable core.

use himsat_events::{SessionId, SourceId};

/// Portable source kind. Adapters refine these; the core never branches
/// on platform specifics.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SourceKind {
    /// Local microphone or line input.
    Microphone,
    /// OS-provided system/loopback audio where policy permits.
    SystemAudio,
    /// User-supplied file or media import.
    ImportedMedia,
    /// Peer device source over an authorized bridge.
    BridgePeer,
}

/// Opaque source descriptor: the 003 identity binding plus the
/// user-authorized label and portable kind. Labels carry no more
/// identity than the user approved; adapters own their content.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceDescriptor {
    id: SourceId,
    session_id: SessionId,
    kind: SourceKind,
    label: String,
}

impl SourceDescriptor {
    /// Binds a source id to its session with kind and label.
    #[must_use]
    pub fn new(id: SourceId, session_id: SessionId, kind: SourceKind, label: String) -> Self {
        Self {
            id,
            session_id,
            kind,
            label,
        }
    }

    /// Returns the 003 source identity.
    #[must_use]
    pub const fn id(&self) -> SourceId {
        self.id
    }

    /// Returns the owning session identity.
    #[must_use]
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    /// Returns the portable source kind.
    #[must_use]
    pub const fn kind(&self) -> SourceKind {
        self.kind
    }

    /// Returns the user-authorized display label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }
}

/// Machine reason for health-relevant transitions. Reasons classify;
/// telemetry records the details.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum HealthReason {
    /// OS audio route changed beneath the session.
    RouteChanged,
    /// Capture permission was revoked mid-session.
    PermissionRevoked,
    /// Screen/projection capture stopped externally.
    ProjectionStopped,
    /// Source connected but emits no meaningful audio.
    SourceSilent,
    /// Durable storage pressure threatens the session.
    DiskPressure,
    /// Writer process was interrupted (kill-equivalent).
    ProcessInterrupted,
    /// Thermal or resource pressure degrades capture.
    ThermalPressure,
}

/// Portable capture session states (architecture reliability machine).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CaptureState {
    /// Created, nothing prepared.
    Idle,
    /// Adapters preparing; sources may attach.
    Preparing,
    /// Audio flowing and durable.
    RecordingHealthy,
    /// Audio flowing with a classified degradation.
    RecordingDegraded,
    /// Flow stopped by a classified interruption.
    Interrupted,
    /// Adapters re-establishing flow after interruption.
    Recovering,
    /// Draining adapters toward finalization.
    Stopping,
    /// Sealing journal and manifest (006C checkpoint).
    Finalizing,
    /// Sealed and complete.
    Completed,
    /// Failed but retryable from prepare.
    FailedRecoverable,
    /// Failed terminally; the session never resumes.
    FailedTerminal,
}

impl CaptureState {
    /// Reports whether audio may be flowing (healthy, degraded,
    /// interrupted-but-held, or recovering).
    #[must_use]
    pub const fn is_active(self) -> bool {
        matches!(
            self,
            Self::RecordingHealthy | Self::RecordingDegraded | Self::Interrupted | Self::Recovering
        )
    }

    /// Reports whether the session reached a terminal state.
    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Completed | Self::FailedRecoverable | Self::FailedTerminal
        )
    }
}

/// Inputs to the session machine. Every variant is handled in every
/// state: accepted with a next state or explicitly refused.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CaptureEvent {
    /// Begin preparation from idle.
    Prepare,
    /// Adapters report ready with at least one source attached.
    SourcesReady,
    /// A flowing source degraded with reason.
    SourceDegraded(HealthReason),
    /// A degraded source recovered.
    SourceRecovered,
    /// Flow stopped with reason.
    Interrupted(HealthReason),
    /// Operator requests resume from interruption.
    ResumeRequested,
    /// Recovery re-established flow.
    RecoveryConfirmed,
    /// Recovery failed back to interruption.
    RecoveryFailed(HealthReason),
    /// Operator requests stop.
    StopRequested,
    /// Adapters drained; ready to seal.
    StopCompleted,
    /// Checkpoint sealed (006C).
    CheckpointSealed,
    /// Unrecoverable failure with reason.
    FailUnrecoverable(HealthReason),
    /// Recoverable failure with reason.
    FailRecoverable(HealthReason),
    /// Retry from a recoverable failure.
    RetryRequested,
}

/// Explicit refusal: the event, the state that refused it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RefusedTransition {
    state: CaptureState,
    event: CaptureEvent,
}

impl RefusedTransition {
    /// Returns the refusing state.
    #[must_use]
    pub const fn state(&self) -> CaptureState {
        self.state
    }

    /// Returns the refused event.
    #[must_use]
    pub const fn event(&self) -> CaptureEvent {
        self.event
    }
}

/// Source attach refusal.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AttachError {
    /// A source with this id is already attached.
    DuplicateSource(SourceId),
    /// The descriptor belongs to another session (transplant refused).
    SessionMismatch {
        /// Session that owns this capture session.
        expected: SessionId,
        /// Session named by the descriptor.
        found: SessionId,
    },
}

/// Portable capture session: attached sources plus the health state
/// machine. Single-owner by contract; callers serialize access.
#[derive(Clone, Debug, PartialEq)]
pub struct CaptureSession {
    session_id: SessionId,
    state: CaptureState,
    sources: Vec<SourceDescriptor>,
}

impl CaptureSession {
    /// Creates an idle session with no attached sources.
    #[must_use]
    pub const fn new(session_id: SessionId) -> Self {
        Self {
            session_id,
            state: CaptureState::Idle,
            sources: Vec::new(),
        }
    }

    /// Returns the owning session identity.
    #[must_use]
    pub const fn session_id(&self) -> SessionId {
        self.session_id
    }

    /// Returns the current health state.
    #[must_use]
    pub const fn state(&self) -> CaptureState {
        self.state
    }

    /// Returns attached sources in attach order.
    #[must_use]
    pub fn sources(&self) -> &[SourceDescriptor] {
        &self.sources
    }

    /// Attaches a source bound to this session. Duplicate ids and
    /// cross-session descriptors are refused.
    pub fn attach(&mut self, source: SourceDescriptor) -> Result<(), AttachError> {
        if source.session_id != self.session_id {
            return Err(AttachError::SessionMismatch {
                expected: self.session_id,
                found: source.session_id,
            });
        }
        if self.sources.iter().any(|known| known.id == source.id) {
            return Err(AttachError::DuplicateSource(source.id));
        }
        self.sources.push(source);
        Ok(())
    }

    /// Detaches a source by id, returning it when present.
    pub fn detach(&mut self, id: SourceId) -> Option<SourceDescriptor> {
        let index = self.sources.iter().position(|known| known.id == id)?;
        Some(self.sources.remove(index))
    }

    /// Applies one event: accepted events move the state, refused
    /// events leave it unchanged with an explicit refusal.
    pub fn apply(&mut self, event: CaptureEvent) -> Result<CaptureState, RefusedTransition> {
        let next = match event {
            CaptureEvent::Prepare => {
                if self.state == CaptureState::Idle {
                    Some(CaptureState::Preparing)
                } else {
                    None
                }
            }
            CaptureEvent::SourcesReady => {
                if self.state == CaptureState::Preparing && !self.sources.is_empty() {
                    Some(CaptureState::RecordingHealthy)
                } else {
                    None
                }
            }
            CaptureEvent::SourceDegraded(_) => match self.state {
                CaptureState::RecordingHealthy | CaptureState::RecordingDegraded => {
                    Some(CaptureState::RecordingDegraded)
                }
                _ => None,
            },
            CaptureEvent::SourceRecovered => {
                if self.state == CaptureState::RecordingDegraded {
                    Some(CaptureState::RecordingHealthy)
                } else {
                    None
                }
            }
            CaptureEvent::Interrupted(_) => match self.state {
                CaptureState::RecordingHealthy
                | CaptureState::RecordingDegraded
                | CaptureState::Interrupted => Some(CaptureState::Interrupted),
                _ => None,
            },
            CaptureEvent::ResumeRequested => {
                if self.state == CaptureState::Interrupted {
                    Some(CaptureState::Recovering)
                } else {
                    None
                }
            }
            CaptureEvent::RecoveryConfirmed => {
                if self.state == CaptureState::Recovering {
                    Some(CaptureState::RecordingHealthy)
                } else {
                    None
                }
            }
            CaptureEvent::RecoveryFailed(_) => {
                if self.state == CaptureState::Recovering {
                    Some(CaptureState::Interrupted)
                } else {
                    None
                }
            }
            CaptureEvent::StopRequested => {
                if self.state.is_active() {
                    Some(CaptureState::Stopping)
                } else {
                    None
                }
            }
            CaptureEvent::StopCompleted => {
                if self.state == CaptureState::Stopping {
                    Some(CaptureState::Finalizing)
                } else {
                    None
                }
            }
            CaptureEvent::CheckpointSealed => {
                if self.state == CaptureState::Finalizing {
                    Some(CaptureState::Completed)
                } else {
                    None
                }
            }
            CaptureEvent::FailUnrecoverable(_) => {
                if self.state.is_terminal() {
                    None
                } else {
                    Some(CaptureState::FailedTerminal)
                }
            }
            CaptureEvent::FailRecoverable(_) => {
                if self.state.is_terminal() {
                    None
                } else {
                    Some(CaptureState::FailedRecoverable)
                }
            }
            CaptureEvent::RetryRequested => {
                if self.state == CaptureState::FailedRecoverable {
                    Some(CaptureState::Preparing)
                } else {
                    None
                }
            }
        };
        match next {
            Some(state) => {
                self.state = state;
                Ok(state)
            }
            None => Err(RefusedTransition {
                state: self.state,
                event,
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AttachError, CaptureEvent, CaptureSession, CaptureState, HealthReason, SourceDescriptor,
        SourceKind,
    };
    use himsat_events::{SessionId, SourceId};

    const SESSION: SessionId = SessionId::new(1);
    const FOREIGN: SessionId = SessionId::new(2);

    fn descriptor(id: u128) -> SourceDescriptor {
        SourceDescriptor::new(
            SourceId::new(id),
            SESSION,
            SourceKind::Microphone,
            format!("mic-{id}"),
        )
    }

    fn session() -> CaptureSession {
        CaptureSession::new(SESSION)
    }

    fn ready_session() -> CaptureSession {
        let mut session = session();
        session.attach(descriptor(10)).expect("attach");
        session.apply(CaptureEvent::Prepare).expect("prepare");
        session.apply(CaptureEvent::SourcesReady).expect("ready");
        session
    }

    fn session_in(target: CaptureState) -> CaptureSession {
        use CaptureEvent::{
            CheckpointSealed, FailRecoverable, FailUnrecoverable, Interrupted, Prepare,
            ResumeRequested, SourceDegraded, SourcesReady, StopCompleted, StopRequested,
        };
        use HealthReason::{DiskPressure, PermissionRevoked, RouteChanged};
        let mut session = session();
        if target != CaptureState::Idle {
            session.attach(descriptor(10)).expect("attach");
            session.apply(Prepare).expect("prepare");
        }
        match target {
            CaptureState::Idle | CaptureState::Preparing => {}
            CaptureState::RecordingHealthy => {
                session.apply(SourcesReady).expect("ready");
            }
            CaptureState::RecordingDegraded => {
                session.apply(SourcesReady).expect("ready");
                session
                    .apply(SourceDegraded(RouteChanged))
                    .expect("degrade");
            }
            CaptureState::Interrupted => {
                session.apply(SourcesReady).expect("ready");
                session.apply(Interrupted(RouteChanged)).expect("interrupt");
            }
            CaptureState::Recovering => {
                session.apply(SourcesReady).expect("ready");
                session.apply(Interrupted(RouteChanged)).expect("interrupt");
                session.apply(ResumeRequested).expect("resume");
            }
            CaptureState::Stopping => {
                session.apply(SourcesReady).expect("ready");
                session.apply(StopRequested).expect("stop");
            }
            CaptureState::Finalizing => {
                session.apply(SourcesReady).expect("ready");
                session.apply(StopRequested).expect("stop");
                session.apply(StopCompleted).expect("drained");
            }
            CaptureState::Completed => {
                session.apply(SourcesReady).expect("ready");
                session.apply(StopRequested).expect("stop");
                session.apply(StopCompleted).expect("drained");
                session.apply(CheckpointSealed).expect("seal");
            }
            CaptureState::FailedRecoverable => {
                session.apply(SourcesReady).expect("ready");
                session
                    .apply(FailRecoverable(DiskPressure))
                    .expect("fail soft");
            }
            CaptureState::FailedTerminal => {
                session.apply(SourcesReady).expect("ready");
                session
                    .apply(FailUnrecoverable(PermissionRevoked))
                    .expect("fail hard");
            }
        }
        assert_eq!(session.state(), target);
        session
    }

    #[test]
    fn new_session_is_idle_without_sources() {
        let session = session();
        assert_eq!(session.session_id(), SESSION);
        assert_eq!(session.state(), CaptureState::Idle);
        assert!(session.sources().is_empty());
        assert!(!session.state().is_active());
        assert!(!session.state().is_terminal());
    }

    #[test]
    fn attach_detach_lifecycle() {
        let mut session = session();
        session.attach(descriptor(10)).expect("attach first");
        session.attach(descriptor(11)).expect("attach second");
        assert_eq!(session.sources().len(), 2);
        assert_eq!(session.sources()[0].id(), SourceId::new(10));
        assert_eq!(session.sources()[0].label(), "mic-10");
        assert_eq!(session.sources()[0].kind(), SourceKind::Microphone);
        let detached = session.detach(SourceId::new(10)).expect("detach");
        assert_eq!(detached.id(), SourceId::new(10));
        assert!(session.detach(SourceId::new(99)).is_none());
        assert_eq!(session.sources().len(), 1);
    }

    #[test]
    fn attach_refuses_duplicate_and_foreign_session() {
        let mut session = session();
        session.attach(descriptor(10)).expect("attach");
        assert_eq!(
            session.attach(descriptor(10)),
            Err(AttachError::DuplicateSource(SourceId::new(10)))
        );
        let foreign = SourceDescriptor::new(
            SourceId::new(20),
            FOREIGN,
            SourceKind::SystemAudio,
            String::from("foreign"),
        );
        assert_eq!(
            session.attach(foreign),
            Err(AttachError::SessionMismatch {
                expected: SESSION,
                found: FOREIGN,
            })
        );
        assert_eq!(session.sources().len(), 1);
    }

    #[test]
    fn sources_ready_needs_an_attached_source() {
        let mut session = session();
        session.apply(CaptureEvent::Prepare).expect("prepare");
        let refusal = session
            .apply(CaptureEvent::SourcesReady)
            .expect_err("ready without sources");
        assert_eq!(refusal.state(), CaptureState::Preparing);
        assert_eq!(refusal.event(), CaptureEvent::SourcesReady);
        assert_eq!(session.state(), CaptureState::Preparing);
    }

    #[test]
    fn happy_path_reaches_completed() {
        let mut session = ready_session();
        assert_eq!(session.state(), CaptureState::RecordingHealthy);
        assert!(session.state().is_active());
        assert_eq!(
            session.apply(CaptureEvent::StopRequested),
            Ok(CaptureState::Stopping)
        );
        assert_eq!(
            session.apply(CaptureEvent::StopCompleted),
            Ok(CaptureState::Finalizing)
        );
        assert_eq!(
            session.apply(CaptureEvent::CheckpointSealed),
            Ok(CaptureState::Completed)
        );
        assert!(session.state().is_terminal());
    }

    #[test]
    fn degrade_recover_loop() {
        let mut session = ready_session();
        assert_eq!(
            session.apply(CaptureEvent::SourceDegraded(HealthReason::ThermalPressure)),
            Ok(CaptureState::RecordingDegraded)
        );
        assert_eq!(
            session.apply(CaptureEvent::SourceRecovered),
            Ok(CaptureState::RecordingHealthy)
        );
    }

    #[test]
    fn interrupt_resume_recover_or_fail_back() {
        let mut session = ready_session();
        assert_eq!(
            session.apply(CaptureEvent::Interrupted(HealthReason::RouteChanged)),
            Ok(CaptureState::Interrupted)
        );
        assert_eq!(
            session.apply(CaptureEvent::ResumeRequested),
            Ok(CaptureState::Recovering)
        );
        assert_eq!(
            session.apply(CaptureEvent::RecoveryFailed(HealthReason::DiskPressure)),
            Ok(CaptureState::Interrupted)
        );
        assert_eq!(
            session.apply(CaptureEvent::ResumeRequested),
            Ok(CaptureState::Recovering)
        );
        assert_eq!(
            session.apply(CaptureEvent::RecoveryConfirmed),
            Ok(CaptureState::RecordingHealthy)
        );
    }

    #[test]
    fn retry_from_recoverable_reprepares() {
        let mut session = ready_session();
        assert_eq!(
            session.apply(CaptureEvent::FailRecoverable(HealthReason::DiskPressure)),
            Ok(CaptureState::FailedRecoverable)
        );
        assert_eq!(
            session.apply(CaptureEvent::RetryRequested),
            Ok(CaptureState::Preparing)
        );
        assert_eq!(
            session.apply(CaptureEvent::SourcesReady),
            Ok(CaptureState::RecordingHealthy)
        );
    }

    #[test]
    fn terminal_states_refuse_everything() {
        use CaptureEvent::{FailUnrecoverable, Prepare, StopRequested};
        use HealthReason::ProcessInterrupted;
        for state in [CaptureState::Completed, CaptureState::FailedTerminal] {
            let mut session = session_in(state);
            for event in [
                Prepare,
                StopRequested,
                FailUnrecoverable(ProcessInterrupted),
            ] {
                let before = session.state();
                assert_eq!(
                    session.apply(event),
                    Err(super::RefusedTransition {
                        state: before,
                        event,
                    }),
                    "{state:?} refuses {event:?}"
                );
                assert_eq!(session.state(), before);
            }
        }
    }

    #[test]
    fn every_reason_is_accepted_in_degraded() {
        use HealthReason::{
            DiskPressure, PermissionRevoked, ProcessInterrupted, ProjectionStopped, RouteChanged,
            SourceSilent, ThermalPressure,
        };
        for reason in [
            RouteChanged,
            PermissionRevoked,
            ProjectionStopped,
            SourceSilent,
            DiskPressure,
            ProcessInterrupted,
            ThermalPressure,
        ] {
            let mut session = ready_session();
            assert_eq!(
                session.apply(CaptureEvent::SourceDegraded(reason)),
                Ok(CaptureState::RecordingDegraded),
                "{reason:?} classifies"
            );
        }
    }

    #[test]
    fn machine_is_total_over_states_and_events() {
        use HealthReason::{DiskPressure, PermissionRevoked, RouteChanged, ThermalPressure};
        const STATES: [CaptureState; 11] = [
            CaptureState::Idle,
            CaptureState::Preparing,
            CaptureState::RecordingHealthy,
            CaptureState::RecordingDegraded,
            CaptureState::Interrupted,
            CaptureState::Recovering,
            CaptureState::Stopping,
            CaptureState::Finalizing,
            CaptureState::Completed,
            CaptureState::FailedRecoverable,
            CaptureState::FailedTerminal,
        ];
        const EVENTS: [CaptureEvent; 14] = [
            CaptureEvent::Prepare,
            CaptureEvent::SourcesReady,
            CaptureEvent::SourceDegraded(RouteChanged),
            CaptureEvent::SourceRecovered,
            CaptureEvent::Interrupted(PermissionRevoked),
            CaptureEvent::ResumeRequested,
            CaptureEvent::RecoveryConfirmed,
            CaptureEvent::RecoveryFailed(DiskPressure),
            CaptureEvent::StopRequested,
            CaptureEvent::StopCompleted,
            CaptureEvent::CheckpointSealed,
            CaptureEvent::FailUnrecoverable(ThermalPressure),
            CaptureEvent::FailRecoverable(DiskPressure),
            CaptureEvent::RetryRequested,
        ];
        for state in STATES {
            for event in EVENTS {
                let mut session = session_in(state);
                match session.apply(event) {
                    Ok(next) => assert_eq!(session.state(), next),
                    Err(refusal) => {
                        assert_eq!(refusal.state(), state);
                        assert_eq!(refusal.event(), event);
                        assert_eq!(session.state(), state);
                    }
                }
            }
        }
    }
}
