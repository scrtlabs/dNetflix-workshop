use cosmwasm_std::{Addr, MessageInfo, StdError, StdResult, Storage};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use secret_toolkit::{
    storage::{TypedStore, TypedStoreMut},
    utils::types::{Contract, WasmCode},
};
use serde::{Deserialize, Serialize};

use crate::types::Payment;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub owner: Addr,
    pub access_token_wasm: WasmCode,
}

impl Config {
    pub const STORAGE_KEY: &'static [u8] = b"config";

    pub fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
        TypedStoreMut::attach(storage).store(Self::STORAGE_KEY, self)
    }

    pub fn load(storage: &dyn Storage) -> StdResult<Self> {
        TypedStore::attach(storage).load(Self::STORAGE_KEY)
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
    const STORAGE_KEY: &'static [u8] = b"videos_id";

    pub fn current(storage: &dyn Storage) -> StdResult<u64> {
        TypedStore::attach(storage).load(Self::STORAGE_KEY)
    }

    pub fn load_and_increment(storage: &mut dyn Storage) -> StdResult<u64> {
        let mut id_store = TypedStoreMut::attach(storage);
        let new_id = match id_store.may_load(Self::STORAGE_KEY)? {
            Some(id) => id + 1,
            None => 1,
        };
        id_store.store(Self::STORAGE_KEY, &new_id)?;

        Ok(new_id)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Video {
    pub id: u64,
    pub access_token: Option<Contract>,
    pub info: VideoInfo,
}

impl Video {
    pub const STORAGE_KEY: &'static [u8] = b"videos";

    pub fn new(id: u64, info: VideoInfo) -> Self {
        Self {
            id,
            access_token: None,
            info,
        }
    }

    pub fn load_and_set_token(
        storage: &mut dyn Storage,
        id: u64,
        access_token: Contract,
    ) -> StdResult<()> {
        let mut video = Self::load(storage, id)?;
        video.access_token = Some(access_token);
        video.save(storage)
    }

    pub fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
        let mut videos_store = PrefixedStorage::new(storage, Self::STORAGE_KEY);
        TypedStoreMut::attach(&mut videos_store).store(&self.id.to_be_bytes(), self)
    }

    pub fn load(storage: &dyn Storage, id: u64) -> StdResult<Self> {
        let videos_store = ReadonlyPrefixedStorage::new(storage, Self::STORAGE_KEY);
        TypedStore::attach(&videos_store).load(&id.to_be_bytes())
    }
}

#[derive(Serialize, Deserialize)]
pub struct VideoInfo {
    pub name: String,
    pub royalty_info: snip721::royalties::RoyaltyInfo,
    pub price: Payment,
}

impl Payment {
    pub const STORAGE_PREFIX: &'static [u8] = b"snip20";

    pub fn register_snip20(storage: &mut dyn Storage, address: Addr) {
        let mut snip20_store = PrefixedStorage::new(storage, Self::STORAGE_PREFIX);
        match snip20_store.get(address.as_bytes()) {
            Some(_) => {}
            None => snip20_store.set(address.as_bytes(), &[1]),
        }
    }

    pub fn is_snip20_registered(storage: &dyn Storage, address: Addr) -> bool {
        let snip20_store = ReadonlyPrefixedStorage::new(storage, Self::STORAGE_PREFIX);
        match snip20_store.get(address.as_bytes()) {
            Some(_) => true,
            None => false,
        }
    }
}
