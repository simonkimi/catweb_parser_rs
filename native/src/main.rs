mod models;


fn main() {
    let json_str = r##"{"selector":"#gdo","type":"css","function":"raw","param":"","regex":"<div class=\"ths nosel\">(\\d+) rows.+sel\">(\\w)","replace":"$1|$2","script":{"script":"function hook(t){var row=t.split('|')[0];var column=t.split('|')[1];return parseInt(row)*{'L':5,'N':10}[column]}","type":"javascript"},"defaultValue":"123123"}"##;
    let selector: models::Selector = serde_json::from_str(json_str).unwrap();
    println!("{:#?}", selector);
}