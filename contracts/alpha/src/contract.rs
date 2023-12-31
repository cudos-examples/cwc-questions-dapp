use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{QuestionResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{STATE, Question};

const CONTRACT_NAME: &str = "crates.io:cwc-questions-dapp";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION);

    Ok(Response::new()
    .add_attribute("method", "instantiate")
    .add_attribute("owner", info.sender))
}

// And declare a custom Error variant for the ones where you will want to make use of it
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::PostQuestion { handle, question } => try_post_question(deps, question),
    }
}

pub fn try_post_question(deps: DepsMut, question: String) -> Result<Response, ContractError> {
    
    let question = Question {
        question: question
    };

    STATE.save(deps.storage, &question);

    Ok(Response::new().add_attribute("action", "post_question"))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetQuestions{} => to_binary(&query_questions(deps)?),
    }
}

fn query_questions(deps: Deps) -> StdResult<QuestionResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(QuestionResponse { questions: state.question })
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::{mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info};
//     use cosmwasm_std::{coins, from_binary};
//
//     #[test]
//     fn proper_initialization() {
//         let mut deps = mock_dependencies();
//
//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(1000, "earth"));
//
//         // we can just call .unwrap() to assert this was a success
//         let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
//         assert_eq!(0, res.messages.len());
//
//         // it worked, let's query the state
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(17, value.count);
//     }
//
//     #[test]
//     fn increment() {
//         let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
//
//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
//
//         // beneficiary can release it
//         let info = mock_info("anyone", &coins(2, "token"));
//         let msg = ExecuteMsg::Increment {};
//         let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
//
//         // should increase counter by 1
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(18, value.count);
//     }
//
//     #[test]
//     fn reset() {
//         let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
//
//         let msg = InstantiateMsg { count: 17 };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
//
//         // beneficiary can release it
//         let unauth_info = mock_info("anyone", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
//         match res {
//             Err(ContractError::Unauthorized {}) => {}
//             _ => panic!("Must return unauthorized error"),
//         }
//
//         // only the original creator can reset the counter
//         let auth_info = mock_info("creator", &coins(2, "token"));
//         let msg = ExecuteMsg::Reset { count: 5 };
//         let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();
//
//         // should now be 5
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
//         let value: CountResponse = from_binary(&res).unwrap();
//         assert_eq!(5, value.count);
//     }
// }
