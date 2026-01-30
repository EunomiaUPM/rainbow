import React, { FC } from "react";
import { Plus, Trash } from "lucide-react";
import { Button } from "shared/src/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "shared/src/components/ui/select";
import { odrlActions } from "shared/src/odrl_actions";
import { ConstraintItem } from "./ConstraintItem";
import { OperandType } from "shared/src/hooks/usePolicyForm";

interface PolicyItemProps {
  item: OdrlPermission; // Assuming OdrlPermission covers the shape for all types
  typeLabel: string;
  onRemove: () => void;
  onUpdateAction: (value: string) => void;
  onAddConstraint: () => void;
  onRemoveConstraint: (index: number) => void;
  onUpdateConstraint: (index: number, operand: OperandType, value: string) => void;
}

export const PolicyItem: FC<PolicyItemProps> = ({
  item,
  typeLabel,
  onRemove,
  onUpdateAction,
  onAddConstraint,
  onRemoveConstraint,
  onUpdateConstraint,
}) => {
  return (
    <div>
      <div className="policy-item-create">
        <div className="flex justify-between">
          <p className="mb-2"> Action: </p>
          <Button
            variant="icon_destructive"
            size="xs"
            className="ml-4"
            onClick={onRemove}
          >
            <Trash className="mb-0.5" />
            Remove {typeLabel}
          </Button>
        </div>
        <Select
          value={item.action}
          onValueChange={onUpdateAction}
        >
          <SelectTrigger className="w-[240px]">
            <SelectValue placeholder="Select action" />
          </SelectTrigger>
          <SelectContent>
            {odrlActions.map((odrlAction) => (
              <SelectItem key={odrlAction} value={odrlAction}>
                {odrlAction}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        <div className="h-6"></div>
        <p className="mb-2"> Constraints: </p>
        {item.constraint.map((constraint, j) => (
          <ConstraintItem
            key={j}
            constraint={constraint}
            onRemove={() => onRemoveConstraint(j)}
            onUpdate={(operand, value) => onUpdateConstraint(j, operand, value)}
          />
        ))}
        <Button
          size="xs"
          variant="outline"
          className="mt-3"
          onClick={onAddConstraint}
        >
          <Plus />
          Add constraint
        </Button>
      </div>
    </div>
  );
};
