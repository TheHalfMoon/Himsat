# Himsat Product Plan

## Product thesis

Himsat is not a meeting bot and not a transcription utility. It is a **private multimodal memory system** for what the user hears, says, watches, reads, decides, and promises.

Core promise:

> **Capture it. Understand it. Remember it. Prove it. Transform it. Act on it — on devices the user controls.**

## Primary user experiences

### 1. Record

One-tap user-authorized capture for an in-person conversation, lecture, interview, field note, or ad-hoc session.

Requirements:

- immediate recording-state feedback;
- microphone/source health;
- pause/resume/bookmark;
- background/locked-screen continuation where the OS permits it;
- crash-safe chunking;
- optional live captions;
- no network requirement after local models are installed.

### 2. Meeting

A structured session enriched by optional calendar/contacts/project context.

Requirements:

- meeting detection/reminder;
- participants and agenda;
- mic + desktop system audio where supported;
- optional selected-screen context;
- human notes that influence emphasis but do not replace transcript evidence;
- live decisions/questions/actions;
- post-meeting brief and evidence links.

### 3. Memory Session

A longer explicit session that continues until the user stops it.

Requirements:

- clear persistent recording state;
- silence/topic/session segmentation;
- storage/battery/thermal forecast;
- privacy retention policy;
- automatic checkpointing;
- optional “transcript only” or ephemeral audio retention modes.

### 4. Watch / Media

Understand a video, lecture, podcast, webinar, or local media file.

Inputs:

- imported file;
- desktop system audio;
- phone microphone listening to another device;
- Himsat Bridge stream;
- selected screen/window capture where supported.

Outputs include transcript, chapters, slides/keyframes, OCR, concepts, questions, study brief, citations, and searchable timeline.

### 5. Read / Documents

Talk to and transform PDF, DOCX, PPTX, XLSX, images, Markdown, HTML, text, and later email/archive/scientific formats through a Himsat-owned intermediate representation.

### 6. Ask

Search one source or the full authorized local memory:

- lexical;
- semantic;
- temporal;
- person/project/topic filters;
- exact evidence;
- cross-document and meeting comparison.

### 7. Publish / Transform

Turn memory into professional artifacts and portable AI context.

### 8. Act

Local flows and agents propose or perform permitted actions using explicit capabilities and evidence.

## Capture feature map

### Desktop capture

Planned baseline:

- macOS microphone;
- macOS system audio using supported native capture APIs;
- Windows microphone;
- Windows system audio;
- Linux microphone;
- Linux PipeWire-first system audio with appropriate fallbacks;
- multi-device/audio-device selection;
- Bluetooth/USB/external microphones;
- per-source level meters;
- menu bar/system tray runtime;
- start at login / ready mode;
- global shortcut;
- automatic meeting detection as an opt-in policy;
- floating recording control;
- selected-window/screen context where supported.

### Mobile capture

Planned baseline:

- native iOS/iPadOS microphone recording;
- native Android microphone recording;
- user-started background continuation;
- lock-screen/notification controls;
- Apple Live Activity and system recording intents where supported;
- Android microphone foreground service;
- Bluetooth headset/microphone route support;
- interruption recovery;
- share-sheet/import audio/video;
- camera document scan as a later bounded unit.

Capabilities that are **not universal baseline promises**:

- arbitrary iOS other-app audio capture;
- arbitrary cellular phone-call uplink/downlink capture;
- Android playback capture when the source app blocks capture;
- protected DRM media capture;
- future/beta ScreenCaptureKit behavior on old OS versions.

### Wearables

Apple Watch first:

- start/stop/pause;
- bookmark/important marker;
- independent short recording where platform and battery allow;
- remote-control phone session;
- encrypted local transfer.

Wear OS is a later parity track.

## Himsat Bridge

Himsat Bridge is a local device protocol that allows a phone and desktop to behave as one session.

Example:

```text
Phone UI             Desktop
---------            -------
Start Session   ->   capture system audio
Phone mic       ->   optional local source
Bookmarks       ->   shared timeline
                <-   live transcript/status
```

Design requirements:

- authenticated device pairing;
- explicit session join;
- local/direct path preference;
- encrypted transport;
- clock synchronization and drift measurement;
- source-level provenance;
- graceful disconnect/rejoin;
- no requirement for a Himsat account.

Later **Consensus Capture** can fuse multiple user-authorized devices while preserving source lineage.

## Recall Buffer

Optional privacy-sensitive feature:

- visible recording mode;
- bounded rolling duration;
- disabled by default;
- overwritten unless saved;
- preferably RAM-backed with encrypted spill when necessary;
- user action makes the selected interval durable;
- subject to the same permission/indicator/consent policy as normal recording.

## Audio intelligence

### Capture-quality processing

Planned stages:

- device/source normalization;
- resampling;
- AEC;
- noise suppression;
- AGC/limiting;
- clipping detection;
- VAD;
- channel/source alignment;
- silence and discontinuity detection;
- optional dereverberation/beamforming research.

### Capture Health

Live health signals:

- source connected;
- meaningful audio detected;
- clipping/noise warnings;
- route changed;
- source became silent;
- storage remaining;
- thermal/power status;
- transcription backlog;
- last durable checkpoint.

