use cosmwasm_std::{
    entry_point, to_binary, BankMsg, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, attr, Uint128,
};
use serde::{Deserialize, Serialize};
use serde_json_wasm as serde_json; // Change this line to use serde_json_wasm

// Constants for storage keys and the rest of your code remains unchanged...


// Constants for storage keys
const ADDRESSES_KEY: &str = "addresses";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {
    pub addresses: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Disperse {},
    UpdateAddress { new_address: String },
}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let addresses = msg.addresses.unwrap_or_else(|| vec![
        "sei1qc39rrkf23c87zuds6tqq4wew3j8w6mwxvgqcq".to_string(),
        "sei1y72dwh0tm55jvc2gdt077clf2dy2wmnwalr94n".to_string(),
        "sei1tm6rksgqme9a3qghy32wl6kes288vc2llyka78".to_string(),
        "sei1fq7vzjkhcfqcluqdlaj3m0mqpgpdecnuqtnwn9".to_string(),
        "sei1mmua7vjdp7s777y5j5k0d0kxhsvf86wu5z0x0j".to_string(),
    ]);
    store_addresses(deps, &addresses)?;
    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Disperse {} => try_disperse(deps, env, info),
        ExecuteMsg::UpdateAddress { new_address } => {
            try_update_address(deps, info, new_address)
        },
    }
}

fn try_disperse(deps: DepsMut, env: Env, _info: MessageInfo) -> StdResult<Response> {
    let addresses = load_addresses(deps.as_ref())?;
    // Query the contract's balance for the specific denomination
    let contract_balance = deps.querier.query_balance(env.contract.address, "usei")?.amount;

    let percentages = vec![25u128, 20, 20, 20, 15];

    let msgs: Vec<CosmosMsg> = addresses
        .iter()
        .zip(percentages.iter())
        .map(|(address, &percentage)| {
            // Calculate the amount based on the contract's current balance
            let amount = contract_balance.multiply_ratio(percentage, 100u128);
            CosmosMsg::Bank(BankMsg::Send {
                to_address: address.to_string(),
                amount: vec![Coin {
                    denom: "usei".to_string(), // Ensure this matches the denomination queried
                    amount,
                }],
            })
        })
        .collect();

    Ok(Response::new()
        .add_messages(msgs)
        .add_attributes(vec![attr("action", "disperse"), attr("method", "try_disperse")]))
}


fn try_update_address(
    deps: DepsMut,
    info: MessageInfo,
    new_address: String,
) -> StdResult<Response> {
    let addresses = load_addresses(deps.as_ref())?; // Borrow deps immutably
    if let Some(index) = addresses.iter().position(|address| *address == info.sender.as_str()) {
        let mut addresses = addresses.clone(); // Clone for modification
        addresses[index] = new_address;
        store_addresses(deps, &addresses)?; // deps are moved here
        Ok(Response::new().add_attribute("method", "update_address"))
    } else {
        Err(cosmwasm_std::StdError::generic_err("Unauthorized"))
    }
}

// Adjust the signature of load_addresses to accept an immutable borrow
fn load_addresses(deps: Deps) -> StdResult<Vec<String>> {
    deps.storage.get(ADDRESSES_KEY.as_bytes()).map_or(Ok(vec![]), |bytes| {
        // Now using serde_json_wasm for deserialization
        serde_json::from_slice(&bytes)
            .map_err(|err| cosmwasm_std::StdError::generic_err(format!("Failed to deserialize addresses: {}", err)))
    })
}

fn store_addresses(deps: DepsMut, addresses: &Vec<String>) -> StdResult<()> {
    // Now using serde_json_wasm for serialization
    let serialized = serde_json::to_vec(addresses).map_err(|e| cosmwasm_std::StdError::generic_err(e.to_string()))?;
    deps.storage.set(ADDRESSES_KEY.as_bytes(), &serialized);
    Ok(())
}


