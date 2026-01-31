import React from "react";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "shared/src/components/ui/accordion";
import { Button } from "shared/src/components/ui/button";
import { Plus, Trash } from "lucide-react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "shared/src/components/ui/select";
import { Input } from "shared/src/components/ui/input";
import { leftOperands, odrlActions, operators } from "shared/src/odrl_actions";

export type ComponentType = "permission" | "obligation" | "prohibition";
export type OperandType = "leftOperand" | "rightOperand" | "operator";

interface PolicyEditSectionProps {
  type: ComponentType;
  items: any[]; // OdrlPermission[] | OdrlObligation[] | OdrlProhibition[] - usage is generic enough
  onAdd: (type: ComponentType) => void;
  onRemove: (type: ComponentType, index: number) => void;
  onActionChange: (type: ComponentType, index: number, value: string) => void;
  onAddConstraint: (type: ComponentType, index: number) => void;
  onRemoveConstraint: (type: ComponentType, index: number, constraintIndex: number) => void;
  onOperandChange: (
    type: ComponentType,
    index: number,
    constraintIndex: number,
    operand: OperandType,
    value: string
  ) => void;
}

const styles = {
  permission: {
    accordionItem: "bg-success-500/10 border border-success-600/20",
    trigger: "bg-success-400/25",
    removeIcon: "text-foreground",
  },
  obligation: {
    accordionItem: "bg-warn-500/10 border border-warn-600/20",
    trigger: "bg-warn-400/25",
    removeIcon: "text-danger",
  },
  prohibition: {
    accordionItem: "bg-danger-500/10 border border-danger-600/20",
    trigger: "bg-danger-500/25",
    removeIcon: "text-foreground",
  },
};

export const PolicyEditSection: React.FC<PolicyEditSectionProps> = ({
  type,
  items,
  onAdd,
  onRemove,
  onActionChange,
  onAddConstraint,
  onRemoveConstraint,
  onOperandChange,
}) => {
  const currentStyle = styles[type];

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
          <Button
            className="border-b border-white/15"
            variant="outline"
            size="xs"
            onClick={() => onAdd(type)}
          >
            <Plus />
            Add {type}
          </Button>
          {items.map((item, i) => (
            <div key={`${type}-${i}-${item.action || "new"}`}>
              <div className="policy-item-create">
                <div className="flex justify-between">
                  <p className="mb-2"> Action: </p>
                  <Button
                    variant="icon_destructive"
                    size="xs"
                    className="ml-4 border"
                    onClick={() => onRemove(type, i)}
                  >
                    <Trash className={`mb-0.5 ${currentStyle.removeIcon}`} />
                    Remove {type}
                  </Button>
                </div>
                <Select
                  onValueChange={(value: string) => onActionChange(type, i, value)}
                  value={item.action}
                >
                  <SelectTrigger className="w-[240px]">
                    <SelectValue placeholder="Select action" />
                  </SelectTrigger>
                  <SelectContent>
                    {odrlActions.map((odrlAction) => (
                      <SelectItem value={odrlAction} key={odrlAction}>
                        {odrlAction}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
                <div className="h-6"></div>
                <p className="mb-2"> Constraints: </p>
                {item.constraint?.map((constraint: any, j: number) => (
                  <div className="flex flex-col gap-2" key={j}>
                    <div className="constraint-create flex gap-3 items-end">
                      <Select
                        onValueChange={(value: string) =>
                          onOperandChange(type, i, j, "leftOperand", value)
                        }
                        value={constraint.leftOperand}
                      >
                        <div className="flex flex-col">
                          <p className="text-xs text-gray-400 mb-1">Left Operand:</p>
                          <SelectTrigger className="w-[180px]">
                            <SelectValue placeholder="Select item" />
                          </SelectTrigger>
                          <SelectContent>
                            {leftOperands.map((leftOperand) => (
                              <SelectItem value={leftOperand} key={leftOperand}>
                                {leftOperand}
                              </SelectItem>
                            ))}
                          </SelectContent>
                        </div>
                      </Select>
                      <Select
                        onValueChange={(value: string) =>
                          onOperandChange(type, i, j, "operator", value)
                        }
                        value={constraint.operator}
                      >
                        <div className="flex flex-col">
                          <p className="text-xs text-gray-400 mb-1">Operator:</p>
                          <SelectTrigger className="w-[140px]">
                            <SelectValue placeholder="Select operator" />
                          </SelectTrigger>
                          <SelectContent>
                            {operators.map((operator) => (
                              <SelectItem value={operator} key={operator}>
                                {operator}
                              </SelectItem>
                            ))}
                          </SelectContent>
                        </div>
                      </Select>
                      <div className="flex flex-col">
                        <p className="text-xs text-gray-400 mb-1">Right Operand:</p>
                        <Input
                          placeholder="Type value"
                          value={constraint.rightOperand}
                          onChange={(ev) =>
                            onOperandChange(type, i, j, "rightOperand", ev.currentTarget.value)
                          }
                        />
                      </div>
                      <div>
                        <Button
                          variant="icon_destructive"
                          size="icon"
                          onClick={() => onRemoveConstraint(type, i, j)}
                        >
                          <Trash className="mb-0.5" />
                        </Button>
                      </div>
                    </div>
                  </div>
                ))}
                <Button
                  size="xs"
                  variant="outline"
                  className="mt-3"
                  onClick={() => onAddConstraint(type, i)}
                >
                  <Plus />
                  Add constraint
                </Button>
              </div>
            </div>
          ))}
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  );
};
