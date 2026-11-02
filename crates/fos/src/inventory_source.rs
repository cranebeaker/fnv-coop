//! Immutable original inventory payloads and their source context.
//!
//! The existing reference/actor decoder records spans while reading each item,
//! stack and extra. Successful *complete record* decoding publishes envelopes;
//! failures retain the entire record without guessing unknown payload lengths.
//! This module does not import game state, resolve current plugins, or edit saves.
//! Span boundaries follow the existing PC-derived parsers: ItemChange SaveGame
//! 004bed60, ExtraDataList SaveGame 00426a30 and InventoryChanges SaveGame 004d4090
//! (class names Xbox PDB; FalloutNV.exe 1.4.0.525).

use crate::{decode, save_type, ChangeForm, Error, RefId, Result, Save};
use std::{fmt, ops::Range, sync::Arc};

#[derive(Default)]
pub(crate) struct InventoryCapture {
    pub(crate) items: Vec<ItemSpans>,
}

pub(crate) struct ItemSpans {
    pub(crate) range: Range<usize>,
    pub(crate) stacks: Vec<StackSpans>,
}

pub(crate) struct StackSpans {
    pub(crate) range: Range<usize>,
    pub(crate) extras: Vec<Range<usize>>,
}

/// Resolution through the captured save's table, not the active load order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    Null,
    ResolvedInSave(u32),
    MissingIndex { index: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SavedIdentity {
    saved: RefId,
    resolution: Resolution,
}

impl SavedIdentity {
    fn new(saved: RefId, form_ids: &[u32]) -> Self {
        let resolution = if saved.0 == 0 {
            Resolution::Null
        } else {
            match saved.form_id(form_ids) {
                Some(id) => Resolution::ResolvedInSave(id),
                None => Resolution::MissingIndex { index: saved.0 },
            }
        };
        Self { saved, resolution }
    }

    pub fn saved(&self) -> RefId {
        self.saved
    }
    pub fn resolution(&self) -> Resolution {
        self.resolution
    }
}

/// One captured immutable input. A label or filename is not its identity.
pub struct SaveContext {
    bytes: Arc<[u8]>,
    version: u32,
    minor_version: u8,
    plugins: Vec<String>,
    form_ids: Vec<u32>,
}

impl fmt::Debug for SaveContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SaveContext")
            .field("source_length", &self.bytes.len())
            .field("version", &self.version)
            .field("minor_version", &self.minor_version)
            .field("plugins", &self.plugins)
            .field("form_id_count", &self.form_ids.len())
            .finish()
    }
}

impl SaveContext {
    pub fn original_bytes(&self) -> &[u8] {
        &self.bytes
    }
    pub fn version(&self) -> u32 {
        self.version
    }
    pub fn minor_version(&self) -> u8 {
        self.minor_version
    }
    pub fn plugins(&self) -> &[String] {
        &self.plugins
    }
    pub fn form_ids(&self) -> &[u32] {
        &self.form_ids
    }

