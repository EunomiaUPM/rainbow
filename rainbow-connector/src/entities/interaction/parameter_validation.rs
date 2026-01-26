use crate::entities::common::parameter_visitor::ParameterVisitor;
use crate::entities::common::parameters::TemplateVisitable;
use crate::entities::interaction::{InteractionConfig, PullLifecycle, PushLifecycle};

impl TemplateVisitable for InteractionConfig {
    fn accept<V: ParameterVisitor>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            InteractionConfig::Pull(lifecycle) => {
                visitor.enter_scope("pull");
                lifecycle.accept(visitor)?;
                visitor.exit_scope();
            }
            InteractionConfig::Push(lifecycle) => {
                visitor.enter_scope("push");
                lifecycle.accept(visitor)?;
                visitor.exit_scope();
            }
        }
        Ok(())
    }
}

impl TemplateVisitable for PullLifecycle {
    fn accept<V: ParameterVisitor>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        visitor.enter_scope("dataAccess");
        self.data_access.accept(visitor)?;
        visitor.exit_scope();
        Ok(())
    }
}

impl TemplateVisitable for PushLifecycle {
    fn accept<V: ParameterVisitor>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        visitor.enter_scope("subscribe");
        self.subscribe.accept(visitor)?;
        visitor.exit_scope();

        if let Some(unsubscribe) = &self.unsubscribe {
            visitor.enter_scope("unsubscribe");
            unsubscribe.clone().accept(visitor)?;
            visitor.exit_scope();
        }

        Ok(())
    }
}
