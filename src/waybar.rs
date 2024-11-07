use serde::Serialize;

#[derive(Default, Serialize)]
pub struct WaybarResult<'a> {
    pub text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tooltip: Option<&'a str>,
    pub class: &'a [&'a str],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<f32>,
}
