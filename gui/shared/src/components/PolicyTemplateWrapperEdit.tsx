import React, { useEffect } from "react";
import { InfoList } from "shared/src/components/ui/info-list";
import Heading from "shared/src/components/ui/heading";
import { ControllerRenderProps, FormProvider, useForm } from "react-hook-form";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "shared/src/components/ui/accordion";
import { Input } from "shared/src/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "shared/src/components/ui/select";
import { FormField } from "shared/src/components/ui/form";
import { Button } from "shared/src/components/ui/button";

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


  const renderOperandOptions = (field: ControllerRenderProps, operand: string): React.ReactElement => {
    const options = policyTemplate.operand_options[operand];
    const formType = options.formType;
    const defaultValue = options.defaultValue;
    const innerOptions = options.options;
    let component;
    switch (formType) {
      case "date":
      case "text":
        component = <Input type={formType} defaultValue={defaultValue} {...field} />;
        break;
      case "select":
        component = (
          <Select defaultValue={defaultValue} onValueChange={field.onChange}>
            <SelectTrigger className="min-w-[200px] w-fit">
              <SelectValue placeholder="Select an option" />
            </SelectTrigger>
            <SelectContent>
              {innerOptions?.map((opt) => (
                <SelectItem className=" text-white/70 min-w-fit" key={opt.value} value={opt.value}>
                  {opt.label.find((l) => l["@language"] == "en")?.["@value"]}
                  {opt.value}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        );
        break;
      default:
        component = <></>
        break
    }
    return component;
  };


  const renderOperandOptionsFormControl = (operand: string) => {
    return (
      <FormField
        render={({ field }) => renderOperandOptions(field, operand)}
        name={operand}
        key={operand}
        control={form.control}
      />
    );
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
                <Accordion type="single" collapsible className="w-full">
                  <AccordionItem
                    value="item-1"
                    className="bg-success-500/10 border border-success-600/20"
                  >
                    <AccordionTrigger
                      className="text-white/70 flex bg-success-400/25 uppercase overflow-hidden rounded-md data-[state=open]:rounded-b-none">
                      <div className="flex items-center w-full">
                        <p className="text-current">permission</p>
                      </div>
                    </AccordionTrigger>
                    <AccordionContent className="relative ">
                      {(policyTemplate.content.permission == undefined ||
                        policyTemplate.content.permission?.length == 0) && <p>No policy defined</p>}
                      {policyTemplate.content.permission !== undefined &&
                        (policyTemplate.content.permission || []).map((permission, i) => (
                          <div className="border-b border-white/20 last:border-0 first:mt-0 mt-3">
                            <div className="policy-item-template">
                              <div className="flex gap-3">
                                <p className="mb-2 opacity-80 font-bold"> Action: </p>
                                <p> {permission.action.toUpperCase()} </p>
                              </div>
                              <div className="h-2"></div>
                              <p className="mb-1 opacity-80 font-bold"> Constraints: </p>
                              {permission.constraint?.map((constraint, j) => (
                                <div className="constraint-template-group">
                                  <div className="constraint-create flex gap-3">
                                    <div className="flex flex-col">
                                      <p className="text-xs text-gray-500 mb-1">Left Operand:</p>
                                      <p className="key-policy-template">
                                        {constraint.leftOperand}
                                      </p>
                                    </div>
                                    <div className="flex flex-col">
                                      <p className="text-xs text-gray-500 mb-1">Operator:</p>
                                      <p className="key-policy-template">{constraint.operator}</p>
                                    </div>
                                    <div className="flex flex-col">
                                      <p className="text-xs text-gray-500 mb-1">Right Operand:</p>
                                      <Select>
                                        {renderOperandOptionsFormControl(constraint.rightOperand)}
                                      </Select>
                                    </div>
                                  </div>
                                </div>
                              ))}
                            </div>
                          </div>
                        ))}
                    </AccordionContent>
                  </AccordionItem>
                </Accordion>
                <Accordion type="single" collapsible className="w-full">
                  <AccordionItem
                    value="item-1"
                    className="bg-warn-500/10 border border-warn-600/20"
                  >
                    <AccordionTrigger
                      className="text-white/70 flex bg-warn-400/25 uppercase overflow-hidden rounded-md data-[state=open]:rounded-b-none">
                      <div className="flex items-center w-full">
                        <p className="text-current">obligation</p>
                      </div>
                    </AccordionTrigger>
                    <AccordionContent className="relative">
                      {(policyTemplate.content.obligation == undefined ||
                        policyTemplate.content.obligation?.length == 0) && <p>No policy defined</p>}
                      {(policyTemplate.content.obligation || []).map((obligation, i) => (
                        <div>
                          <div className="policy-item-template">
                            <div className="flex gap-3">
                              <p className="mb-2 opacity-80 font-bold"> Action: </p>
                              <p>{obligation.action?.toUpperCase()}</p>
                            </div>
                            <div className="h-2"></div>
                            <p className="mb-2"> Constraints: </p>
                            {obligation.constraint?.map((constraint, j) => (
                              <div className="constraint-template-group">
                                <div className="constraint-create flex gap-3">
                                  <div className="flex flex-col">
                                    <p className="text-xs text-gray-500 mb-1">Left Operand:</p>
                                    <p className="key-policy-template">{constraint.leftOperand}</p>
                                  </div>
                                  <div className="flex flex-col">
                                    <p className="text-xs text-gray-500 mb-1">Operator:</p>
                                    <p className="key-policy-template">{constraint.operator}</p>
                                  </div>
                                  <div className="flex flex-col">
                                    <p className="text-xs text-gray-500 mb-1">Right Operand:</p>
                                    <div>
                                      {renderOperandOptionsFormControl(constraint.rightOperand)}
                                    </div>
                                  </div>
                                </div>
                              </div>
                            ))}
                          </div>
                        </div>
                      ))}
                    </AccordionContent>
                  </AccordionItem>
                </Accordion>
                <Accordion type="single" collapsible className="w-full">
                  <AccordionItem
                    value="item-1"
                    className="bg-danger-500/10 border border-danger-600/20"
                  >
                    <AccordionTrigger
                      className="text-white/70 flex bg-danger-500/25 uppercase overflow-hidden rounded-md data-[state=open]:rounded-b-none">
                      <div className="flex items-center w-full">
                        <p className="text-current">prohibition</p>
                      </div>
                    </AccordionTrigger>
                    <AccordionContent className="relative">
                      {(policyTemplate.content.prohibition == undefined ||
                        policyTemplate.content.prohibition?.length == 0) && (
                          <p>No policy defined</p>
                        )}
                      {(policyTemplate.content.prohibition || []).map((prohibition, i) => (
                        <div>
                          <div className="policy-item-template">
                            <div className="flex gap-3">
                              <p className="mb-2 opacity-80 font-bold"> Action: </p>
                              <p> {prohibition.action?.toUpperCase()} </p>
                            </div>
                            <div className="h-2"></div>
                            <p className="mb-2"> Constraints: </p>
                            {prohibition.constraint?.map((constraint, j) => (
                              <div className="constraint-template-group">
                                <div className="constraint-create flex gap-3">
                                  <div className="flex flex-col">
                                    <p className="text-xs text-gray-500 mb-1">Left Operand:</p>
                                    <p className="key-policy-template">{constraint.leftOperand}</p>
                                  </div>
                                  <div className="flex flex-col">
                                    <p className="text-xs text-gray-500 mb-1">Operator:</p>
                                    <p className="key-policy-template">{constraint.operator}</p>
                                  </div>
                                  <div className="flex flex-col">
                                    <p className="text-xs text-gray-500 mb-1">Right Operand:</p>
                                    <div>
                                      {renderOperandOptionsFormControl(constraint.rightOperand)}
                                    </div>
                                  </div>
                                </div>
                              </div>
                            ))}
                          </div>
                        </div>
                      ))}
                    </AccordionContent>
                  </AccordionItem>
                </Accordion>

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
