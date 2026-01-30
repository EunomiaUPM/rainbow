use crate::entities::common::parameter_mutator::TemplateMutator;
use crate::entities::common::parameter_visitor::ParameterVisitor;
use crate::entities::common::parameters::{
    TemplateMapString, TemplateMutable, TemplateString, TemplateVecString, TemplateVisitable,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "protocol")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProtocolSpec {
    Http(HttpSpec),
    Kafka(KafkaSpec),
}

impl TemplateVisitable for ProtocolSpec {
    fn accept<V: ParameterVisitor>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            ProtocolSpec::Http(spec) => {
                visitor.enter_scope("http");
                spec.accept(visitor)?;
                visitor.exit_scope();
            }
            ProtocolSpec::Kafka(spec) => {
                visitor.enter_scope("kafka");
                spec.accept(visitor)?;
                visitor.exit_scope();
            }
        }
        Ok(())
    }
}

impl TemplateMutable for ProtocolSpec {
    fn accept_mutator<V: TemplateMutator>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        match self {
            ProtocolSpec::Http(spec) => {
                visitor.enter_scope("http");
                spec.accept_mutator(visitor)?;
                visitor.exit_scope();
            }
            ProtocolSpec::Kafka(spec) => {
                visitor.enter_scope("kafka");
                spec.accept_mutator(visitor)?;
                visitor.exit_scope();
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpSpec {
    pub url_template: TemplateString,
    pub method: TemplateVecString,
    pub headers: Option<TemplateMapString>,
    pub body_template: Option<TemplateString>,
}

impl TemplateVisitable for HttpSpec {
    fn accept<V: ParameterVisitor>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        visitor.enter_scope("urlTemplate");
        self.url_template.accept(visitor)?;
        visitor.exit_scope();

        visitor.enter_scope("method");
        self.method.accept(visitor)?;
        visitor.exit_scope();

        if let Some(headers) = &mut self.headers {
            visitor.enter_scope("headers");
            headers.clone().accept(visitor)?;
            visitor.exit_scope();
        }

        if let Some(body_template) = &mut self.headers {
            visitor.enter_scope("bodyTemplate");
            body_template.clone().accept(visitor)?;
            visitor.exit_scope();
        }
        Ok(())
    }
}

impl TemplateMutable for HttpSpec {
    fn accept_mutator<V: TemplateMutator>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        visitor.enter_scope("urlTemplate");
        self.url_template.accept_mutator(visitor)?;
        visitor.exit_scope();

        visitor.enter_scope("method");
        self.method.accept_mutator(visitor)?;
        visitor.exit_scope();

        if let Some(headers) = &mut self.headers {
            visitor.enter_scope("headers");
            headers.accept_mutator(visitor)?;
            visitor.exit_scope();
        }

        if let Some(body) = &mut self.body_template {
            visitor.enter_scope("bodyTemplate");
            body.accept_mutator(visitor)?;
            visitor.exit_scope();
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KafkaSpec {
    pub brokers: TemplateVecString,
    pub topic: TemplateString,
    pub group_id: Option<TemplateString>,
}

impl TemplateVisitable for KafkaSpec {
    fn accept<V: ParameterVisitor>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        visitor.enter_scope("brokers");
        self.brokers.accept(visitor)?;
        visitor.exit_scope();

        visitor.enter_scope("topic");
        self.topic.accept(visitor)?;
        visitor.exit_scope();

        if let Some(group_id) = &self.group_id {
            visitor.enter_scope("groupId");
            group_id.clone().accept(visitor)?;
            visitor.exit_scope();
        }
        Ok(())
    }
}

impl TemplateMutable for KafkaSpec {
    fn accept_mutator<V: TemplateMutator>(&mut self, visitor: &mut V) -> Result<(), V::Error> {
        visitor.enter_scope("brokers");
        self.brokers.accept_mutator(visitor)?;
        visitor.exit_scope();

        visitor.enter_scope("topic");
        self.topic.accept_mutator(visitor)?;
        visitor.exit_scope();

        if let Some(group_id) = &self.group_id {
            visitor.enter_scope("groupId");
            group_id.clone().accept_mutator(visitor)?;
            visitor.exit_scope();
        }
        Ok(())
    }
}
