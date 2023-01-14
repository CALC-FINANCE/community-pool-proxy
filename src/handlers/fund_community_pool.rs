use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
use cosmos_sdk_proto::{cosmos::distribution::v1beta1::MsgFundCommunityPool, traits::Message};
use cosmwasm_std::{Binary, CosmosMsg, MessageInfo, Response, SubMsg, Env};

use crate::ContractError;

pub fn fund_community_pool(env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let amount: Vec<Coin> = info
        .funds
        .iter()
        .map(|coin| Coin {
            denom: coin.denom.clone(),
            amount: coin.amount.to_string(),
        })
        .collect();

    let mut buffer = vec![];

    MsgFundCommunityPool {
        amount,
        depositor: env.contract.address.into(),
    }
    .encode(&mut buffer)
    .unwrap();

    let fund_community_pool_msg = CosmosMsg::Stargate {
        type_url: "/cosmos.distribution.v1beta1.MsgFundCommunityPool".to_string(),
        value: Binary::from(buffer),
    };

    Ok(Response::new()
        .add_attribute("method", "fund_community_pool")
        .add_submessage(SubMsg::new(fund_community_pool_msg)))
}
