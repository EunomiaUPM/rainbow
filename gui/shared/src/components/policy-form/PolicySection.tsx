import React, { FC } from "react";
import {
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "shared/src/components/ui/accordion";
import { Button, ButtonProps } from "shared/src/components/ui/button";
import { Plus } from "lucide-react";
import { PolicyItem } from "./PolicyItem";
import { ComponentType, OperandType } from "shared/src/hooks/usePolicyForm";

interface PolicySectionProps {
  type: ComponentType;
  items: OdrlPermission[];
  accordionValue: string;
  accordionStyles: string;
  triggerStyles: string;
  onAdd: () => void;
  onRemove: (index: number) => void;
  onUpdateAction: (index: number, value: string) => void;
  onAddConstraint: (index: number) => void;
  onRemoveConstraint: (itemIndex: number, constraintIndex: number) => void;
  onUpdateConstraint: (
    itemIndex: number,
    constraintIndex: number,
    operand: OperandType,
    value: string
  ) => void;
}

export const PolicySection: FC<PolicySectionProps> = ({
  type,
  items,
  accordionValue,
  accordionStyles,
  triggerStyles,
  onAdd,
  onRemove,
  onUpdateAction,
  onAddConstraint,
  onRemoveConstraint,
  onUpdateConstraint,
}) => {
  return (
    <AccordionItem value={accordionValue} className={accordionStyles}>
      <AccordionTrigger className={`${triggerStyles} text-white/70 flex uppercase overflow-hidden rounded-md data-[state=open]:rounded-b-none`}>
        <div className="flex items-center w-full">
          <p className="text-current">{type}</p>
        </div>
      </AccordionTrigger>
      <AccordionContent className="relative">
        <Button
          className="border-b border-white/15"
          // @ts-ignore - The `policy` prop seems custom on Button or passed down? Keeping it for safety if styled-components rely on it, though it's likely invalid HTML attribute.
          policy={type}
          variant="outline"
          size="xs"
          onClick={onAdd}
        >
          <Plus />
          Add {type}
        </Button>
        {items.map((item, i) => (
          <PolicyItem
            key={i}
            item={item}
            typeLabel={type}
            onRemove={() => onRemove(i)}
            onUpdateAction={(value) => onUpdateAction(i, value)}
            onAddConstraint={() => onAddConstraint(i)}
            onRemoveConstraint={(constraintIndex) =>
              onRemoveConstraint(i, constraintIndex)
            }
            onUpdateConstraint={(constraintIndex, operand, value) =>
              onUpdateConstraint(i, constraintIndex, operand, value)
            }
          />
        ))}
      </AccordionContent>
    </AccordionItem>
  );
};
