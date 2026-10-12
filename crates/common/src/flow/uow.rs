/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

//! The unit of work a flow's writes join. For now it only commits callbacks in order;
//! the transaction behind it arrives with the integrity stage, keeping this API.

use std::sync::Mutex;
use ymir::errors::Outcome;

type CommitHook = Box<dyn FnOnce() + Send>;

/// Everything one use case writes; callbacks run only if it commits.
#[derive(Default)]
pub struct UnitOfWork {
    after_commit: Mutex<Vec<CommitHook>>,
}

impl UnitOfWork {
    /// Unit of work without a transaction: each write stands on its own.
    pub fn autocommit() -> Self {
        Self::default()
    }

    /// Runs `hook` after a successful commit (cache writes, waking the outbox).
    pub fn on_commit(&self, hook: impl FnOnce() + Send + 'static) {
        self.after_commit
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push(Box::new(hook));
    }

    pub async fn commit(self) -> Outcome<()> {
        let hooks = self
            .after_commit
            .into_inner()
            .unwrap_or_else(|p| p.into_inner());
        hooks.into_iter().for_each(|hook| hook());
        Ok(())
    }

    /// Discards the work and its commit hooks.
    pub async fn rollback(self) -> Outcome<()> {
        Ok(())
    }
}

/// Opens one unit of work per flow run.
#[async_trait::async_trait]
pub trait UnitOfWorkFactoryTrait: Send + Sync {
    async fn begin(&self) -> Outcome<UnitOfWork>;
}

/// Factory of units of work without a transaction.
pub struct AutocommitUnitOfWorkFactory;

#[async_trait::async_trait]
impl UnitOfWorkFactoryTrait for AutocommitUnitOfWorkFactory {
    async fn begin(&self) -> Outcome<UnitOfWork> {
        Ok(UnitOfWork::autocommit())
    }
}