## Speech intelligence

### Two-pass transcription

**Live pass** prioritizes latency and battery/resource use.  
**Quality pass** reprocesses durable audio with the best locally qualified engine for the device/language.

The refined transcript must preserve lineage to the live transcript and source audio.

### Speech engine router

Planned adapters may include:

- Whisper/whisper.cpp family;
- sherpa-onnx;
- Apple-native Argmax OSS/WhisperKit path;
- Moonshine where license/model and benchmark gates pass;
- Parakeet/ONNX paths;
- future engines through a stable Himsat interface.

No single model is the product contract.

### Language quality

First-class benchmark areas:

- English;
- Modern Standard Arabic;
- Saudi/Najdi/Hijazi;
- Gulf Arabic;
- Egyptian and Levantine expansion;
- Arabic-English code switching;
- proper nouns;
- medical, technical, research, and business terminology;
- numbers, dates, currencies, acronyms.

### Personal vocabulary

Local user-controlled phrase/name/domain vocabulary with correction history and scoped biasing. Corrections must remain exportable/deletable.

## Speaker intelligence

- diarization;
- overlapping speech detection;
- editable speaker labels;
- optional speaker enrollment/profile;
- local encrypted speaker embeddings;
- confidence and uncertainty;
- user correction;
- delete/reset speaker identity independently of transcript.

Speaker biometrics are sensitive and are never an ambient agent capability.

## Multimodal timeline

One canonical session timeline can reference:

- microphone source;
- system audio source;
- transcript words/segments;
- speaker events;
- manual notes;
- bookmarks;
- selected-screen frames;
- OCR blocks;
- slides;
- imported document pages;
- decisions/tasks/questions;
- model outputs;
- corrections.

Every derived object carries provenance to source objects.

## Himsat Lens

Opt-in visual context:

- selected screen/window or system-selected share surface;
- keyframes instead of indiscriminate high-rate storage by default;
- slide/change detection;
- local OCR;
- local vision where available;
- link visual evidence to meeting timestamps;
- clear on/off state;
- retention controls independent of audio.

## Document Intelligence

### Input targets

Priority:

1. PDF;
2. DOCX;
3. PPTX;
4. XLSX/CSV;
5. images/scans;
6. Markdown/TXT/HTML;
7. email/archives/eBooks/scientific formats based on evidence of need.

### Himsat Document IR

The canonical memory layer must not depend on parser-specific types. IR objects should support:

- document/page/slide/sheet hierarchy;
- headings/paragraphs/lists;
- tables/cells;
- figures/images/charts;
- formulas/code;
- annotations/comments;
- regions/bounding boxes;
- OCR confidence;
- hyperlinks/references;
- attachments/media;
- source digest;
- parser/model identity.

### Talk to PDF/Documents

Examples:

- “What methodology did the authors use?”
- “Show the page and paragraph supporting this answer.”
- “Compare Table 3 and Table 7.”
- “What changed between contract v1 and v2?”
- “What did we agree in the meeting that is missing from v2?”
- “Find claims in this paper that conflict with our last design meeting.”

Answers must cite page/region/source evidence when available.

### Live document linkage

When a document/slide is present during a meeting, Himsat should be able to bind discussion timestamps to page/slide evidence where the platform/user has authorized the context.

## Memory system

### Core entities

- Person;
- Organization;
- Project/Space;
- Session/Meeting;
- Topic;
- Document;
- Decision;
- Commitment;
- Task;
- Risk;
- Question;
- Claim;
- Deadline;
- Artifact;
- Evidence.

### Temporal relationships

Support:

- made_by;
- discussed_in;
- supports;
- contradicts;
- supersedes;
- replaced_by;
- promised_to;
- due_at;
- resolved_by;
- depends_on;
- changed_at.

### Killer memory experiences

**Decision Drift** — show how a decision changed and why.  
**Commitment Ledger** — owner, due date, status, evidence.  
**Conversation Time Machine** — answer what the project believed at a historical date.  
**What Changed?** — changes since the previous meeting/week/version.  
**Open Questions** — unresolved questions across meetings/documents.  
**Contradiction Radar** — source-backed conflicts between meetings, documents, and current project state.

## Notes and human steering

Human notes are strong signals but not hidden replacements for evidence.

Markers may include:

- normal note;
- important;
- question;
- decision candidate;
- follow-up;
- bookmark;
- private.

AI output should be able to report that a user-marked question was never answered.

## Himsat Brief Engine

Post-session profiles:

- 60-second brief;
- five-minute brief;
- executive summary;
- detailed notes/minutes;
- decisions;
- commitments/actions;
- open questions;
- risks;
- what changed;
- study notes;
- technical notes;
- Q&A;
- timeline;
- follow-up draft.

Pre-session **Himsat Brief**:

- who/what the meeting is about;
- relevant past meetings;
- open commitments;
- overdue items;
- unresolved questions;
- recent decision changes;
- relevant documents;
- suggested evidence-backed questions.

## Himsat Live

Local floating assistance during a session:

