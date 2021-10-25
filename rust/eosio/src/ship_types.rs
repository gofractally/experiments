use crate::types::*;
use eosio_proc_macros::EosioDeserialize;

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct StateHistoryLogHeader {
    pub magic: u64,
    pub block_id: Checksum256,
    pub payload_size: u64,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct GetStatusRequestV0 {}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct BlockPosition {
    pub block_num: u32,
    pub block_id: Checksum256,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct GetStatusResultV0 {
    pub head: BlockPosition,
    pub last_irreversible: BlockPosition,
    pub trace_begin_block: u32,
    pub trace_end_block: u32,
    pub chain_state_begin_block: u32,
    pub chain_state_end_block: u32,
    pub chain_id: BinaryExtension<Checksum256>, // nodeos v2.1
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct GetBlocksRequestV0 {
    pub start_block_num: u32,
    pub end_block_num: u32,
    pub max_messages_in_flight: u32,
    pub have_positions: Vec<BlockPosition>,
    pub irreversible_only: bool,
    pub fetch_block: bool,
    pub fetch_traces: bool,
    pub fetch_deltas: bool,
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub struct GetBlocksRequestV1 {
    pub start_block_num: u32,
    pub end_block_num: u32,
    pub max_messages_in_flight: u32,
    pub have_positions: Vec<BlockPosition>,
    pub irreversible_only: bool,
    pub fetch_block: bool,
    pub fetch_traces: bool,
    pub fetch_deltas: bool,
    pub fetch_block_header: bool,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct GetBlocksAckRequestV0 {
    pub num_messages: u32,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct GetBlocksResultV0<'a> {
    pub head: BlockPosition,
    pub last_irreversible: BlockPosition,
    pub this_block: Option<BlockPosition>,
    pub prev_block: Option<BlockPosition>,
    pub block: Option<Bytes<'a>>,
    pub traces: Option<Bytes<'a>>,
    pub deltas: Option<Bytes<'a>>,
}

// nodeos v2.1: in response to GetBlocksRequestV0
#[derive(Debug, EosioDeserialize)]
pub struct GetBlocksResultV1<'a> {
    pub head: BlockPosition,
    pub last_irreversible: BlockPosition,
    pub this_block: Option<BlockPosition>,
    pub prev_block: Option<BlockPosition>,
    pub block: Option<Box<SignedBlockVariant<'a>>>,
    pub traces: Bytes<'a>,
    pub deltas: Bytes<'a>,
}

// nodeos v2.1: in response to GetBlocksRequestV1
#[derive(Debug, EosioDeserialize)]
pub struct GetBlocksResultV2<'a> {
    pub head: BlockPosition,
    pub last_irreversible: BlockPosition,
    pub this_block: Option<BlockPosition>,
    pub prev_block: Option<BlockPosition>,
    pub block: Bytes<'a>,
    pub block_header: Bytes<'a>,
    pub traces: Bytes<'a>,
    pub deltas: Bytes<'a>,
}

// nodeos v2.0: didn't have version suffix
#[derive(Debug, EosioDeserialize)]
pub struct RowV0<'a> {
    pub present: bool,
    pub data: Bytes<'a>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct TableDeltaV0<'a> {
    pub name: Stringish<'a>,
    pub rows: Vec<RowV0<'a>>,
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub struct RowV1<'a> {
    pub present: u8,
    pub data: Bytes<'a>,
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub struct TableDeltaV1<'a> {
    pub name: Stringish<'a>,
    pub rows: Vec<RowV1<'a>>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct Action<'a> {
    pub account: Name,
    pub name: Name,
    pub authorization: Vec<PermissionLevel>,
    pub data: Bytes<'a>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct AccountAuthSequence {
    pub account: Name,
    pub sequence: u64,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ActionReceiptV0 {
    pub receiver: Name,
    pub act_digest: Checksum256,
    pub global_sequence: u64,
    pub recv_sequence: u64,
    pub auth_sequence: Vec<AccountAuthSequence>,
    pub code_sequence: Varuint32,
    pub abi_sequence: Varuint32,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct AccountDelta {
    pub account: Name,
    pub delta: i64,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ActionTraceV0<'a> {
    pub action_ordinal: Varuint32,
    pub creator_action_ordinal: Varuint32,
    pub receipt: Option<ActionReceipt>,
    pub receiver: Name,
    pub act: Action<'a>,
    pub context_free: bool,
    pub elapsed: i64,
    pub console: Stringish<'a>,
    pub account_ram_deltas: Vec<AccountDelta>,
    pub except: Option<Stringish<'a>>,
    pub error_code: Option<u64>,
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub struct ActionTraceV1<'a> {
    pub action_ordinal: Varuint32,
    pub creator_action_ordinal: Varuint32,
    pub receipt: Option<ActionReceipt>,
    pub receiver: Name,
    pub act: Action<'a>,
    pub context_free: bool,
    pub elapsed: i64,
    pub console: Stringish<'a>,
    pub account_ram_deltas: Vec<AccountDelta>,
    pub account_disk_deltas: Vec<AccountDelta>,
    pub except: Option<Stringish<'a>>,
    pub error_code: Option<u64>,
    pub return_value: Bytes<'a>,
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub struct PrunableDataNone {
    pub prunable_digest: Checksum256,
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub struct PrunableDataPartial<'a> {
    pub signatures: Vec<Signature<'a>>,
    pub context_free_segments: Vec<SegmentType<'a>>,
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub struct PrunableDataFull<'a> {
    pub signatures: Vec<Signature<'a>>,
    pub context_free_segments: Vec<Bytes<'a>>,
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub struct PrunableDataFullLegacy<'a> {
    pub signatures: Vec<Signature<'a>>,
    pub packed_context_free_data: Bytes<'a>,
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub struct PrunableDataStruct<'a> {
    pub prunable_data: PrunableDataVariant<'a>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct PartialTransactionV0<'a> {
    pub expiration: TimePointSec,
    pub ref_block_num: u16,
    pub ref_block_prefix: u32,
    pub max_net_usage_words: Varuint32,
    pub max_cpu_usage_ms: u8,
    pub delay_sec: Varuint32,
    pub transaction_extensions: Vec<Extension<'a>>,
    pub signatures: Vec<Signature<'a>>,
    pub context_free_data: Vec<Bytes<'a>>,
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub struct PartialTransactionV1<'a> {
    pub expiration: TimePointSec,
    pub ref_block_num: u16,
    pub ref_block_prefix: u32,
    pub max_net_usage_words: Varuint32,
    pub max_cpu_usage_ms: u8,
    pub delay_sec: Varuint32,
    pub transaction_extensions: Vec<Extension<'a>>,
    pub prunable_data: Option<PrunableDataStruct<'a>>,
}

#[derive(Debug, EosioDeserialize)]
pub struct TransactionTraceV0<'a> {
    pub id: Checksum256,
    pub status: u8,
    pub cpu_usage_us: u32,
    pub net_usage_words: Varuint32,
    pub elapsed: i64,
    pub net_usage: u64,
    pub scheduled: bool,
    pub action_traces: Vec<ActionTrace<'a>>,
    pub account_ram_delta: Option<AccountDelta>,
    pub except: Option<Stringish<'a>>,
    pub error_code: Option<u64>,
    pub failed_dtrx_trace: Option<Box<TransactionTrace<'a>>>,
    pub partial: Option<PartialTransaction<'a>>,
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub struct PackedTransaction<'a> {
    pub compression: u8,
    pub prunable_data: PrunableDataStruct<'a>,
    pub packed_trx: Bytes<'a>,
}

// nodeos v2.0: didn't have version suffix
#[derive(Debug, EosioDeserialize)]
pub struct PackedTransactionV0<'a> {
    pub signatures: Vec<Signature<'a>>,
    pub compression: u8,
    pub packed_context_free_data: Bytes<'a>,
    pub packed_trx: Bytes<'a>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct TransactionReceiptHeader {
    pub status: u8,
    pub cpu_usage_us: u32,
    pub net_usage_words: Varuint32,
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub struct TransactionReceipt<'a> {
    pub transaction_receipt_header: TransactionReceiptHeader,
    pub trx: TransactionVariant<'a>,
}

// nodeos v2.0: didn't have version suffix
#[derive(Debug, EosioDeserialize)]
pub struct TransactionReceiptV0<'a> {
    pub transaction_receipt_header: TransactionReceiptHeader,
    pub trx: TransactionVariantV0<'a>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct Extension<'a> {
    pub type_: u16,
    pub data: Bytes<'a>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct BlockHeader<'a> {
    pub timestamp: BlockTimestamp,
    pub producer: Name,
    pub confirmed: u16,
    pub previous: Checksum256,
    pub transaction_mroot: Checksum256,
    pub action_mroot: Checksum256,
    pub schedule_version: u32,
    pub new_producers: Option<ProducerSchedule<'a>>,
    pub header_extensions: Vec<Extension<'a>>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct SignedBlockHeader<'a> {
    pub block_header: BlockHeader<'a>,
    pub producer_signature: Signature<'a>,
}

// nodeos v2.0: didn't have version suffix
#[derive(Debug, EosioDeserialize)]
pub struct SignedBlockV0<'a> {
    pub signed_block_header: SignedBlockHeader<'a>,
    pub transactions: Vec<TransactionReceiptV0<'a>>,
    pub block_extensions: Vec<Extension<'a>>,
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub struct SignedBlockV1<'a> {
    pub signed_block_header: SignedBlockHeader<'a>,
    pub prune_state: u8,
    pub transactions: Vec<TransactionReceipt<'a>>,
    pub block_extensions: Vec<Extension<'a>>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct TransactionHeader {
    pub expiration: TimePointSec,
    pub ref_block_num: u16,
    pub ref_block_prefix: u32,
    pub max_net_usage_words: Varuint32,
    pub max_cpu_usage_ms: u8,
    pub delay_sec: Varuint32,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct Transaction<'a> {
    pub transaction_header: TransactionHeader,
    pub context_free_actions: Vec<Action<'a>>,
    pub actions: Vec<Action<'a>>,
    pub transaction_extensions: Vec<Extension<'a>>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct CodeId {
    pub vm_type: u8,
    pub vm_version: u8,
    pub code_hash: Checksum256,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct AccountV0<'a> {
    pub name: Name,
    pub creation_date: BlockTimestamp,
    pub abi: Bytes<'a>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct AccountMetadataV0 {
    pub name: Name,
    pub privileged: bool,
    pub last_code_update: TimePoint,
    pub code: Option<CodeId>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct CodeV0<'a> {
    pub vm_type: u8,
    pub vm_version: u8,
    pub code_hash: Checksum256,
    pub code: Bytes<'a>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ContractTableV0 {
    pub code: Name,
    pub scope: Name,
    pub table: Name,
    pub payer: Name,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ContractRowV0<'a> {
    pub code: Name,
    pub scope: Name,
    pub table: Name,
    pub primary_key: u64,
    pub payer: Name,
    pub value: Bytes<'a>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ContractIndex64V0 {
    pub code: Name,
    pub scope: Name,
    pub table: Name,
    pub primary_key: u64,
    pub payer: Name,
    pub secondary_key: u64,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ContractIndex128V0 {
    pub code: Name,
    pub scope: Name,
    pub table: Name,
    pub primary_key: u64,
    pub payer: Name,
    pub secondary_key: u128,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ContractIndex256V0 {
    pub code: Name,
    pub scope: Name,
    pub table: Name,
    pub primary_key: u64,
    pub payer: Name,
    pub secondary_key: Checksum256,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ContractIndexDoubleV0 {
    pub code: Name,
    pub scope: Name,
    pub table: Name,
    pub primary_key: u64,
    pub payer: Name,
    pub secondary_key: f64,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ContractIndexLongDoubleV0 {
    pub code: Name,
    pub scope: Name,
    pub table: Name,
    pub primary_key: u64,
    pub payer: Name,
    pub secondary_key: Float128,
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub struct KeyValueV0<'a> {
    pub contract: Name,
    pub key: Bytes<'a>,
    pub value: Bytes<'a>,
    pub payer: Name,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ProducerKey<'a> {
    pub producer_name: Name,
    pub block_signing_key: PublicKey<'a>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ProducerSchedule<'a> {
    pub version: u32,
    pub producers: Vec<ProducerKey<'a>>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct BlockSigningAuthorityV0<'a> {
    pub threshold: u32,
    pub keys: Vec<KeyWeight<'a>>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ProducerAuthority<'a> {
    pub producer_name: Name,
    pub authority: BlockSigningAuthority<'a>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ProducerAuthoritySchedule<'a> {
    pub version: u32,
    pub producers: Vec<ProducerAuthority<'a>>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ChainConfigV0 {
    pub max_block_net_usage: u64,
    pub target_block_net_usage_pct: u32,
    pub max_transaction_net_usage: u32,
    pub base_per_transaction_net_usage: u32,
    pub net_usage_leeway: u32,
    pub context_free_discount_net_usage_num: u32,
    pub context_free_discount_net_usage_den: u32,
    pub max_block_cpu_usage: u32,
    pub target_block_cpu_usage_pct: u32,
    pub max_transaction_cpu_usage: u32,
    pub min_transaction_cpu_usage: u32,
    pub max_transaction_lifetime: u32,
    pub deferred_trx_expiration_window: u32,
    pub max_transaction_delay: u32,
    pub max_inline_action_size: u32,
    pub max_inline_action_depth: u16,
    pub max_authority_depth: u16,
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub struct ChainConfigV1 {
    pub max_block_net_usage: u64,
    pub target_block_net_usage_pct: u32,
    pub max_transaction_net_usage: u32,
    pub base_per_transaction_net_usage: u32,
    pub net_usage_leeway: u32,
    pub context_free_discount_net_usage_num: u32,
    pub context_free_discount_net_usage_den: u32,
    pub max_block_cpu_usage: u32,
    pub target_block_cpu_usage_pct: u32,
    pub max_transaction_cpu_usage: u32,
    pub min_transaction_cpu_usage: u32,
    pub max_transaction_lifetime: u32,
    pub deferred_trx_expiration_window: u32,
    pub max_transaction_delay: u32,
    pub max_inline_action_size: u32,
    pub max_inline_action_depth: u16,
    pub max_authority_depth: u16,
    pub max_action_return_value_size: u32,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct GlobalPropertyV0<'a> {
    pub proposed_schedule_block_num: Option<u32>,
    pub proposed_schedule: ProducerSchedule<'a>,
    pub configuration: ChainConfig,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct GlobalPropertyV1<'a> {
    pub proposed_schedule_block_num: Option<u32>,
    pub proposed_schedule: ProducerAuthoritySchedule<'a>,
    pub configuration: ChainConfig,
    pub chain_id: Checksum256,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct GeneratedTransactionV0<'a> {
    pub sender: Name,
    pub sender_id: u128,
    pub payer: Name,
    pub trx_id: Checksum256,
    pub packed_trx: Bytes<'a>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ActivatedProtocolFeatureV0 {
    pub feature_digest: Checksum256,
    pub activation_block_num: u32,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ProtocolStateV0 {
    pub activated_protocol_features: Vec<ActivatedProtocolFeature>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct KeyWeight<'a> {
    pub key: PublicKey<'a>,
    pub weight: u16,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct PermissionLevel {
    pub actor: Name,
    pub permission: Name,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct PermissionLevelWeight {
    pub permission: PermissionLevel,
    pub weight: u16,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct WaitWeight {
    pub wait_sec: u32,
    pub weight: u16,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct Authority<'a> {
    pub threshold: u32,
    pub keys: Vec<KeyWeight<'a>>,
    pub accounts: Vec<PermissionLevelWeight>,
    pub waits: Vec<WaitWeight>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct PermissionV0<'a> {
    pub owner: Name,
    pub name: Name,
    pub parent: Name,
    pub last_updated: TimePoint,
    pub auth: Authority<'a>,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct PermissionLinkV0 {
    pub account: Name,
    pub code: Name,
    pub message_type: Name,
    pub required_permission: Name,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ResourceLimitsV0 {
    pub owner: Name,
    pub net_weight: i64,
    pub cpu_weight: i64,
    pub ram_bytes: i64,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct UsageAccumulatorV0 {
    pub last_ordinal: u32,
    pub value_ex: u64,
    pub consumed: u64,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ResourceUsageV0 {
    pub owner: Name,
    pub net_usage: UsageAccumulator,
    pub cpu_usage: UsageAccumulator,
    pub ram_usage: u64,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ResourceLimitsStateV0 {
    pub average_block_net_usage: UsageAccumulator,
    pub average_block_cpu_usage: UsageAccumulator,
    pub total_net_weight: u64,
    pub total_cpu_weight: u64,
    pub total_ram_bytes: u64,
    pub virtual_net_limit: u64,
    pub virtual_cpu_limit: u64,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ResourceLimitsRatioV0 {
    pub numerator: u64,
    pub denominator: u64,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ElasticLimitParametersV0 {
    pub target: u64,
    pub max: u64,
    pub periods: u32,
    pub max_multiplier: u32,
    pub contract_rate: ResourceLimitsRatio,
    pub expand_rate: ResourceLimitsRatio,
}

// nodeos v2.0
#[derive(Debug, EosioDeserialize)]
pub struct ResourceLimitsConfigV0 {
    pub cpu_limit_parameters: ElasticLimitParameters,
    pub net_limit_parameters: ElasticLimitParameters,
    pub account_cpu_usage_average_window: u32,
    pub account_net_usage_average_window: u32,
}

type TransactionId = Checksum256;

#[derive(Debug, EosioDeserialize)]
pub enum Request {
    GetStatusRequestV0(GetStatusRequestV0),
    GetBlocksRequestV0(GetBlocksRequestV0),
    GetBlocksAckRequestV0(GetBlocksAckRequestV0),
    GetBlocksRequestV1(GetBlocksRequestV1), // nodeos v2.1
}

#[derive(Debug, EosioDeserialize)]
pub enum Result<'a> {
    GetStatusResultV0(GetStatusResultV0),
    GetBlocksResultV0(GetBlocksResultV0<'a>),
    GetBlocksResultV1(GetBlocksResultV1<'a>), // nodeos v2.1
    GetBlocksResultV2(GetBlocksResultV2<'a>), // nodeos v2.1
}

#[derive(Debug, EosioDeserialize)]
pub enum ActionReceipt {
    ActionReceiptV0(ActionReceiptV0),
}

#[derive(Debug, EosioDeserialize)]
pub enum ActionTrace<'a> {
    ActionTraceV0(ActionTraceV0<'a>),
    ActionTraceV1(ActionTraceV1<'a>), // nodeos v2.1
}

#[derive(Debug, EosioDeserialize)]
pub enum PartialTransaction<'a> {
    PartialTransactionV0(PartialTransactionV0<'a>),
    PartialTransactionV1(PartialTransactionV1<'a>), // nodeos v2.1
}

#[derive(Debug, EosioDeserialize)]
pub enum TransactionTrace<'a> {
    TransactionTraceV0(TransactionTraceV0<'a>),
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub enum TransactionVariant<'a> {
    Checksum256(Checksum256),
    PackedTransaction(PackedTransaction<'a>),
}

// nodeos v2.0: didn't have version suffix
#[derive(Debug, EosioDeserialize)]
pub enum TransactionVariantV0<'a> {
    TransactionId(TransactionId),
    PackedTransactionV0(PackedTransactionV0<'a>),
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub enum SignedBlockVariant<'a> {
    SignedBlockV0(SignedBlockV0<'a>),
    SignedBlockV1(SignedBlockV1<'a>),
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub enum SegmentType<'a> {
    Checksum256(Checksum256),
    Bytes(Bytes<'a>),
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub enum PrunableDataVariant<'a> {
    PrunableDataFullLegacy(PrunableDataFullLegacy<'a>),
    PrunableDataNone(PrunableDataNone),
    PrunableDataPartial(PrunableDataPartial<'a>),
    PrunableDataFull(PrunableDataFull<'a>),
}

#[derive(Debug, EosioDeserialize)]
pub enum TableDelta<'a> {
    TableDeltaV0(TableDeltaV0<'a>),
    TableDeltaV1(TableDeltaV1<'a>), // nodeos v2.1
}

#[derive(Debug, EosioDeserialize)]
pub enum Account<'a> {
    AccountV0(AccountV0<'a>),
}

#[derive(Debug, EosioDeserialize)]
pub enum AccountMetadata {
    AccountMetadataV0(AccountMetadataV0),
}

#[derive(Debug, EosioDeserialize)]
pub enum Code<'a> {
    CodeV0(CodeV0<'a>),
}

#[derive(Debug, EosioDeserialize)]
pub enum ContractTable {
    ContractTableV0(ContractTableV0),
}

#[derive(Debug, EosioDeserialize)]
pub enum ContractRow<'a> {
    ContractRowV0(ContractRowV0<'a>),
}

#[derive(Debug, EosioDeserialize)]
pub enum ContractIndex64 {
    ContractIndex64V0(ContractIndex64V0),
}

#[derive(Debug, EosioDeserialize)]
pub enum ContractIndex128 {
    ContractIndex128V0(ContractIndex128V0),
}

#[derive(Debug, EosioDeserialize)]
pub enum ContractIndex256 {
    ContractIndex256V0(ContractIndex256V0),
}

#[derive(Debug, EosioDeserialize)]
pub enum ContractIndexDouble {
    ContractIndexDoubleV0(ContractIndexDoubleV0),
}

#[derive(Debug, EosioDeserialize)]
pub enum ContractIndexLongDouble {
    ContractIndexLongDoubleV0(ContractIndexLongDoubleV0),
}

// nodeos v2.1
#[derive(Debug, EosioDeserialize)]
pub enum KeyValue<'a> {
    KeyValueV0(KeyValueV0<'a>),
}

#[derive(Debug, EosioDeserialize)]
pub enum ChainConfig {
    ChainConfigV0(ChainConfigV0),
    ChainConfigV1(ChainConfigV1), // nodeos v2.1
}

#[derive(Debug, EosioDeserialize)]
pub enum GlobalProperty<'a> {
    GlobalPropertyV0(GlobalPropertyV0<'a>),
    GlobalPropertyV1(GlobalPropertyV1<'a>),
}

#[derive(Debug, EosioDeserialize)]
pub enum GeneratedTransaction<'a> {
    GeneratedTransactionV0(GeneratedTransactionV0<'a>),
}

#[derive(Debug, EosioDeserialize)]
pub enum ActivatedProtocolFeature {
    ActivatedProtocolFeatureV0(ActivatedProtocolFeatureV0),
}

#[derive(Debug, EosioDeserialize)]
pub enum ProtocolState {
    ProtocolStateV0(ProtocolStateV0),
}

#[derive(Debug, EosioDeserialize)]
pub enum Permission<'a> {
    PermissionV0(PermissionV0<'a>),
}

#[derive(Debug, EosioDeserialize)]
pub enum PermissionLink {
    PermissionLinkV0(PermissionLinkV0),
}

#[derive(Debug, EosioDeserialize)]
pub enum ResourceLimits {
    ResourceLimitsV0(ResourceLimitsV0),
}

#[derive(Debug, EosioDeserialize)]
pub enum UsageAccumulator {
    UsageAccumulatorV0(UsageAccumulatorV0),
}

#[derive(Debug, EosioDeserialize)]
pub enum ResourceUsage {
    ResourceUsageV0(ResourceUsageV0),
}

#[derive(Debug, EosioDeserialize)]
pub enum ResourceLimitsState {
    ResourceLimitsStateV0(ResourceLimitsStateV0),
}

#[derive(Debug, EosioDeserialize)]
pub enum ResourceLimitsRatio {
    ResourceLimitsRatioV0(ResourceLimitsRatioV0),
}

#[derive(Debug, EosioDeserialize)]
pub enum ElasticLimitParameters {
    ElasticLimitParametersV0(ElasticLimitParametersV0),
}

#[derive(Debug, EosioDeserialize)]
pub enum ResourceLimitsConfig {
    ResourceLimitsConfigV0(ResourceLimitsConfigV0),
}

#[derive(Debug, EosioDeserialize)]
pub enum BlockSigningAuthority<'a> {
    BlockSigningAuthorityV0(BlockSigningAuthorityV0<'a>),
}

/*
    "tables": [
        { "name": "account", "type": "account", "key_names": ["name"] },
        { "name": "account_metadata", "type": "account_metadata", "key_names": ["name"] },
        { "name": "code", "type": "code", "key_names": ["vm_type", "vm_version", "code_hash"] },
        { "name": "contract_table", "type": "contract_table", "key_names": ["code", "scope", "table"] },
        { "name": "contract_row", "type": "contract_row", "key_names": ["code", "scope", "table", "primary_key"] },
        { "name": "contract_index64", "type": "contract_index64", "key_names": ["code", "scope", "table", "primary_key"] },
        { "name": "contract_index128", "type": "contract_index128", "key_names": ["code", "scope", "table", "primary_key"] },
        { "name": "contract_index256", "type": "contract_index256", "key_names": ["code", "scope", "table", "primary_key"] },
        { "name": "contract_index_double", "type": "contract_index_double", "key_names": ["code", "scope", "table", "primary_key"] },
        { "name": "contract_index_long_double", "type": "contract_index_long_double", "key_names": ["code", "scope", "table", "primary_key"] },
        { "name": "key_value", "type": "key_value", "key_names": ["contract", "key"] },
        { "name": "global_property", "type": "global_property", "key_names": [] },
        { "name": "generated_transaction", "type": "generated_transaction", "key_names": ["sender", "sender_id"] },
        { "name": "protocol_state", "type": "protocol_state", "key_names": [] },
        { "name": "permission", "type": "permission", "key_names": ["owner", "name"] },
        { "name": "permission_link", "type": "permission_link", "key_names": ["account", "code", "message_type"] },
        { "name": "resource_limits", "type": "resource_limits", "key_names": ["owner"] },
        { "name": "resource_usage", "type": "resource_usage", "key_names": ["owner"] },
        { "name": "resource_limits_state", "type": "resource_limits_state", "key_names": [] },
        { "name": "resource_limits_config", "type": "resource_limits_config", "key_names": [] }
    ]
})";
*/
