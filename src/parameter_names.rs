//******************* */
//Sampler Parameters //
//******************* */

/// Seed: the seed used to initialize llama_sampler
/// A u32 number.
pub const SAMPLER_SEED: &str = "seed";

/// Penalty-related constants

/// How many recent tokens to consider when applying repetition penalties.
/// (0 = disable penalty, -1 = context size)
/// - Example: `64`
/// - Default: `64`
pub const SAMPLER_PENALTY_LAST_N: &str = "penalty_last_n";

/// Penalty factor for repeated tokens (discourages repetition).
/// Typically a value less than 1 reduces repetition probability.
/// 1.0 = disabled
/// - Example: `0.8`
/// - Default: `0.8`
pub const SAMPLER_PENALTY_REPEAT: &str = "penalty_repeat";

/// Frequency penalty — reduces likelihood of frequently used tokens.
/// Penalizes tokens proportional to their frequency in the context.
/// 0.0 = disabled
/// - Example: `0.5`
/// - Default: `0.5`
pub const SAMPLER_PENALTY_FREQ: &str = "penalty_freq";

/// Presence penalty — discourages repeating any previously used token.
/// Penalizes tokens simply if they appeared before, regardless of count.
/// 0.0 = disabled
/// - Example: `0.6`
/// - Default: `0.6`
pub const SAMPLER_PENALTY_PRESENT: &str = "penalty_present";


/// Dry-related constants (custom or experimental sampling logic)

/// Multiplier for reducing probabilities in dry-run scenarios.
/// Used to scale probabilities during dry runs.
/// - Example: `0.9`
/// - Default: `0.9`
pub const SAMPLER_DRY_MULTIPLIER: &str = "dry_multiplier";

/// Base probability scaling factor in dry sampling mode.
/// Serves as a baseline probability multiplier.
/// - Example: `0.1`
/// - Default: `0.1`
pub const SAMPLER_DRY_BASE: &str = "dry_base";

/// Maximum length allowed in dry sampling mode.
/// Limits the length during dry runs.
/// - Example: `128`
/// - Default: `128`
pub const SAMPLER_DRY_ALLOWED_LENGTH: &str = "dry_allowed_length";

/// Last N tokens considered for dry-mode repetition penalty.
/// Defines the recent token window for dry-mode penalty.
/// - Example: `32`
/// - Default: `32`
pub const SAMPLER_DRY_PENALTY_LAST_N: &str = "dry_penalty_last_n";


/// Sampling-related: Top-k, Top-p, and Temperature sampling

/// Top-K sampling — consider only the top K highest tokens (probab).
/// <= 0 to use vocab size
/// - Example: `40`
/// - Default: `40`
pub const SAMPLER_TOP_K: &str = "top_k";

/// Top-P (nucleus) sampling — keep top tokens that cover P cumulative probability.
/// 1.0 = disabled
/// - Example: `0.9`
/// - Default: `0.95`
pub const SAMPLER_TOP_P: &str = "top_p";

/// Minimum probability threshold — tokens below this are filtered out.
/// 0.0 = disabled
/// - Example: `0.01`
/// - Default: `0.01`
pub const SAMPLER_MIN_P: &str = "min_p";

/// Typical p sampling — prefers tokens close to typical conditional probability.
/// 1.0 = disabled
/// - Example: `0.95`
/// - Default: `0.95`
pub const SAMPLER_TYP_P: &str = "typ_p";

/// Temperature — controls randomness; lower is more deterministic, higher is more random.
/// higher is more creative, lower is more coherent / factual
/// <= 0.0 to sample greedily, 0.0 to not output probabilities
/// - Example: `0.4`
/// - Default: `0.8`
pub const SAMPLER_TEMP: &str = "temperature";

/// Advanced sampling configuration

/// Custom sampling algorithm probability cutoff (e.g., XTC strategy).
/// Used to determine probability threshold for specialized sampling methods.
/// 0.0 = disabled
/// - Example: `0.75`
/// - Default: `0.75`
pub const SAMPLER_XTC_PROBABILITY: &str = "xtc_probability";

/// Threshold used in XTC-style filtering or boosting.
/// Helps fine-tune when to apply XTC filtering or boost token probabilities.
/// > 0.5 disables XTC
/// - Example: `0.1`
/// - Default: `0.1`
pub const SAMPLER_XTC_THRESHOLD: &str = "xtc_threshold";

