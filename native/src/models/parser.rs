use crate::models::{ExtraSelector, ImageSelector, Selector};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AutoCompleteParser {
    pub name: String,
    pub uuid: String,
    pub extra: Vec<ExtraSelector>,
    pub item_selector: Selector,
    pub item_complete: Selector,
    pub item_title: Selector,
    pub item_subtitle: Selector,
    pub success_selector: Selector,
    pub failed_selector: Selector,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ImageReaderParser {
    pub name: String,
    pub uuid: String,
    pub extra: Vec<ExtraSelector>,

    pub id: Selector,
    pub image: ImageSelector,
    pub larger_image: Selector,
    pub raw_image: Selector,
    pub rating: Selector,
    pub score: Selector,
    pub source: Selector,
    pub upload_time: Selector,
    pub badge_selector: Selector,
    pub badge_text: Selector,
    pub success_selector: Selector,
    pub failed_selector: Selector,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GalleryParser {
    pub name: String,
    pub uuid: String,
    pub extra: Vec<ExtraSelector>,
    pub title: Selector,
    pub subtitle: Selector,
    pub upload_time: Selector,
    pub star: Selector,
    pub img_count: Selector,
    pub page_count: Selector,
    pub language: Selector,
    pub cover_img: ImageSelector,
    pub description: Selector,
    pub thumbnail_selector: Selector,
    pub thumbnail: ImageSelector,
    pub target: Selector,
    pub comment_selector: Selector,
    pub comments: CommentSelector,
    pub tag: Selector,
    pub tag_color: Selector,
    pub badge_selector: Selector,
    pub badge_text: Selector,
    pub badge_category: Selector,
    pub chapter_selector: Selector,
    pub chapter_title: Selector,
    pub chapter_subtitle: Selector,
    pub chapter_cover: ImageSelector,
    pub next_page: Selector,
    pub count_pre_page: Selector,
    pub success_selector: Selector,
    pub failed_selector: Selector,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListViewParser {
    pub name: String,
    pub uuid: String,
    pub extra: Vec<ExtraSelector>,
    pub item_selector: Selector,
    pub title: Selector,
    pub subtitle: Selector,
    pub upload_time: Selector,
    pub star: Selector,
    pub img_count: Selector,
    pub language: Selector,
    pub preview_img: ImageSelector,
    pub target: Selector,
    pub tag: Selector,
    pub tag_color: Selector,
    pub badge_selector: Selector,
    pub badge_text: Selector,
    pub badge_color: Selector,
    pub paper: Selector,
    pub id_code: Selector,
    pub next_page: Selector,
    pub success_selector: Selector,
    pub failed_selector: Selector,
}