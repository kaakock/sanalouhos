use std::io::Read;

use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};

static BOARD_BASE_URL: &str = "https://sanalouhos.datadesk.hs.fi/api/game/";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BoardBodyItem {
    game_char_array: Vec<char>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BoardBody {
    success: bool,
    item: BoardBodyItem,
}

fn parse_board_res(res: &mut Response) -> Vec<char> {
    let mut body = String::new();
    let _ = res.read_to_string(&mut body);
    let deserialized: BoardBody = serde_json::from_str(&body).unwrap();
    return deserialized.item.game_char_array;
}

// date is formatted as D(D).M(M).YYYY
// ie. 15.5.2024
// 1.12.2024
pub fn fetch_board_for_date(date: &str) -> Vec<char> {
    let path: String = BOARD_BASE_URL.to_string() + date;
    let res = reqwest::blocking::get(path);
    let char_array = match res {
        Ok(mut r) => parse_board_res(&mut r),
        Err(_) => todo!(),
    };
    // map to uppercase should work also with non-ascii characters
    return char_array
        .into_iter()
        .map(|x| x.to_uppercase().next().unwrap())
        .collect();
}
