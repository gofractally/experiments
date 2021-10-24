use crate::{derive_eosio_deserialize, eosio_types::*};

derive_eosio_deserialize! {
    pub struct StateHistoryLogHeader {
        pub magic: u64,
        pub block_id: Checksum256,
        pub payload_size: u64,
    }
}

derive_eosio_deserialize! {
    pub struct GetStatusRequestV0 {}
}

derive_eosio_deserialize! {
    pub struct BlockPosition {
        pub block_num: u32,
        pub block_id: Checksum256,
    }
}

derive_eosio_deserialize! {
    pub struct GetStatusResultV0 {
        pub head: BlockPosition,
        pub last_irreversible: BlockPosition,
        pub trace_begin_block: u32,
        pub trace_end_block: u32,
        pub chain_state_begin_block: u32,
        pub chain_state_end_block: u32,
    }
}

derive_eosio_deserialize! {
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
}

derive_eosio_deserialize! {
    pub struct GetBlocksAckRequestV0 {
        pub num_messages: u32,
    }
}

derive_eosio_deserialize! {
    pub struct GetBlocksResultV0<'a> {
        pub head: BlockPosition,
        pub last_irreversible: BlockPosition,
        pub this_block: Option<BlockPosition>,
        pub prev_block: Option<BlockPosition>,
        pub block: Option<Bytes<'a>>,
        pub traces: Option<Bytes<'a>>,
        pub deltas: Option<Bytes<'a>>,
    }
}

derive_eosio_deserialize! {
    pub struct Row<'a> {
        pub present: bool,
        pub data: Bytes<'a>,
    }
}

derive_eosio_deserialize! {
    pub struct TableDeltaV0<'a> {
        pub name: Stringish<'a>,
        pub rows: Vec<Row<'a>>,
    }
}

derive_eosio_deserialize! {
    pub struct Action<'a> {
        pub account: Name,
        pub name: Name,
        pub authorization: Vec<PermissionLevel>,
        pub data: Bytes<'a>,
    }
}

derive_eosio_deserialize! {
    pub struct AccountAuthSequence {
        pub account: Name,
        pub sequence: u64,
    }
}

derive_eosio_deserialize! {
    pub struct ActionReceiptV0 {
        pub receiver: Name,
        pub act_digest: Checksum256,
        pub global_sequence: u64,
        pub recv_sequence: u64,
        pub auth_sequence: Vec<AccountAuthSequence>,
        pub code_sequence: Varuint32,
        pub abi_sequence: Varuint32,
    }
}

derive_eosio_deserialize! {
    pub struct AccountDelta {
        pub account: Name,
        pub delta: i64,
    }
}

derive_eosio_deserialize! {
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
}

derive_eosio_deserialize! {
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
}

derive_eosio_deserialize! {
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
}

derive_eosio_deserialize! {
    pub struct PackedTransaction<'a> {
        pub signatures: Vec<Signature<'a>>,
        pub compression: u8,
        pub packed_context_free_data: Bytes<'a>,
        pub packed_trx: Bytes<'a>,
    }
}

derive_eosio_deserialize! {
    pub struct TransactionReceiptHeader {
        pub status: u8,
        pub cpu_usage_us: u32,
        pub net_usage_words: Varuint32,
    }
}

derive_eosio_deserialize! {
    pub struct TransactionReceipt<'a> {
        pub transaction_receipt_header: TransactionReceiptHeader,
        pub trx: TransactionVariant<'a>,
    }
}

derive_eosio_deserialize! {
    pub struct Extension<'a> {
        pub type_: u16,
        pub data: Bytes<'a>,
    }
}

derive_eosio_deserialize! {
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
}

derive_eosio_deserialize! {
    pub struct SignedBlockHeader<'a> {
        pub block_header: BlockHeader<'a>,
        pub producer_signature: Signature<'a>,
    }
}

derive_eosio_deserialize! {
    pub struct SignedBlock<'a> {
        pub signed_block_header: SignedBlockHeader<'a>,
        pub transactions: Vec<TransactionReceipt<'a>>,
        pub block_extensions: Vec<Extension<'a>>,
    }
}

derive_eosio_deserialize! {
    pub struct TransactionHeader {
        pub expiration: TimePointSec,
        pub ref_block_num: u16,
        pub ref_block_prefix: u32,
        pub max_net_usage_words: Varuint32,
        pub max_cpu_usage_ms: u8,
        pub delay_sec: Varuint32,
    }
}

