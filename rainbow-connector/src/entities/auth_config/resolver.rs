use crate::entities::auth_config::AuthenticationConfig;
use crate::entities::common::parameter_mutator::TemplateMutator;
use crate::entities::common::parameters::TemplateMutable;

impl TemplateMutable for AuthenticationConfig {
    fn accept_mutator<V: TemplateMutator>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            AuthenticationConfig::NoAuth => Ok(()),
            AuthenticationConfig::BasicAuth(config) => {
                visitor.enter_scope("basicAuth");
                config.username.accept_mutator(visitor)?;
                config.password.accept_mutator(visitor)?;
                visitor.exit_scope();
                Ok(())
            }
            AuthenticationConfig::BearerToken { token } => {
                visitor.enter_scope("bearerToken");
                token.accept_mutator(visitor)?;
                visitor.exit_scope();
                Ok(())
            }
            AuthenticationConfig::ApiKey { key, value, .. } => {
                visitor.enter_scope("apiKey");
                key.accept_mutator(visitor)?;
                value.accept_mutator(visitor)?;
                visitor.exit_scope();
                Ok(())
            }
            AuthenticationConfig::OAuth2 {
                token_url, client_id, client_secret, scopes, ..
            } => {
                visitor.enter_scope("oauth2");
                token_url.accept_mutator(visitor)?;
                client_id.accept_mutator(visitor)?;
                client_secret.accept_mutator(visitor)?;
                scopes.accept_mutator(visitor)?;
                visitor.exit_scope();
                Ok(())
            }
        }
    }
}
