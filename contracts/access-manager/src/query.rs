use cosmwasm_std::{to_binary, Binary, Deps, StdResult};

use crate::{msg::QueryAnswer, state::Video};

pub fn video_info(deps: Deps, id: u64) -> StdResult<Binary> {
    let video = Video::load(deps.storage, id)?;

    to_binary(&QueryAnswer::VideoInfo {
        id,
        access_token: deps.api.addr_validate(&video.access_token.address)?,
        name: video.info.name,
        royalty_info: video.info.royalty_info,
        price: video.info.price,
    })
}