derive_eosio_deserialize! {
    pub struct Transaction<'a> {
        pub transaction_header: TransactionHeader,
        pub context_free_actions: Vec<Action<'a>>,
        pub actions: Vec<Action<'a>>,
        pub transaction_extensions: Vec<Extension<'a>>,
    }
}

derive_eosio_deserialize! {
    pub struct CodeId {
        pub vm_type: u8,
        pub vm_version: u8,
        pub code_hash: Checksum256,
    }
}

derive_eosio_deserialize! {
    pub struct AccountV0<'a> {
        pub name: Name,
        pub creation_date: BlockTimestamp,
        pub abi: Bytes<'a>,
    }
}

derive_eosio_deserialize! {
    pub struct AccountMetadataV0 {
        pub name: Name,
        pub privileged: bool,
        pub last_code_update: TimePoint,
        pub code: Option<CodeId>,
    }
}

derive_eosio_deserialize! {
    pub struct CodeV0<'a> {
        pub vm_type: u8,
        pub vm_version: u8,
        pub code_hash: Checksum256,
        pub code: Bytes<'a>,
    }
}

derive_eosio_deserialize! {
    pub struct ContractTableV0 {
        pub code: Name,
        pub scope: Name,
        pub table: Name,
        pub payer: Name,
    }
}

derive_eosio_deserialize! {
    pub struct ContractRowV0<'a> {
        pub code: Name,
        pub scope: Name,
        pub table: Name,
        pub primary_key: u64,
        pub payer: Name,
        pub value: Bytes<'a>,
    }
}

derive_eosio_deserialize! {
    pub struct ContractIndex64V0 {
        pub code: Name,
        pub scope: Name,
        pub table: Name,
        pub primary_key: u64,
        pub payer: Name,
        pub secondary_key: u64,
    }
}

derive_eosio_deserialize! {
    pub struct ContractIndex128V0 {
        pub code: Name,
        pub scope: Name,
        pub table: Name,
        pub primary_key: u64,
        pub payer: Name,
        pub secondary_key: u128,
    }
}

derive_eosio_deserialize! {
    pub struct ContractIndex256V0 {
        pub code: Name,
        pub scope: Name,
        pub table: Name,
        pub primary_key: u64,
        pub payer: Name,
        pub secondary_key: Checksum256,
    }
}

derive_eosio_deserialize! {
    pub struct ContractIndexDoubleV0 {
        pub code: Name,
        pub scope: Name,
        pub table: Name,
        pub primary_key: u64,
        pub payer: Name,
        pub secondary_key: f64,
    }
}

derive_eosio_deserialize! {
    pub struct ContractIndexLongDoubleV0 {
        pub code: Name,
        pub scope: Name,
        pub table: Name,
        pub primary_key: u64,
        pub payer: Name,
        pub secondary_key: Float128,
    }
}

derive_eosio_deserialize! {
    pub struct ProducerKey<'a> {
        pub producer_name: Name,
        pub block_signing_key: PublicKey<'a>,
    }
}

derive_eosio_deserialize! {
    pub struct ProducerSchedule<'a> {
        pub version: u32,
        pub producers: Vec<ProducerKey<'a>>,
    }
}

derive_eosio_deserialize! {
    pub struct BlockSigningAuthorityV0<'a> {
        pub threshold: u32,
        pub keys: Vec<KeyWeight<'a>>,
    }
}

derive_eosio_deserialize! {
    pub struct ProducerAuthority<'a> {
        pub producer_name: Name,
        pub authority: BlockSigningAuthority<'a>,
    }
}

derive_eosio_deserialize! {
    pub struct ProducerAuthoritySchedule<'a> {
        pub version: u32,
        pub producers: Vec<ProducerAuthority<'a>>,
    }
}

derive_eosio_deserialize! {
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
}

derive_eosio_deserialize! {
    pub struct GlobalPropertyV0<'a> {
        pub proposed_schedule_block_num: Option<u32>,
        pub proposed_schedule: ProducerSchedule<'a>,
        pub configuration: ChainConfig,
    }
}

derive_eosio_deserialize! {
    pub struct GlobalPropertyV1<'a> {
        pub proposed_schedule_block_num: Option<u32>,
        pub proposed_schedule: ProducerAuthoritySchedule<'a>,
        pub configuration: ChainConfig,
        pub chain_id: Checksum256,
    }
}

