use serde::{Deserialize, Serialize};

// Generate Text

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateTextRequest {
    pub prompt: TextPrompt,
    #[serde(skip_serializing_if = "Option::is_none", rename = "safetyRatings")]
    pub safety_settings: Option<Vec<SafetySetting>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "stopSequences")]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "candidateCount")]
    pub candidate_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxOutputTokens")]
    pub max_output_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "topP")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "topK")]
    pub top_k: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextPrompt {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SafetySetting {
    pub category: HarmCategory,
    pub threshold: HarmBlockThreshold,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HarmCategory {
    #[serde(rename = "HARM_CATEGORY_DEROGATORY")]
    HarmCategoryDerogatory,
    #[serde(rename = "HARM_CATEGORY_TOXICITY")]
    HarmCategoryToxicity,
    #[serde(rename = "HARM_CATEGORY_VIOLENCE")]
    HarmCategoryViolence,
    #[serde(rename = "HARM_CATEGORY_SEXUAL")]
    HarmCategorySexual,
    #[serde(rename = "HARM_CATEGORY_MEDICAL")]
    HarmCategoryMedical,
    #[serde(rename = "HARM_CATEGORY_DANGEROUS")]
    HarmCategoryDangerous,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HarmBlockThreshold {
    #[serde(rename = "HARM_BLOCK_THRESHOLD_UNSPECIFIED")]
    HarmBlockThresholdUnspecified,
    #[serde(rename = "BLOCK_LOW_AND_ABOVE")]
    BlockLowAndAbove,
    #[serde(rename = "BLOCK_MEDIUM_AND_ABOVE")]
    BlockMediumAndAbove,
    #[serde(rename = "BLOCK_ONLY_HIGH")]
    BlockOnlyHigh,
    #[serde(rename = "BLOCK_NONE")]
    BlockNone,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenerateTextResponse {
    pub candidates: Vec<TextCompletion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ContentFilter>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_feedback: Option<Vec<SafetyFeedback>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextCompletion {
    pub output: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "safetyRatings")]
    pub safety_ratings: Option<Vec<SafetyRating>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "citation_metadata")]
    pub citation_metadata: Option<CitationMetadata>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SafetyRating {
    pub category: HarmCategory,
    pub probability: HarmProbability,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HarmProbability {
    #[serde(rename = "NEGLIGIBLE")]
    Negligible,
    #[serde(rename = "LOW")]
    Low,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "HIGH")]
    High,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SafetyFeedback {
    pub rating: SafetyRating,
    pub setting: SafetySetting,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BlockedReason {
    BlockedReasonUnspecified,
    Safety,
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentFilter {
    pub reason: BlockedReason,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CitationSource {
    #[serde(skip_serializing_if = "Option::is_none", rename = "startIndex")]
    pub start_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "endIndex")]
    pub end_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CitationMetadata {
    #[serde(skip_serializing_if = "Option::is_none", rename = "citationSources")]
    pub citation_sources: Option<Vec<CitationSource>>,
}

// Embeddings

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedTextRequest {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EmbedTextResponse {
    pub embedding: Embedding,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Embedding {
    pub value: Vec<f64>,
}
