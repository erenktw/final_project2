use soroban_sdk::{contractimpl, Address, Env, Symbol, Map};

pub struct TokenContract;

#[derive(Clone)]
pub struct DataKey;

impl DataKey {
    const BALANCES: Symbol = Symbol::short("balances");
    const FROZEN: Symbol = Symbol::short("frozen");
}

#[contractimpl]
impl TokenContract {
    pub fn freeze_account(env: Env, account: Address) {
        let mut frozen: Map<Address, bool> = env.storage().persistent().get(&DataKey::FROZEN).unwrap_or_default();
        frozen.set(account.clone(), true);
        env.storage().persistent().set(&DataKey::FROZEN, &frozen);
        env.events().publish((Symbol::short("freeze"), account), true);
    }

    pub fn unfreeze_account(env: Env, account: Address) {
        let mut frozen: Map<Address, bool> = env.storage().persistent().get(&DataKey::FROZEN).unwrap_or_default();
        frozen.set(account.clone(), false);
        env.storage().persistent().set(&DataKey::FROZEN, &frozen);
        env.events().publish((Symbol::short("unfreeze"), account), true);
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        let frozen: Map<Address, bool> = env.storage().persistent().get(&DataKey::FROZEN).unwrap_or_default();
        if let Some(true) = frozen.get(from.clone()) {
            panic!("Transfer denied: account is frozen.");
        }

        let mut balances: Map<Address, i128> = env.storage().persistent().get(&DataKey::BALANCES).unwrap_or_default();
        let from_balance = balances.get(from.clone()).unwrap_or(0);

        if from_balance < amount {
            panic!("Transfer failed: insufficient balance.");
        }

        balances.set(from.clone(), from_balance - amount);
        let to_balance = balances.get(to.clone()).unwrap_or(0);
        balances.set(to.clone(), to_balance + amount);
        env.storage().persistent().set(&DataKey::BALANCES, &balances);
    }
}