derive_eosio_deserialize! {
    pub struct GeneratedTransactionV0<'a> {
        pub sender: Name,
        pub sender_id: u128,
        pub payer: Name,
        pub trx_id: Checksum256,
        pub packed_trx: Bytes<'a>,
    }
}

derive_eosio_deserialize! {
    pub struct ActivatedProtocolFeatureV0 {
        pub feature_digest: Checksum256,
        pub activation_block_num: u32,
    }
}

derive_eosio_deserialize! {
    pub struct ProtocolStateV0 {
        pub activated_protocol_features: Vec<ActivatedProtocolFeature>,
    }
}

derive_eosio_deserialize! {
    pub struct KeyWeight<'a> {
        pub key: PublicKey<'a>,
        pub weight: u16,
    }
}

derive_eosio_deserialize! {
    pub struct PermissionLevel {
        pub actor: Name,
        pub permission: Name,
    }
}

derive_eosio_deserialize! {
    pub struct PermissionLevelWeight {
        pub permission: PermissionLevel,
        pub weight: u16,
    }
}

derive_eosio_deserialize! {
    pub struct WaitWeight {
        pub wait_sec: u32,
        pub weight: u16,
    }
}

derive_eosio_deserialize! {
    pub struct Authority<'a> {
        pub threshold: u32,
        pub keys: Vec<KeyWeight<'a>>,
        pub accounts: Vec<PermissionLevelWeight>,
        pub waits: Vec<WaitWeight>,
    }
}

derive_eosio_deserialize! {
    pub struct PermissionV0<'a> {
        pub owner: Name,
        pub name: Name,
        pub parent: Name,
        pub last_updated: TimePoint,
        pub auth: Authority<'a>,
    }
}

derive_eosio_deserialize! {
    pub struct PermissionLinkV0 {
        pub account: Name,
        pub code: Name,
        pub message_type: Name,
        pub required_permission: Name,
    }
}

derive_eosio_deserialize! {
    pub struct ResourceLimitsV0 {
        pub owner: Name,
        pub net_weight: i64,
        pub cpu_weight: i64,
        pub ram_bytes: i64,
    }
}

derive_eosio_deserialize! {
    pub struct UsageAccumulatorV0 {
        pub last_ordinal: u32,
        pub value_ex: u64,
        pub consumed: u64,
    }
}

derive_eosio_deserialize! {
    pub struct ResourceUsageV0 {
        pub owner: Name,
        pub net_usage: UsageAccumulator,
        pub cpu_usage: UsageAccumulator,
        pub ram_usage: u64,
    }
}

derive_eosio_deserialize! {
    pub struct ResourceLimitsStateV0 {
        pub average_block_net_usage: UsageAccumulator,
        pub average_block_cpu_usage: UsageAccumulator,
        pub total_net_weight: u64,
        pub total_cpu_weight: u64,
        pub total_ram_bytes: u64,
        pub virtual_net_limit: u64,
        pub virtual_cpu_limit: u64,
    }
}

derive_eosio_deserialize! {
    pub struct ResourceLimitsRatioV0 {
        pub numerator: u64,
        pub denominator: u64,
    }
}

derive_eosio_deserialize! {
    pub struct ElasticLimitParametersV0 {
        pub target: u64,
        pub max: u64,
        pub periods: u32,
        pub max_multiplier: u32,
        pub contract_rate: ResourceLimitsRatio,
        pub expand_rate: ResourceLimitsRatio,
    }
}

derive_eosio_deserialize! {
    pub struct ResourceLimitsConfigV0 {
        pub cpu_limit_parameters: ElasticLimitParameters,
        pub net_limit_parameters: ElasticLimitParameters,
        pub account_cpu_usage_average_window: u32,
        pub account_net_usage_average_window: u32,
    }
}

type TransactionId = Checksum256;

derive_eosio_deserialize! {
    pub enum Request {
        GetStatusRequestV0(GetStatusRequestV0),
        GetBlocksRequestV0(GetBlocksRequestV0),
        GetBlocksAckRequestV0(GetBlocksAckRequestV0),
    }
}

derive_eosio_deserialize! {
    pub enum Result<'a> {
        GetStatusResultV0(GetStatusResultV0),
        GetBlocksResultV0(GetBlocksResultV0<'a>),
    }
}

derive_eosio_deserialize! {
    pub enum ActionReceipt {
        ActionReceiptV0(ActionReceiptV0),
    }
}

derive_eosio_deserialize! {
    pub enum ActionTrace<'a> {
        ActionTraceV0(ActionTraceV0<'a>),
    }
}