/// Top-N sampling with Gaussian sigma scaling (alternative to Top-K).
/// Controls spread (sigma) for Gaussian weighting of Top-N tokens.
/// -1.0 = disabled
/// - Example: `1.0`
/// - Default: `1.0`
pub const SAMPLER_TOP_N_SIGMA: &str = "top_n_sigma";

/// Mirostat sampling (adaptive control over entropy)

/// Enable or disable Mirostat sampling algorithm.
/// Toggles adaptive sampling that targets specific entropy levels.
/// Top K, Nucleus and Locally Typical samplers are ignored if used
/// default: 0 = disabled, 1 = Mirostat, 2 = Mirostat 2.0
/// - Example: 2
/// - Default: 0
pub const SAMPLER_MIROSTAT: &str = "mirostat";

/// Learning rate for Mirostat algorithm (controls adaptation speed).
/// Higher values adapt faster but may be less stable.
/// 
/// - Example: `0.1`
/// - Default: `0.1`
pub const SAMPLER_MIROSTAT_ETA: &str = "mirostat_eta";

/// Target entropy value for Mirostat to maintain output quality.
/// Typical values depend on model size and task complexity.
/// - Example: `5.0`
/// - Default: `5.0`
pub const SAMPLER_MIROSTAT_TAU: &str = "mirostat_tau";

/// Disable performance metrics
/// - Example: `true`
/// - Default: `false`
pub const SAMPLER_NO_PREF: &str = "no_perf";

//******************* */
//CONTEXT PARAMETERS //
//******************* */
// === General Model Parameters ===

/// Context size (number of tokens in the model's sliding window).
/// This controls how many tokens the model can "remember".
/// 0 = from model
/// - Example: `512` for small models, `4096` or higher for large ones.
/// - Default: `512`
pub const CTX_N_CTX: &str = "n_ctx";

/// Number of tokens to process at once in a forward pass.
/// Logical maximum batch size that can be submitted to llama_decode
/// Affects performance and memory usage.
/// - Example: `1024`
/// - Default: `2048`
pub const CTX_N_BATCH: &str = "n_batch";

/// Size of a micro-batch (subset of a batch) for partial processing.
/// physical maximum batch size
/// - Example: `256`
/// - Default: `512`
pub const CTX_N_UBATCH: &str = "n_ubatch";

/// Maximum number of concurrent sequences processed.
/// Rarely changed; used in streaming or batched mode.
/// - Example: `2`
/// - Default: `1`
pub const CTX_N_SEQ_MAX: &str = "n_seq_max";

/// Number of threads for model computation.
/// Generally set to number of physical cores.
/// - Example: `8`
/// - Default: `GGML_DEFAULT_N_THREADS`
pub const CTX_N_THREADS: &str = "n_threads";

/// Number of threads specifically for batching operations.
/// Should generally match or be less than `n_threads`.
/// - Example: `4`
/// - Default: `GGML_DEFAULT_N_THREADS`
pub const CTX_N_THREADS_BATCH: &str = "n_threads_batch";

// === Positional Encoding (RoPE) and Attention Config ===

/// RoPE scaling strategy used for positional embeddings.
/// RoPE scaling type, from `enum llama_rope_scaling_type`
/// Choose from unspecified, linear, or dynamic.
/// if (value == "none" rope_scaling_type = LLAMA_ROPE_SCALING_TYPE_NONE
/// if (value == "linear" rope_scaling_type = LLAMA_ROPE_SCALING_TYPE_LINEAR
/// if (value == "yarn" rope_scaling_type = LLAMA_ROPE_SCALING_TYPE_YARN
/// - Example: `"linear"`
/// - Default: `"none"`
pub const CTX_ROPE_SCALING_TYPE: &str = "rope_scaling_type";

/// Base frequency used in RoPE calculations.
/// - Example: `10000.0`
/// - Default: `0.0`
pub const CTX_ROPE_FREQ_BASE: &str = "rope_freq_base";

/// Frequency scaling factor for RoPE.
/// - Example: `1.0`
/// - Default: `0.0`
pub const CTX_ROPE_FREQ_SCALE: &str = "rope_freq_scale";

/// Type of attention mechanism to use.
/// Choose from variants like multi-head, grouped, etc.
/// - Example: `"multihead"`
/// - Default: `"LLAMA_ATTENTION_TYPE_UNSPECIFIED"`
pub const CTX_ATTENTION_TYPE: &str = "attention_type";

/// Pooling strategy applied after encoding.
/// Used in sentence embeddings or classifier heads.
/// - Example: `"mean"`
/// - Default: `"LLAMA_POOLING_TYPE_UNSPECIFIED"`
pub const CTX_POOLING_TYPE: &str = "pooling_type";


