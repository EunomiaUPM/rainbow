import React from "react";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "shared/src/components/ui/accordion";
import { FormField } from "shared/src/components/ui/form";
import { Input } from "shared/src/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "shared/src/components/ui/select";
import { Control, ControllerRenderProps } from "react-hook-form";

interface PolicyTemplateSectionProps {
  type: "permission" | "obligation" | "prohibition";
  items: any[] | undefined;
  operandOptions: any;
  control: Control<any>;
}

const styles = {
  permission: {
    accordionItem: "bg-success-500/10 border border-success-600/20",
    trigger: "bg-success-400/25",
  },
  obligation: {
    accordionItem: "bg-warn-500/10 border border-warn-600/20",
    trigger: "bg-warn-400/25",
  },
  prohibition: {
    accordionItem: "bg-danger-500/10 border border-danger-600/20",
    trigger: "bg-danger-500/25",
  },
};

export const PolicyTemplateSection: React.FC<PolicyTemplateSectionProps> = ({
  type,
  items,
  operandOptions,
  control,
}) => {
  const currentStyle = styles[type];

  const renderOperandOptions = (
    field: ControllerRenderProps,
    operand: string,
  ): React.ReactElement => {
    const options = operandOptions[operand];
    if (!options) return <></>; // Guard clause

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
              {innerOptions?.map((opt: any) => (
                <SelectItem className=" text-white/70 min-w-fit" key={opt.value} value={opt.value}>
                  {opt.label?.find((l: any) => l["@language"] == "en")?.["@value"]}
                  {opt.value}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        );
        break;
      default:
        component = <></>;
        break;
    }
    return component;
  };

  const renderOperandOptionsFormControl = (operand: string) => {
    return (
      <FormField
        render={({ field }) => renderOperandOptions(field, operand)}
        name={operand}
        key={operand}
        control={control}
      />
    );
  };

  return (
    <Accordion type="single" collapsible className="w-full">
      <AccordionItem value="item-1" className={currentStyle.accordionItem}>
        <AccordionTrigger
          className={`text-white/70 flex ${currentStyle.trigger} uppercase overflow-hidden rounded-md data-[state=open]:rounded-b-none`}
        >
          <div className="flex items-center w-full">
            <p className="text-current">{type}</p>
          </div>
        </AccordionTrigger>
        <AccordionContent className="relative">
          {(!items || items.length === 0) && <p>No policy defined</p>}
          {(items || []).map((item, i) => (
            <div
              key={i}
              className="border-b border-white/20 last:border-0 first:mt-0 mt-3"
            >
              <div className="policy-item-template">
                <div className="flex gap-3">
                  <p className="mb-2 opacity-80 font-bold"> Action: </p>
                  <p> {item.action?.toUpperCase()} </p>
                </div>
                <div className="h-2"></div>
                <p className="mb-1 opacity-80 font-bold"> Constraints: </p>
                {item.constraint?.map((constraint: any, j: number) => (
                  <div className="constraint-template-group" key={j}>
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
  );
};
