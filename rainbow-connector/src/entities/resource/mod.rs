use crate::entities::common::parameters::{TemplateMapString, TemplateString, TemplateVecString, TemplateVisitable};
use crate::entities::common::parameter_visitor::ParameterVisitor;
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

        if let Some(headers) = &self.headers {
            visitor.enter_scope("headers");
            headers.clone().accept(visitor)?;
            visitor.exit_scope();
        }

        if let Some(body_template) = &self.body_template {
            visitor.enter_scope("bodyTemplate");
            body_template.clone().accept(visitor)?;
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
