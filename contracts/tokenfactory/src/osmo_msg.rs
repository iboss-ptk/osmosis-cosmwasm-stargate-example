use cosmwasm_std::{CosmosMsg, Empty};
use std::convert::TryFrom;

/// MsgCreateDenom is the sdk.Msg type for allowing an account to create
/// a new denom. It requires a sender address and a subdenomination.
/// The (sender_address, sub_denomination) pair must be unique and cannot be
/// re-used. The resulting denom created is `factory/{creator
/// address}/{subdenom}`. The resultant denom's admin is originally set to be the
/// creator, but this can be changed later. The token denom does not indicate the
/// current admin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDenom {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// subdenom can be up to 44 "alphanumeric" characters long.
    #[prost(string, tag = "2")]
    pub subdenom: ::prost::alloc::string::String,
}

impl TryFrom<MsgCreateDenom> for CosmosMsg {
    type Error = cosmwasm_std::StdError;

    fn try_from(msg: MsgCreateDenom) -> Result<Self, Self::Error> {
        let mut bytes = Vec::new();
        prost::Message::encode(&msg, &mut bytes).map_err(|e| {
            cosmwasm_std::StdError::SerializeErr {
                source_type: "MsgCreateDenom".to_string(),
                msg: format!("{:?}", e),
            }
        })?;
        Ok(CosmosMsg::<Empty>::Stargate {
            type_url: "/osmosis.tokenfactory.v1beta1.MsgCreateDenom".to_string(),
            value: cosmwasm_std::Binary(bytes),
        })
    }
}
