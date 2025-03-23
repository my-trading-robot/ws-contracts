use crate::WsMessage;

use super::*;

const REQUEST_CANDLES: &'static str = "candles-req";
const RESPONSE_CANDLES: &'static str = "candles-res";
const BID_ASK: &'static str = "bid-ask";

pub enum WsContract {
    GetCandlesRequest(GetCandlesWsRequestContract),
    GetCandlesResponse(GetCandlesWsResponseContract),
    BidAsk(Vec<BidAskWsModel>),
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
                result.insert_str(0, REQUEST_CANDLES);
                result
            }

            WsContract::BidAsk(data) => {
                let mut result = serde_json::to_string(data).unwrap();
                result.insert(0, ':');
                result.insert_str(0, BID_ASK);
                result
            }
        };

        WsMessage::Text(result.into())
    }
}
