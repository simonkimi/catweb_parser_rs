use serde::{Deserialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Selector {
    pub selector: String,
    #[serde(alias = "type")]
    pub selector_type: SelectorType,
    pub param: String,
    pub regex: String,
    pub replace: String,
    pub script: ScriptField,
    pub default_value: String,
}

#[derive(Deserialize, Debug)]
pub enum SelectorType {
    #[serde(rename = "css")]
    Css,
    #[serde(rename = "xpath")]
    Xpath,
    #[serde(rename = "json path")]
    JsonPath,
}

#[derive(Deserialize, Debug)]
pub struct ScriptField {
    pub script: String,
    #[serde(alias = "type")]
    pub script_type: ScriptFieldType,
}

#[derive(Deserialize, Debug)]
pub enum ScriptFieldType {
    #[serde(rename = "output")]
    Output,
    #[serde(rename = "javascript")]
    Js,
    #[serde(rename = "lua")]
    Lua,
    #[serde(rename = "computed")]
    Computed,
    #[serde(rename = "replace")]
    Replace,
}

#[derive(Deserialize, Debug)]
pub struct ExtraSelector {
    pub selector: Selector,
    pub id: String,
    pub global: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImageSelector {
    pub img_url: Selector,
    pub img_width: Selector,
    pub img_height: Selector,
    pub img_x: Selector,
    pub img_y: Selector,
}