// === YARN-Specific Parameters ===

/// External scaling factor for RoPE normalization in YARN.
/// - Example: `1.0`
/// - Default: `-1.0`
pub const CTX_YARN_EXT_FACTOR: &str = "yarn_ext_factor";

/// Factor applied to attention scores in YARN.
/// - Example: `0.5`
/// - Default: `1.0`
pub const CTX_YARN_ATTN_FACTOR: &str = "yarn_attn_factor";

/// Fast beta parameter for dynamic behavior in YARN.
/// - Example: `32.0`
/// - Default: `32.0`
pub const CTX_YARN_BETA_FAST: &str = "yarn_beta_fast";

/// Slow beta parameter for stable control in YARN.
/// - Example: `1.0`
/// - Default: `1.0`
pub const CTX_YARN_BETA_SLOW: &str = "yarn_beta_slow";

/// Original context length for YARN to reference when adapting.
/// - Example: `512`
/// - Default: `0`
pub const CTX_YARN_ORIG_CTX: &str = "yarn_orig_ctx";


// === Memory, Evaluation, and Performance ===

/// Memory defragmentation threshold (in MB or custom unit).
/// Used to control memory compaction.
/// - Example: `128.0`
/// - Default: `-1.0` (disabled)
pub const CTX_DEFRAG_THOLD: &str = "defrag_thold";

/// Callback function invoked during evaluation steps.
/// Pointer or identifier.
/// - Example: `"my_eval_fn"`
/// - Default: `nullptr`
pub const CTX_CB_EVAL: &str = "cb_eval";

/// Opaque data passed into the evaluation callback.
/// Often used for context or state.
/// - Example: `"my_data_ptr"`
/// - Default: `nullptr`
pub const CTX_CB_EVAL_USER_DATA: &str = "cb_eval_user_data";


/// Data type used for the key vectors in attention.
/// - Example: `"GGML_TYPE_F16"`
/// - Default: `"GGML_TYPE_F16"`
pub const CTX_TYPE_K: &str = "type_k";

/// Data type used for the value vectors.
/// - Example: `"GGML_TYPE_F16"`
/// - Default: `"GGML_TYPE_F16"`
pub const CTX_TYPE_V: &str = "type_v";


/// Whether to return logits for **all tokens**, not just the last.
/// Useful for fine-tuning or analyzing token-by-token output.
/// - Example: `true`
/// - Default: `false`
pub const CTX_LOGITS_ALL: &str = "logits_all";

/// Whether to return the final **embedding vector** for input.
///
/// Used in sentence embeddings, classifiers, etc.
/// - Example: `true`
/// - Default: `false`
pub const CTX_EMBEDDINGS: &str = "embeddings";


/// Whether to offload K/Q/V projections (attention ops) to another device.
/// - Example: `true`
/// - Default: `true`
pub const CTX_OFFLOAD_KQV: &str = "offload_kqv";

/// Whether to use FlashAttention (fast fused attention kernel).
///
/// Requires specific hardware (e.g., A100) and kernel support.
/// - Example: `true`
/// - Default: `false`
pub const CTX_FLASH_ATTN: &str = "flash_attn";

/// Disables all internal performance reporting.
/// Useful for benchmarking or clean output.
/// - Example: `true`
/// - Default: `true`
pub const CTX_NO_PERF: &str = "no_perf";


/// Callback function to abort inference early.
/// Called periodically to check whether to cancel generation.
/// - Example: `"should_abort"`
/// - Default: `nullptr`
pub const CTX_ABORT_CALLBACK: &str = "abort_callback";

/// User-defined data passed into the abort callback.
/// Typically state or control flags.
/// - Example: `"abort_state_ptr"`
/// - Default: `nullptr`
pub const CTX_ABORT_CALLBACK_DATA: &str = "abort_callback_data";

//******************* */
//FRAMEWORK Model config Parameters //
//******************* */
pub const FRAMEWORK_MODEL_DISABLE_GPU: &str = "disable_gpu";

pub const FRAMEWORK_MODEL_MAX_MSG_HISTORY: &str = "max_msg_history";

pub const FRAMEWORK_MODEL_ALWAYS_ADD_SYSTEM: &str = "always_add_system";

pub const FRAMEWORK_MODEL_ENABLE_TINKING: &str  = "enable_thinking";

//******************* */
//LLAMA Model config Parameters //
//******************* */
pub const LLAMA_CONTEXT_LENGTH: &str = "llama.context_length";