use crate::WsMessage;

use super::*;

const REQUEST_CANDLES: &'static str = "candles-req";
const RESPONSE_CANDLES: &'static str = "candles-res";
const BID_ASK: &'static str = "bid-ask";
const SET_LEVEL: &'static str = "set-level";
const LEVELS: &'static str = "levels";

#[derive(Debug)]
pub enum WsContract {
    GetCandlesRequest(GetCandlesWsRequestContract),
    GetCandlesResponse(GetCandlesWsResponseContract),
    BidAsk(Vec<BidAskWsModel>),
    SetLevel(LevelModel),
    Levels(Vec<LevelModel>),
}

impl WsContract {
    pub fn from_str(src: &str) -> Option<Self> {
        let index = src.find(':')?;

        let payload_id = &src[..index];

        let payload = &src[index + 1..];

        match payload_id {
            REQUEST_CANDLES => {
                let data = serde_json::from_str(payload).unwrap();
                return Some(Self::GetCandlesRequest(data));
            }
            RESPONSE_CANDLES => {
                let data = serde_json::from_str(payload).unwrap();
                return Some(Self::GetCandlesResponse(data));
            }
            BID_ASK => {
                let data = serde_json::from_str(payload).unwrap();
                return Some(Self::BidAsk(data));
            }
            SET_LEVEL => {
                let data = serde_json::from_str(payload).unwrap();
                return Some(Self::SetLevel(data));
            }
            LEVELS => {
                let data = serde_json::from_str(payload).unwrap();
                return Some(Self::Levels(data));
            }
            _ => return None,
        }
    }

    pub fn to_ws_message(&self) -> WsMessage {
        let result = match self {
            WsContract::GetCandlesRequest(data) => {
                let mut result = serde_json::to_string(data).unwrap();
                result.insert(0, ':');
                result.insert_str(0, REQUEST_CANDLES);
                result
            }
            WsContract::GetCandlesResponse(data) => {
                let mut result = serde_json::to_string(data).unwrap();
                result.insert(0, ':');
                result.insert_str(0, RESPONSE_CANDLES);
                result
            }

            WsContract::BidAsk(data) => {
                let mut result = serde_json::to_string(data).unwrap();
                result.insert(0, ':');
                result.insert_str(0, BID_ASK);
                result
            }
            WsContract::SetLevel(data) => {
                let mut result = serde_json::to_string(data).unwrap();
                result.insert(0, ':');
                result.insert_str(0, SET_LEVEL);
                result
            }
            WsContract::Levels(data) => {
                let mut result = serde_json::to_string(data).unwrap();
                result.insert(0, ':');
                result.insert_str(0, LEVELS);
                result
            }
        };

        WsMessage::Text(result.into())
    }
}
