import React, { FC } from "react";
import { Trash } from "lucide-react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "shared/src/components/ui/select";
import { Input } from "shared/src/components/ui/input";
import { Button } from "shared/src/components/ui/button";
import { leftOperands, operators } from "shared/src/odrl_actions";
import { OperandType } from "shared/src/hooks/usePolicyForm";

interface ConstraintItemProps {
  constraint: OdrlConstraint;
  onUpdate: (operand: OperandType, value: string) => void;
  onRemove: () => void;
}

export const ConstraintItem: FC<ConstraintItemProps> = ({ constraint, onUpdate, onRemove }) => {
  return (
    <div className="flex flex-col gap-2">
      <div className="constraint-create mb-2 flex gap-3 justify-end items-end">
        <Select
          value={constraint.leftOperand}
          onValueChange={(value: string) => onUpdate("leftOperand", value)}
        >
          <div className="flex flex-col">
            <p className="text-xs text-gray-400 mb-1">Left Operand:</p>
            <SelectTrigger className="w-[180px]">
              <SelectValue placeholder="Select item" />
            </SelectTrigger>
            <SelectContent>
              {leftOperands.map((leftOperand) => (
                <SelectItem key={leftOperand} value={leftOperand}>
                  {leftOperand}
                </SelectItem>
              ))}
            </SelectContent>
          </div>
        </Select>

        <Select
          value={constraint.operator}
          onValueChange={(value: string) => onUpdate("operator", value)}
        >
          <div className="flex flex-col">
            <p className="text-xs text-gray-400 mb-1">Operator:</p>
            <SelectTrigger className="w-[140px]">
              <SelectValue placeholder="Select operator" />
            </SelectTrigger>
            <SelectContent>
              {operators.map((operator) => (
                <SelectItem key={operator} value={operator}>
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
            onChange={(ev) => onUpdate("rightOperand", ev.currentTarget.value)}
          />
        </div>

        <div>
          <Button variant="icon_destructive" size="icon" onClick={onRemove}>
            <Trash className="mb-0.5 " />
          </Button>
        </div>
      </div>
    </div>
  );
};
