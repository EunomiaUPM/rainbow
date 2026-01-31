import React, { useEffect } from "react";
import { InfoList } from "shared/src/components/ui/info-list";
import Heading from "shared/src/components/ui/heading";
import { ControllerRenderProps, FormProvider, useForm } from "react-hook-form";
import { Button } from "shared/src/components/ui/button";
import { PolicyTemplateSection } from "shared/src/components/policy/PolicyTemplateSection";

type DynamicFormValues = {
  [key: string]: string;
};

/**
 * Component for editing a policy template.
 */
export const PolicyTemplateWrapperEdit = ({
  policyTemplate,
  onSubmit,
}: {
  policyTemplate: PolicyTemplate;
  onSubmit: (odrlContent: OdrlInfo) => Promise<void>;
}) => {
  const form = useForm<DynamicFormValues>({
    defaultValues: {},
  });


  useEffect(() => {
    const initialValues: DynamicFormValues = {};
    Object.entries(policyTemplate.operand_options).forEach(([key, value]) => {
      if (key && value.defaultValue !== undefined) {
        initialValues[key] = value.defaultValue;
      }
    });
    form.reset(initialValues);
  }, [policyTemplate, form]);


  const submitHandler = async (
    formData: DynamicFormValues,
    currentPolicyTemplate: PolicyTemplate,
  ) => {
    let odrlContent = JSON.parse(JSON.stringify(currentPolicyTemplate.content));
    for (const key in formData) {
      if (formData.hasOwnProperty(key)) {
        const value = formData[key];
        let contentString = JSON.stringify(odrlContent);
        const escapedKey = key.replace(/[$]/g, "\\$&");
        contentString = contentString.replace(new RegExp(escapedKey, "g"), value);
        odrlContent = JSON.parse(contentString);
      }
    }
    await onSubmit(odrlContent);
    form.reset();
  };




  return (
    <div>
      <FormProvider {...form}>
        <form
          onSubmit={form.handleSubmit((data) => submitHandler(data, policyTemplate))}
          className="space-y-4 "
        >
          <div className="border border-white/30 bg-white/10 px-4 py-2 pb-4 rounded-md justify-start max-h-[80vh] overflow-y-auto">
            <div className="flex mb-4">
              <Heading level="h5" className="flex gap-3">
                <span className="font-light">Policy template:</span>
                {policyTemplate.title}
              </Heading>
            </div>

            <InfoList
              items={[{ label: "Description", value: policyTemplate.description }]}
            />
            <div className="min-h-5"></div>
            <Heading level="h6"> ODRL CONTENT</Heading>
            <p className="mb-2"> Edit the constraint value</p>
            <div className="flex flex-col gap-2">
              <div className="flex flex-col gap-3">
                <PolicyTemplateSection
                  type="permission"
                  items={policyTemplate.content.permission}
                  operandOptions={policyTemplate.operand_options}
                  control={form.control}
                />
                <PolicyTemplateSection
                  type="obligation"
                  items={policyTemplate.content.obligation}
                  operandOptions={policyTemplate.operand_options}
                  control={form.control}
                />
                <PolicyTemplateSection
                  type="prohibition"
                  items={policyTemplate.content.prohibition}
                  operandOptions={policyTemplate.operand_options}
                  control={form.control}
                />

              </div>
            </div>
          </div>
          <Button className="w-fit mt-4" type="submit">
            Create policy
          </Button>
        </form>
      </FormProvider>
    </div>
  );
};
