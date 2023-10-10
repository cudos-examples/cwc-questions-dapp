use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::error::ContractError;
use crate::msg::{QuestionResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        // count: msg.count,
        questions: msg.questions,
        owner: info.sender,
    };
    STATE.save(deps.storage, &state)?;

    Ok(Response::default())
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
        ExecuteMsg::PostQuestion {} => try_post_question(deps, question),
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),
    }
}

pub fn try_post_question(deps: DepsMut, question: String) -> Result<Response, ContractError> {
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
    Ok(QuestionResponse { questions: state.questions })
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