derive_eosio_deserialize! {
    pub enum PartialTransaction<'a> {
        PartialTransactionV0(PartialTransactionV0<'a>),
    }
}

derive_eosio_deserialize! {
    pub enum TransactionTrace<'a> {
        TransactionTraceV0(TransactionTraceV0<'a>),
    }
}

derive_eosio_deserialize! {
    pub enum TransactionVariant<'a> {
        TransactionId(TransactionId),
        PackedTransaction(PackedTransaction<'a>),
    }
}

derive_eosio_deserialize! {
    pub enum TableDelta<'a> {
        TableDeltaV0(TableDeltaV0<'a>),
    }
}

derive_eosio_deserialize! {
    pub enum Account<'a> {
        AccountV0(AccountV0<'a>),
    }
}

derive_eosio_deserialize! {
    pub enum AccountMetadata {
        AccountMetadataV0(AccountMetadataV0),
    }
}

derive_eosio_deserialize! {
    pub enum Code<'a> {
        CodeV0(CodeV0<'a>),
    }
}

derive_eosio_deserialize! {
    pub enum ContractTable {
        ContractTableV0(ContractTableV0),
    }
}

derive_eosio_deserialize! {
    pub enum ContractRow<'a> {
        ContractRowV0(ContractRowV0<'a>),
    }
}

derive_eosio_deserialize! {
    pub enum ContractIndex64 {
        ContractIndex64V0(ContractIndex64V0),
    }
}

derive_eosio_deserialize! {
    pub enum ContractIndex128 {
        ContractIndex128V0(ContractIndex128V0),
    }
}

derive_eosio_deserialize! {
    pub enum ContractIndex256 {
        ContractIndex256V0(ContractIndex256V0),
    }
}

derive_eosio_deserialize! {
    pub enum ContractIndexDouble {
        ContractIndexDoubleV0(ContractIndexDoubleV0),
    }
}

derive_eosio_deserialize! {
    pub enum ContractIndexLongDouble {
        ContractIndexLongDoubleV0(ContractIndexLongDoubleV0),
    }
}

derive_eosio_deserialize! {
    pub enum ChainConfig {
        ChainConfigV0(ChainConfigV0),
    }
}

derive_eosio_deserialize! {
    pub enum GlobalProperty<'a> {
        GlobalPropertyV0(GlobalPropertyV0<'a>),
        GlobalPropertyV1(GlobalPropertyV1<'a>),
    }
}

derive_eosio_deserialize! {
    pub enum GeneratedTransaction<'a> {
        GeneratedTransactionV0(GeneratedTransactionV0<'a>),
    }
}

derive_eosio_deserialize! {
    pub enum ActivatedProtocolFeature {
        ActivatedProtocolFeatureV0(ActivatedProtocolFeatureV0),
    }
}

derive_eosio_deserialize! {
    pub enum ProtocolState {
        ProtocolStateV0(ProtocolStateV0),
    }
}

derive_eosio_deserialize! {
    pub enum Permission<'a> {
        PermissionV0(PermissionV0<'a>),
    }
}

derive_eosio_deserialize! {
    pub enum PermissionLink {
        PermissionLinkV0(PermissionLinkV0),
    }
}

derive_eosio_deserialize! {
    pub enum ResourceLimits {
        ResourceLimitsV0(ResourceLimitsV0),
    }
}

derive_eosio_deserialize! {
    pub enum UsageAccumulator {
        UsageAccumulatorV0(UsageAccumulatorV0),
    }
}

derive_eosio_deserialize! {
    pub enum ResourceUsage {
        ResourceUsageV0(ResourceUsageV0),
    }
}

derive_eosio_deserialize! {
    pub enum ResourceLimitsState {
        ResourceLimitsStateV0(ResourceLimitsStateV0),
    }
}

derive_eosio_deserialize! {
    pub enum ResourceLimitsRatio {
        ResourceLimitsRatioV0(ResourceLimitsRatioV0),
    }
}

derive_eosio_deserialize! {
    pub enum ElasticLimitParameters {
        ElasticLimitParametersV0(ElasticLimitParametersV0),
    }
}

derive_eosio_deserialize! {
    pub enum ResourceLimitsConfig {
        ResourceLimitsConfigV0(ResourceLimitsConfigV0),
    }
}

derive_eosio_deserialize! {
    pub enum BlockSigningAuthority<'a> {
        BlockSigningAuthorityV0(BlockSigningAuthorityV0<'a>),
    }
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
