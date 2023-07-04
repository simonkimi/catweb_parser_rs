use std::collections::HashMap;
use serde::{Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImageRspModel {
    pub img_url: Option<String>,
    pub img_width: Option<i32>,
    pub img_height: Option<i32>,
    pub img_x: Option<i32>,
    pub img_y: Option<i32>,
}

#[derive(Serialize, Debug)]
pub struct TagRspModel {
    pub text: Option<String>,
    pub color: Option<ColorRspModel>,
    pub category: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct ColorRspModel {
    pub a: Option<i32>,
    pub r: Option<i32>,
    pub g: Option<i32>,
    pub b: Option<i32>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListParserResultItem {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub upload_time: Option<String>,
    pub star: Option<f64>,
    pub img_count: Option<i32>,
    pub preview_img: Option<ImageRspModel>,
    pub language: Option<String>,
    pub tag: Option<TagRspModel>,
    pub badges: Option<Vec<TagRspModel>>,
    pub paper: Option<String>,
    pub target: Option<String>,
    pub next_page: Option<String>,
    pub env: HashMap<String, String>,
}


#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListParserResult {
    pub items: Vec<ListParserResultItem>,
    pub next_page: Option<String>,
    pub enable_success: Option<bool>,
    pub enable_fail: Option<bool>,
    pub is_success: Option<bool>,
    pub failed_message: Option<String>,
    pub local_env: HashMap<String, String>,
    pub global_env: HashMap<String, String>,
}


#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GalleryParserResult {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub language: Option<String>,
    pub image_count: Option<i32>,
    pub upload_time: Option<String>,
    pub count_pre_page: Option<i32>,
    pub description: Option<String>,
    pub star: Option<f64>,
    pub items: Vec<GalleryParserResultItem>,
    pub cover_img: Option<ImageRspModel>,
    pub tag: Option<TagRspModel>,
    pub badges: Option<Vec<TagRspModel>>,
    pub comments: Option<Vec<GalleryParserResultComment>>,
    pub next_page: Option<String>,
    pub enable_success: Option<bool>,
    pub enable_fail: Option<bool>,
    pub is_success: Option<bool>,
    pub failed_message: Option<String>,
    pub local_env: HashMap<String, String>,
    pub global_env: HashMap<String, String>,
}


#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GalleryParserResultComment {
    pub username: Option<String>,
    pub content: String,
    pub time: Option<String>,
    pub score: Option<String>,
    pub avatar: Option<ImageRspModel>,
}


#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GalleryParserResultItem {
    pub preview_img: Option<ImageRspModel>,
    pub target: Option<String>,
}


#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImageReaderParserResult {
    pub image: Option<ImageRspModel>,
    pub larger_image_url: Option<String>,
    pub raw_image_url: Option<String>,
    pub upload_time: Option<String>,
    pub source: Option<String>,
    pub rating: Option<String>,
    pub score: Option<String>,
    pub badges: Option<Vec<TagRspModel>>,
    pub local_env: HashMap<String, String>,
    pub global_env: HashMap<String, String>,
    pub enable_success: Option<bool>,
    pub enable_fail: Option<bool>,
    pub is_success: Option<bool>,
    pub failed_message: Option<String>,
}


#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AutoCompleteParserResultItem {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub complete: Option<String>,
}


#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AutoCompleteParserResult {
    pub items: Vec<AutoCompleteParserResultItem>,
    pub local_env: HashMap<String, String>,
    pub global_env: HashMap<String, String>,
    pub enable_success: Option<bool>,
    pub enable_fail: Option<bool>,
    pub is_success: Option<bool>,
    pub failed_message: Option<String>,
}