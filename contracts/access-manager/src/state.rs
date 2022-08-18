use cosmwasm_std::{Addr, MessageInfo, StdError, StdResult, Storage};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use secret_toolkit::{
    storage::{TypedStore, TypedStoreMut},
    utils::types::{Contract, WasmCode},
};
use serde::{Deserialize, Serialize};

use crate::types::Payment;

pub const CONFIG_KEY: &[u8] = b"config";
pub const VIDEOS_ID_KEY: &[u8] = b"videos_id";
pub const VIDEOS_KEY: &[u8] = b"videos";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub owner: Addr,
    pub access_token_wasm: WasmCode,
}

impl Config {
    pub fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
        TypedStoreMut::attach(storage).store(CONFIG_KEY, self)
    }

    pub fn load(storage: &dyn Storage) -> StdResult<Self> {
        TypedStore::attach(storage).load(CONFIG_KEY)
    }

    pub fn assert_owner(&self, info: &MessageInfo) -> StdResult<()> {
        if self.owner != info.sender {
            return Err(StdError::generic_err(
                "you are not the owner of this contract",
            ));
        }

        Ok(())
    }
}

pub struct VideoID {}

impl VideoID {
    pub fn current(storage: &dyn Storage) -> StdResult<u128> {
        TypedStore::attach(storage).load(VIDEOS_ID_KEY)
    }

    pub fn load_and_increment(storage: &mut dyn Storage) -> StdResult<u128> {
        let mut id_store = TypedStoreMut::attach(storage);
        let new_id = match id_store.may_load(VIDEOS_ID_KEY)? {
            Some(id) => id + 1,
            None => 1,
        };
        id_store.store(VIDEOS_ID_KEY, &new_id)?;

        Ok(new_id)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Video {
    pub id: u128,
    pub access_token: Option<Contract>,
    pub info: VideoInfo,
}

impl Video {
    pub fn new(id: u128, info: VideoInfo) -> Self {
        Self {
            id,
            access_token: None,
            info,
        }
    }

    pub fn load_and_set_address(
        storage: &mut dyn Storage,
        id: u128,
        access_token: Contract,
    ) -> StdResult<()> {
        let mut video = Self::load(storage, id)?;
        video.access_token = Some(access_token);
        video.save(storage)
    }

    pub fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
        let mut videos_store = PrefixedStorage::new(storage, VIDEOS_KEY);
        TypedStoreMut::attach(&mut videos_store).store(&self.id.to_be_bytes(), self)
    }

    pub fn load(storage: &dyn Storage, id: u128) -> StdResult<Self> {
        let videos_store = ReadonlyPrefixedStorage::new(storage, VIDEOS_KEY);
        TypedStore::attach(&videos_store).load(&id.to_be_bytes())
    }
}

#[derive(Serialize, Deserialize)]
pub struct VideoInfo {
    pub name: String,
    pub royalty_info: snip721::royalties::RoyaltyInfo,
    pub price: Payment,
}