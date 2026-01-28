use crate::entities::instantiation_engine::validators::ValidatorFactory;
use crate::entities::instantiation_engine::NewPolicyInstantiationDto;
use crate::entities::policy_templates::PolicyTemplateDto;
use anyhow::anyhow;
use rainbow_common::errors::CommonErrors;
use tracing::error;

impl NewPolicyInstantiationDto {
    pub(crate) fn validate_instantiation_request(
        &self,
        policy_template: &PolicyTemplateDto,
    ) -> anyhow::Result<()> {
        for req_key in self.parameters.keys() {
            if !policy_template.parameters.contains_key(req_key) {
                let err = CommonErrors::parse_new(&format!(
                    "Validation Error: Unknown parameter '{}' provided. It is not defined in the template.",
                    req_key
                ));
                error!("{:?}", err);
                return Err(anyhow!(err));
            }
        }

        for (param_key, param_def) in &policy_template.parameters {
            let value_to_validate = match self.parameters.get(param_key) {
                Some(val) => val,
                None => {
                    if let Some(default_val) = &param_def.default_value {
                        default_val
                    } else {
                        let err = CommonErrors::parse_new(&format!(
                            "Validation Error: Missing required parameter '{}'",
                            param_key
                        ));
                        error!("{:?}", err);
                        return Err(anyhow!(err));
                    }
                }
            };

            let validator = ValidatorFactory::get_validator(param_def.data_type);
            validator.validate(value_to_validate, &param_def.restrictions).map_err(|e| {
                let err = CommonErrors::parse_new(&format!(
                    "Validation Error for '{}': {:?}",
                    param_key, e
                ));
                error!("{:?}", err);
                anyhow!(err)
            })?;
        }

        Ok(())
    }
}
