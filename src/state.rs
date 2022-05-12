use fadroma::{
    load, save, Api, CanonicalAddr, Canonize, ContractLink, Extern, HumanAddr, Humanize, Querier,
    StdResult, Storage,
};

const KEY: &'static [u8] = b"SELF_REF";

pub fn load_self_ref<S, A, Q>(deps: &Extern<S, A, Q>) -> StdResult<ContractLink<HumanAddr>>
where
    S: Storage,
    A: Api,
    Q: Querier,
{
    let self_ref: ContractLink<CanonicalAddr> = load(&deps.storage, KEY)?.unwrap();

    Ok(self_ref.humanize(&deps.api)?)
}

pub fn save_self_ref<S, A, Q>(
    deps: &mut Extern<S, A, Q>,
    self_ref: ContractLink<HumanAddr>,
) -> StdResult<()>
where
    S: Storage,
    A: Api,
    Q: Querier,
{
    let canonical = self_ref.canonize(&deps.api)?;
    save(&mut deps.storage, KEY, &canonical)
}
