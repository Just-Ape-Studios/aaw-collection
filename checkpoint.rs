use ink::primitives::AccountId;
use ink::storage::Mapping;

#[ink::storage_item]
pub struct CheckpointData {
    account_to_checkpoint_map: Mapping<(AccountId, u128), Checkpoint>,
    num_of_checkpoints_per_account: Mapping<AccountId, u128>,
}

#[derive(Debug, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
)]
pub struct Checkpoint {
    pub from_block: u32,
    pub votes: u32,
}

impl CheckpointData {
    pub fn new() -> Self {
        CheckpointData {
            account_to_checkpoint_map: Mapping::new(),
            num_of_checkpoints_per_account: Mapping::new(),
        }
    }

    pub fn get_last_checkpoint(&self, account_id: AccountId) -> Option<Checkpoint> {
        let num_checkpoints = self
            .num_of_checkpoints_per_account
            .get(account_id)
            .unwrap_or(0);
        if num_checkpoints == 0 {
            return None;
        }

        let last_checkpoint_idx = num_checkpoints - 1;
        self.account_to_checkpoint_map
            .get((account_id, last_checkpoint_idx))
    }

    pub fn get_checkpoint_at_block(
        &self,
        account_id: AccountId,
        wanted_block: u32,
        current_block: u32,
    ) -> Option<Checkpoint> {
        if wanted_block > current_block {
            return None;
        }

        // ensure at least one checkpoint is present, otherwise no need to continue
        let num_checkpoints = self
            .num_of_checkpoints_per_account
            .get(account_id)
            .unwrap_or(0);
        if num_checkpoints == 0 {
            return None;
        }

        // ensure the first checkpoint is older than the requested block, otherwise no need to continue
        if self
            .account_to_checkpoint_map
            .get((account_id, 0))
            .unwrap()
            .from_block
            > wanted_block
        {
            return None;
        }

        let mut lower = 0;
        let mut upper = num_checkpoints - 1;

        // search for the latest checkpoint that is below the requested block
        while upper > lower {
            let center = upper - (upper - lower) / 2;
            // TODO handle error
            let cp = self
                .account_to_checkpoint_map
                .get((account_id, center))
                .unwrap();

            if cp.from_block == wanted_block {
                return Some(cp);
            } else if cp.from_block < wanted_block {
                lower = center;
            } else {
                upper = center - 1;
            }
        }

        // TODO handle error
        return Some(
            self.account_to_checkpoint_map
                .get((account_id, lower))
                .unwrap(),
        );
    }

    pub fn add_new_checkpoint_to_account(
        &mut self,
        account: AccountId,
        increment: bool,
        current_block: u32,
    ) {
        let num_of_checkpoints = self
            .num_of_checkpoints_per_account
            .get(account)
            .unwrap_or(0);

        if num_of_checkpoints == 0 {
            self.account_to_checkpoint_map.insert(
                (account, 0),
                &Checkpoint {
                    from_block: current_block,
                    votes: 1,
                },
            );

            self.num_of_checkpoints_per_account.insert(account, &1);
        } else {
            let last_checkpoint_idx = num_of_checkpoints - 1;

            // TODO handle error
            let last_checkpoint = self
                .account_to_checkpoint_map
                .get((account, last_checkpoint_idx))
                .unwrap();

            let next_cp_votes = if increment {
                last_checkpoint.votes + 1
            } else {
                last_checkpoint.votes - 1
            };

            self.account_to_checkpoint_map.insert(
                (account, num_of_checkpoints),
                &Checkpoint {
                    from_block: current_block,
                    votes: next_cp_votes,
                },
            );

            self.num_of_checkpoints_per_account
                .insert(account, &(num_of_checkpoints + 1));
        }
    }
}
