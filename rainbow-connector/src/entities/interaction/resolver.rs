use crate::entities::common::parameter_mutator::TemplateMutator;
use crate::entities::common::parameters::TemplateMutable;
use crate::entities::interaction::{InteractionConfig, PullLifecycle, PushLifecycle};

impl TemplateMutable for InteractionConfig {
    fn accept_mutator<V: TemplateMutator>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            InteractionConfig::Pull(lifecycle) => {
                visitor.enter_scope("pull");
                lifecycle.accept_mutator(visitor)?;
                visitor.exit_scope();
            }
            InteractionConfig::Push(lifecycle) => {
                visitor.enter_scope("push");
                lifecycle.accept_mutator(visitor)?;
                visitor.exit_scope();
            }
        }
        Ok(())
    }
}

impl TemplateMutable for PullLifecycle {
    fn accept_mutator<V: TemplateMutator>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        visitor.enter_scope("dataAccess");
        self.data_access.accept_mutator(visitor)?;
        visitor.exit_scope();
        Ok(())
    }
}

impl TemplateMutable for PushLifecycle {
    fn accept_mutator<V: TemplateMutator>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        visitor.enter_scope("subscribe");
        self.subscribe.accept_mutator(visitor)?;
        visitor.exit_scope();

        if let Some(unsubscribe) = &mut self.unsubscribe {
            visitor.enter_scope("unsubscribe");
            unsubscribe.accept_mutator(visitor)?;
            visitor.exit_scope();
        }

        Ok(())
    }
}