- live transcript;
- manual notes;
- bookmarks;
- decisions detected;
- unanswered questions;
- relevant prior facts/doc pages;
- contradictions;
- optional playbook/checklist guidance;
- pace/monologue metrics only as opt-in coaching signals, not personality judgments.

## Search

Hybrid local search:

- FTS/lexical;
- semantic/vector;
- reranking;
- graph traversal;
- temporal filters;
- speaker/person/project/document filters;
- exact evidence jump;
- audio replay from cited range;
- cross-language retrieval.

For translated search, preserve the original-language source as authority.

## Publish and Transform

### Human publishing

Professional outputs:

- PDF;
- DOCX;
- Markdown/HTML;
- VTT/SRT;
- CSV for actions/decisions when useful.

Templates:

- executive report;
- meeting minutes;
- research brief;
- study guide;
- PRD;
- technical specification;
- SOP;
- proposal;
- project plan;
- action register;
- decision record;
- FAQ/knowledge article.

PDF direction:

- Typst-based publishing candidate;
- semantic structure;
- page numbers/TOC/bookmarks;
- RTL and mixed Arabic/English;
- PDF/A archival profiles;
- PDF/UA accessibility where supported;
- selectable/searchable text;
- evidence/source appendix options.

DOCX direction:

- real semantic headings/styles/lists/tables;
- hyperlinks/bookmarks/footnotes where useful;
- clean editing in common office suites;
- no layout tricks that make downstream parsing fragile.

### Himsat Context Pack

Portable AI-first bundle:

```text
manifest.json
content.md
evidence.jsonl
entities.jsonl
relations.jsonl
assets/
```

Properties:

- documented open schema;
- source IDs stable inside bundle;
- token-budget export profiles;
- deterministic ordering;
- included/excluded source declarations;
- model-agnostic Markdown rather than proprietary prompt wrappers;
- usable by local models, ChatGPT/Claude/Gemini-style systems, code agents, and future tooling without requiring Himsat.

## Local agents and Himsat Flows

### Local surfaces

- MCP;
- CLI;
- local HTTP/IPC API;
- Rust SDK where useful;
- later plugin SDK.

### Capability examples

```text
meetings.search            allow
transcripts.read           allow
decisions.read             allow
documents.read             ask
raw_audio.read             ask
speaker_biometrics.read    deny
recording.start            ask
connector.write            ask
filesystem.write           deny by default
```

### Himsat Flows

Condition/action model:

```text
WHEN meeting.ends
IF participant/project/topic conditions match
EXTRACT decisions/actions
THEN create local tasks + draft external actions
REQUIRE approval before external write
```

Support dry-run, replay, deterministic action schema validation, and evidence of why an action was proposed.

### Plugin sandbox

Third-party plugins should prefer a Wasm sandbox/capability host rather than ambient in-process code execution.

## Connectors

Connectors are optional and never required for local core function.

Storage/sync candidates:

- Google Drive;
- Box;
- OneDrive;
- Dropbox;
- WebDAV;
- S3-compatible;
- NAS/local folder;
- removable storage.

Workflow candidates:

- calendar/contacts;
- email;
- Slack;
- Notion;
- GitHub;
- Linear/Jira;
- CRM systems;
- Zapier/Make/webhooks through opt-in extensions.

Rules:

- least privilege;
- content-minimized egress;
- explicit preview for sensitive writes;
- no connector receives vault keys;
- encrypted user-owned storage should see ciphertext for vault backup/sync where that mode is used.

## P2P and sharing

### Device sync

- QR/local pairing;
- device public keys;
- direct/LAN path preferred;
- user-controlled relay/self-hosted relay option if needed;
- strict Network Lock mode forbids unexpected public relay use;
- resumable content-addressed transfer;
- per-space/key separation.

### Selective sharing

Share scopes:

- summary only;
- transcript without audio;
- selected clips/evidence;
- actions only;
- redacted version;
- full session.

Private notes and speaker biometrics stay excluded unless explicitly selected.

## Privacy modes

Retention examples:

- audio + transcript;
- transcript only after successful verification;
- summary/evidence only;
- delete after time limit;
- ephemeral session;
- privacy pause;
- separate visual-context retention.

A deletion flow must address derived indexes, embeddings, summaries, exports, sync replicas, and backups explicitly instead of deleting only the visible row.

## Migration

**Bring Your Memory** adapters should import available exports from incumbent tools and generic media/folder structures while producing a migration report that lists preserved, transformed, and unavailable fields.

## Daily private intelligence

Optional local scheduled surfaces:

- Today brief;
- upcoming meeting prep;
- overdue commitments;
- decisions changed this week;
- unresolved questions;
- project digest;
- storage/model health;
- connector pending approvals.

## “Best ever” definition

The product may only claim comparative quality in dimensions it has measured. Himsat's internal definition of best-in-class requires simultaneously strong performance in:

- capture reliability;
- speech quality;
- diarization;
- multilingual/Arabic quality;
- battery/resource use;
- search/retrieval;
- evidence coverage;
- document extraction;
- professional publishing;
- accessibility;
- privacy/offline behavior;
- portability;
- user experience.

A feature-rich but unreliable recorder fails this definition.
