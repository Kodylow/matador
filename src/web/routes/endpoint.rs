// TODO: rewrite to use endpoint
pub struct Endpoint {
    pub path: &'static str,
    pub pricing: u32, // satoshis
    pub method: &'static str,
    pub handler: &'static str,
}
