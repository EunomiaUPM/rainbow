use crate::entities::auth_config::AuthenticationConfig;
use crate::entities::common::parameter_visitor::ParameterVisitor;
use crate::entities::common::parameters::TemplateVisitable;

impl TemplateVisitable for AuthenticationConfig {
    fn accept<V: ParameterVisitor>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        //
        match self {
            AuthenticationConfig::NoAuth => Ok(()),
            AuthenticationConfig::BasicAuth(config) => {
                visitor.enter_scope("basicAuth");
                config.username.accept(visitor)?;
                config.password.accept(visitor)?;
                visitor.exit_scope();
                Ok(())
            }
            AuthenticationConfig::BearerToken { token } => {
                visitor.enter_scope("bearerToken");
                token.accept(visitor)?;
                visitor.exit_scope();
                Ok(())
            }
            AuthenticationConfig::ApiKey { key, value, .. } => {
                visitor.enter_scope("apiKey");
                key.accept(visitor)?;
                value.accept(visitor)?;
                visitor.exit_scope();
                Ok(())
            }
            AuthenticationConfig::OAuth2 {
                token_url, client_id, client_secret, scopes, ..
            } => {
                visitor.enter_scope("oauth2");
                token_url.accept(visitor)?;
                client_id.accept(visitor)?;
                client_secret.accept(visitor)?;
                scopes.accept(visitor)?;
                visitor.exit_scope();
                Ok(())
            }
        }
    }
}
