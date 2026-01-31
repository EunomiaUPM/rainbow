import React, { forwardRef, useEffect, useImperativeHandle, useState } from "react";
import Heading from "shared/src/components/ui/heading";
import { Badge } from "shared/src/components/ui/badge";
import { InfoList } from "shared/src/components/ui/info-list";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "shared/src/components/ui/accordion";
import { Button } from "shared/src/components/ui/button";
import { Plus, Trash } from "lucide-react";
import {
  ComponentType,
  OperandType,
  PolicyEditSection,
} from "shared/src/components/policy/PolicyEditSection";

export type PolicyEditorHandle = {
  getPolicy: () => OdrlInfo;
};

/**
 * Wrapper component for editing an ODRL policy.
 */
export const PolicyWrapperEdit = forwardRef<PolicyEditorHandle, { policy: OdrlOffer }>(
  ({ policy }, ref) => {
    const [newPolicy, setNewPolicy] = useState<OdrlInfo>({
      obligation: [],
      permission: [],
      prohibition: [],
    });

    useImperativeHandle(ref, () => ({
      getPolicy: () => {
        return newPolicy;
      },
    }));

    useEffect(() => {
      setNewPolicy({
        obligation: policy.obligation || [],
        permission: policy.permission || [],
        prohibition: policy.prohibition || [],
      });
    }, [policy]);

    const addComponentHandler = (componentType: ComponentType) => {
      const newComponent: OdrlPermission = {
        action: "",
        constraint: [],
      };
      const _newPolicy = { ...newPolicy };
      _newPolicy[componentType].push(newComponent);
      setNewPolicy(_newPolicy);
    };
    const removeComponentHandler = (componentType: ComponentType, index: number) => {
      const _newPolicy = { ...newPolicy };
      _newPolicy[componentType].splice(index, 1);

      setNewPolicy(_newPolicy);
    };

    const addConstraintHandler = (componentType: ComponentType, componentIndex: number) => {
      const _newPolicy = { ...newPolicy };
      _newPolicy[componentType][componentIndex].constraint.push({
        leftOperand: "",
        operator: "",
        rightOperand: "",
      });
      setNewPolicy(_newPolicy);
    };

    const removeConstraintHandler = (
      componentType: ComponentType,
      componentIndex: number,
      constraintIndex: number,
    ) => {
      const _newPolicy = { ...newPolicy };
      _newPolicy[componentType][componentIndex].constraint.splice(constraintIndex, 1);
      setNewPolicy(_newPolicy);
    };

    const fieldValueChangeHandler = (
      componentType: ComponentType,
      componentIndex: number,
      value: string,
    ) => {
      const _newPolicy = { ...newPolicy };
      _newPolicy[componentType][componentIndex].action = value;
      setNewPolicy(_newPolicy);
    };

    const operandValueChangeHandler = (
      componentType: ComponentType,
      componentIndex: number,
      constraintIndex: number,
      operand: OperandType,
      value: string,
    ) => {
      const _newPolicy = { ...newPolicy };
      _newPolicy[componentType][componentIndex].constraint[constraintIndex][operand] = value;
      setNewPolicy(_newPolicy);
    };

    return (
      <div className="w-full">
        <div className="border border-white/30 bg-white/10 p-4 pt-2 rounded-md justify-start">
          <div className="flex mb-4">
            <Heading level="h5" className="flex gap-3">
              <div>Policy with ID</div>
              <Badge variant="info" className="h-6">
                {policy["@id"].slice(9, 29) + "[...]"}
              </Badge>
            </Heading>
          </div>
          <InfoList
            items={[
              { label: "Policy Target", value: policy["@type"] },
              { label: "Profile", value: JSON.stringify(policy.profile) },
              { label: "Target", value: policy.target.slice(9) },
            ]}
          />
          <div className="h-5"></div>
          <Heading level="h6"> ODRL CONTENT</Heading>
          <div className="flex flex-col gap-2">
            <div className="flex flex-col gap-4">
              <PolicyEditSection
                type="permission"
                items={newPolicy.permission}
                onAdd={addComponentHandler}
                onRemove={removeComponentHandler}
                onActionChange={fieldValueChangeHandler}
                onAddConstraint={addConstraintHandler}
                onRemoveConstraint={removeConstraintHandler}
                onOperandChange={operandValueChangeHandler}
              />
              <PolicyEditSection
                type="obligation"
                items={newPolicy.obligation}
                onAdd={addComponentHandler}
                onRemove={removeComponentHandler}
                onActionChange={fieldValueChangeHandler}
                onAddConstraint={addConstraintHandler}
                onRemoveConstraint={removeConstraintHandler}
                onOperandChange={operandValueChangeHandler}
              />
              <PolicyEditSection
                type="prohibition"
                items={newPolicy.prohibition}
                onAdd={addComponentHandler}
                onRemove={removeComponentHandler}
                onActionChange={fieldValueChangeHandler}
                onAddConstraint={addConstraintHandler}
                onRemoveConstraint={removeConstraintHandler}
                onOperandChange={operandValueChangeHandler}
              />
            </div>
          </div>
        </div>
      </div>
    );
  },
);
