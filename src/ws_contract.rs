use crate::WsMessage;

use super::*;

const REQUEST_CANDLES: &'static str = "candles-req";
const RESPONSE_CANDLES: &'static str = "candles-res";
const BID_ASK: &'static str = "bid-ask";
const SET_PRICE_LEVEL: &'static str = "set-price_level";
const DEL_PRICE_LEVEL: &'static str = "del-price_level";
const PRICE_LEVELS: &'static str = "price-levels";
const ERROR: &'static str = "error";
const SET_ACTIVE_INSTRUMENT: &'static str = "set-active-instrument";
const INSTRUMENTS: &'static str = "instruments";
const ATR: &'static str = "atr";

#[derive(Debug)]
pub enum WsContract {
    GetCandlesRequest(GetCandlesWsRequestContract),
    GetCandlesResponse(GetCandlesWsResponseContract),
    BidAsk(Vec<BidAskWsModel>),
    SetPriceLevel(PriceLevelWsModel),
    DeletePriceLevel(PriceLevelWsModel),
    PriceLevels(Vec<PriceLevelWsModel>),
    SetActiveInstrument(SetActiveInstrumentWsModel),
    Instruments(Vec<InstrumentWsModel>),
    Atr(Vec<AtrWsModel>),
    Error(ErrorWsModel),
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
            SET_PRICE_LEVEL => {
                let data = serde_json::from_str(payload).unwrap();
                return Some(Self::SetPriceLevel(data));
            }
            DEL_PRICE_LEVEL => {
                let data = serde_json::from_str(payload).unwrap();
                return Some(Self::DeletePriceLevel(data));
            }
            PRICE_LEVELS => {
                let data = serde_json::from_str(payload).unwrap();
                return Some(Self::PriceLevels(data));
            }
            ERROR => {
                let data = serde_json::from_str(payload).unwrap();
                return Some(Self::Error(data));
            }
            SET_ACTIVE_INSTRUMENT => {
                let data = serde_json::from_str(payload).unwrap();
                return Some(Self::SetActiveInstrument(data));
            }
            INSTRUMENTS => {
                let data = serde_json::from_str(payload).unwrap();
                return Some(Self::Instruments(data));
            }
            ATR => {
                let data = serde_json::from_str(payload).unwrap();
                return Some(Self::Atr(data));
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
            WsContract::SetPriceLevel(data) => {
                let mut result = serde_json::to_string(data).unwrap();
                result.insert(0, ':');
                result.insert_str(0, SET_PRICE_LEVEL);
                result
            }
            WsContract::PriceLevels(data) => {
                let mut result = serde_json::to_string(data).unwrap();
                result.insert(0, ':');
                result.insert_str(0, PRICE_LEVELS);
                result
            }
            WsContract::DeletePriceLevel(data) => {
                let mut result = serde_json::to_string(data).unwrap();
                result.insert(0, ':');
                result.insert_str(0, DEL_PRICE_LEVEL);
                result
            }
            WsContract::Error(data) => {
                let mut result = serde_json::to_string(data).unwrap();
                result.insert(0, ':');
                result.insert_str(0, ERROR);
                result
            }
            WsContract::SetActiveInstrument(data) => {
                let mut result = serde_json::to_string(data).unwrap();
                result.insert(0, ':');
                result.insert_str(0, SET_ACTIVE_INSTRUMENT);
                result
            }
            WsContract::Instruments(data) => {
                let mut result = serde_json::to_string(data).unwrap();
                result.insert(0, ':');
                result.insert_str(0, INSTRUMENTS);
                result
            }
            WsContract::Atr(data) => {
                let mut result = serde_json::to_string(data).unwrap();
                result.insert(0, ':');
                result.insert_str(0, ATR);
                result
            }
        };

        WsMessage::Text(result.into())
    }
}