    /// Whether envelopes share this particular captured context. This is not a
    /// content hash: separately parsed equal inputs still have distinct contexts.
    pub fn same_capture(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

#[derive(Debug)]
pub struct RecordSource {
    save: Arc<SaveContext>,
    ordinal: usize,
    header_file_span: Range<usize>,
    data_file_span: Range<usize>,
    holder: SavedIdentity,
    save_type: u8,
    version: u8,
    flags: u32,
}

impl RecordSource {
    fn new(save: Arc<SaveContext>, cf: &ChangeForm<'_>, ordinal: usize) -> Result<Self> {
        let end = cf
            .data_offset
            .checked_add(cf.data.len())
            .ok_or_else(|| Error::new(cf.offset, "record span overflow"))?;
        if cf.offset >= cf.data_offset || save.bytes.get(cf.data_offset..end) != Some(cf.data) {
            return Err(Error::new(cf.offset, "record is not a direct source slice"));
        }
        let holder = SavedIdentity::new(cf.ref_id, &save.form_ids);
        Ok(Self {
            save,
            ordinal,
            header_file_span: cf.offset..cf.data_offset,
            data_file_span: cf.data_offset..end,
            holder,
            save_type: cf.save_type,
            version: cf.version,
            flags: cf.flags,
        })
    }

    pub fn save(&self) -> &SaveContext {
        &self.save
    }
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
    pub fn header_file_span(&self) -> Range<usize> {
        self.header_file_span.clone()
    }
    pub fn data_file_span(&self) -> Range<usize> {
        self.data_file_span.clone()
    }
    pub fn original_header(&self) -> &[u8] {
        &self.save.bytes[self.header_file_span.clone()]
    }
    pub fn original_data(&self) -> &[u8] {
        &self.save.bytes[self.data_file_span.clone()]
    }
    pub fn holder(&self) -> &SavedIdentity {
        &self.holder
    }
    pub fn save_type(&self) -> u8 {
        self.save_type
    }
    pub fn version(&self) -> u8 {
        self.version
    }
    pub fn flags(&self) -> u32 {
        self.flags
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordFailure {
    /// The actor/player distinction cannot be inferred from an unresolved holder.
    UnresolvedHolder(SavedIdentity),
    /// Original decoder diagnostic. Some existing decoder offsets are relative
    /// to a nested reader; use RecordSource for authoritative source ranges.
    Decode(Error),
    /// Internal span bookkeeping failed; no partial items are published.
    InvalidCapture(Error),
}

#[derive(Debug, Clone)]
pub enum RecordState {
    /// The header has no inventory flag. Its other fields were not decoded here.
    NoInventory,
    Decoded(Vec<ItemEnvelope>),
    PreservedUndecoded(RecordFailure),
}

#[derive(Debug, Clone)]
pub struct InventoryRecord {
    source: Arc<RecordSource>,
    state: RecordState,
}

impl InventoryRecord {
    pub fn source(&self) -> &RecordSource {
        &self.source
    }
    pub fn state(&self) -> &RecordState {
        &self.state
    }
}

/// Captured REFR/ACHR/ACRE inventory records, with failures explicitly retained.
#[derive(Debug, Clone)]
pub struct InventorySource {
    context: Arc<SaveContext>,
    records: Vec<InventoryRecord>,
}

impl InventorySource {
    /// Parses one owned immutable input. Structural save failures return Err;
    /// individual record decode failures remain inspectable in `records()`.
    pub fn parse(bytes: Arc<[u8]>) -> Result<Self> {
        let parsed = Save::parse(&bytes)?;
        let context = Arc::new(SaveContext {
            bytes: Arc::clone(&bytes),
            version: parsed.header.version,
            minor_version: parsed.minor_version,
            plugins: parsed.plugins.clone(),
            form_ids: parsed.form_ids.clone(),
        });
        let mut records = Vec::new();
        for (ordinal, cf) in parsed.change_forms.iter().enumerate() {
            if !matches!(
                cf.save_type,
                save_type::REFR | save_type::ACHR | save_type::ACRE
            ) {
                continue;
            }
            let source = Arc::new(RecordSource::new(Arc::clone(&context), cf, ordinal)?);
            let state = if cf.flags & 0x0800_0020 == 0 {
                RecordState::NoInventory
            } else if let Resolution::ResolvedInSave(holder) = source.holder.resolution {
                let mut capture = InventoryCapture::default();
                let decoded = if cf.save_type == save_type::REFR {
                    decode::reference_captured(cf, &mut capture).map(|r| r.data.inventory)
                } else {
                    decode::actor_form_captured(cf, holder == 0x14, &mut capture)
                        .map(|r| r.actor.mobile.data.inventory)
                };
                match decoded {
                    Err(error) => RecordState::PreservedUndecoded(RecordFailure::Decode(error)),
                    Ok(Some(items)) => match envelopes(&source, items, capture) {
                        Ok(items) => RecordState::Decoded(items),
                        Err(error) => {
                            RecordState::PreservedUndecoded(RecordFailure::InvalidCapture(error))
                        }
                    },
                    Ok(None) => {
                        RecordState::PreservedUndecoded(RecordFailure::InvalidCapture(Error::new(
                            cf.data_offset,
                            "inventory flag produced no decoded inventory",
                        )))
                    }
                }
            } else {
                RecordState::PreservedUndecoded(RecordFailure::UnresolvedHolder(source.holder))
            };
            records.push(InventoryRecord { source, state });
        }
        Ok(Self { context, records })
    }

    pub fn context(&self) -> &SaveContext {
        &self.context
    }
    pub fn original_bytes(&self) -> &[u8] {
        &self.context.bytes
    }
    pub fn records(&self) -> &[InventoryRecord] {
        &self.records
    }
}

#[derive(Debug, Clone)]
pub struct ItemEnvelope {
    source: Arc<RecordSource>,
    ordinal: usize,
    identity: SavedIdentity,
    count_delta: i32,
    data_span: Range<usize>,
    stacks: Vec<StackEnvelope>,
}

impl ItemEnvelope {
    pub fn source(&self) -> &RecordSource {
        &self.source
    }
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
    pub fn identity(&self) -> &SavedIdentity {
        &self.identity
    }
    pub fn count_delta(&self) -> i32 {
        self.count_delta
    }
    /// Range relative to the containing record's original data.
    pub fn record_data_span(&self) -> Range<usize> {
        self.data_span.clone()
    }
    pub fn file_span(&self) -> Range<usize> {
        file_span(&self.source, &self.data_span)
    }
    pub fn original_payload(&self) -> &[u8] {
        &self.source.original_data()[self.data_span.clone()]
    }
    pub fn stacks(&self) -> &[StackEnvelope] {
        &self.stacks
    }
}

#[derive(Debug, Clone)]
pub struct StackEnvelope {
    source: Arc<RecordSource>,
    ordinal: usize,
    data_span: Range<usize>,
    extras: Vec<ExtraEnvelope>,
}

impl StackEnvelope {
    pub fn source(&self) -> &RecordSource {
        &self.source
    }
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
    pub fn record_data_span(&self) -> Range<usize> {
        self.data_span.clone()
    }
    pub fn file_span(&self) -> Range<usize> {
        file_span(&self.source, &self.data_span)
    }
    pub fn original_payload(&self) -> &[u8] {
        &self.source.original_data()[self.data_span.clone()]
    }
    pub fn extras(&self) -> &[ExtraEnvelope] {
        &self.extras
    }
}

#[derive(Debug, Clone)]
pub struct ExtraEnvelope {
    source: Arc<RecordSource>,
    ordinal: usize,
    data_span: Range<usize>,
    value: decode::Extra,
}

impl ExtraEnvelope {
    pub fn source(&self) -> &RecordSource {
        &self.source
    }
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
    pub fn record_data_span(&self) -> Range<usize> {
        self.data_span.clone()
    }
    pub fn file_span(&self) -> Range<usize> {
        file_span(&self.source, &self.data_span)
    }
    pub fn original_payload(&self) -> &[u8] {
        &self.source.original_data()[self.data_span.clone()]
    }
    pub fn decoded(&self) -> &decode::Extra {
        &self.value
    }
    pub fn kind(&self) -> u8 {
        self.value.kind()
    }
    /// The decoder knows its length but does not retain an interpreted payload.
    /// Original bytes remain available; this does not grant mutation support.
    pub fn is_opaque(&self) -> bool {
        matches!(&self.value, decode::Extra::Other(_))
    }
}

fn file_span(source: &RecordSource, relative: &Range<usize>) -> Range<usize> {
    // All private ranges were checked against the original file during admission.
    source.data_file_span.start + relative.start..source.data_file_span.start + relative.end
}

fn relative_span(source: &RecordSource, span: Range<usize>) -> Result<Range<usize>> {
    if span.start < source.data_file_span.start
        || span.start >= span.end
        || span.end > source.data_file_span.end
    {
        return Err(Error::new(
            source.data_file_span.start,
            "captured span outside record",
        ));
    }
    Ok(span.start - source.data_file_span.start..span.end - source.data_file_span.start)
}

fn nested_span(parent: &Range<usize>, previous_end: usize, span: &Range<usize>) -> Result<()> {
    if span.start < parent.start
        || span.start < previous_end
        || span.start >= span.end
        || span.end > parent.end
    {
        return Err(Error::new(
            span.start,
            "captured spans overlap or escape their parent",
        ));
    }
    Ok(())
}

fn envelopes(
    source: &Arc<RecordSource>,
    items: Vec<decode::ItemChange>,
    capture: InventoryCapture,
) -> Result<Vec<ItemEnvelope>> {
    if items.len() != capture.items.len() {
        return Err(Error::new(
            source.data_file_span.start,
            "item capture count mismatch",
        ));
    }
    let mut output = Vec::new();
    let record_span = 0..source.original_data().len();
    let mut previous_item = 0;
    for (ordinal, (item, spans)) in items.into_iter().zip(capture.items).enumerate() {
        let item_span = relative_span(source, spans.range)?;
        nested_span(&record_span, previous_item, &item_span)?;
        previous_item = item_span.end;
        if item.stacks.len() != spans.stacks.len() {
            return Err(Error::new(
                source.data_file_span.start,
                "stack capture count mismatch",
            ));
        }
        let mut stacks = Vec::new();
        let mut previous_stack = item_span.start;
        for (ordinal, (values, spans)) in item.stacks.into_iter().zip(spans.stacks).enumerate() {
            let stack_span = relative_span(source, spans.range)?;
            nested_span(&item_span, previous_stack, &stack_span)?;
            previous_stack = stack_span.end;
            if values.len() != spans.extras.len() {
                return Err(Error::new(
                    source.data_file_span.start,
                    "extra capture count mismatch",
                ));
            }
            let mut extras = Vec::new();
            let mut previous_extra = stack_span.start;
            for (ordinal, (value, span)) in values.into_iter().zip(spans.extras).enumerate() {
                let extra_span = relative_span(source, span)?;
                nested_span(&stack_span, previous_extra, &extra_span)?;
                previous_extra = extra_span.end;
                extras.push(ExtraEnvelope {
                    source: Arc::clone(source),
                    ordinal,
                    data_span: extra_span,
                    value,
                });
            }
            stacks.push(StackEnvelope {
                source: Arc::clone(source),
                ordinal,
                data_span: stack_span,
                extras,
            });
        }
        output.push(ItemEnvelope {
            source: Arc::clone(source),
            ordinal,
            identity: SavedIdentity::new(item.item, &source.save.form_ids),
            count_delta: item.count,
            data_span: item_span,
            stacks,
        });
    }
    Ok(output)
}

#[cfg(test)]
mod tests;
