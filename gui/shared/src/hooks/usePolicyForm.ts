import { useState } from "react";

export type ComponentType = "permission" | "obligation" | "prohibition";
export type OperandType = "leftOperand" | "rightOperand" | "operator";

export const usePolicyForm = (initialPolicy?: OdrlInfo) => {
  const [newPolicy, setNewPolicy] = useState<OdrlInfo>(initialPolicy || {
    obligation: [],
    permission: [],
    prohibition: [],
  });

  const addComponentHandler = (componentType: ComponentType) => {
    const newComponent: OdrlPermission = {
      action: "",
      constraint: [],
    };
    const _newPolicy = { ...newPolicy };
    // @ts-ignore
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
    // @ts-ignore
    _newPolicy[componentType][componentIndex].constraint[constraintIndex][operand] = value;
    setNewPolicy(_newPolicy);
  };

  const resetPolicy = () => {
    setNewPolicy({
      permission: [],
      obligation: [],
      prohibition: [],
    });
  };

  return {
    policy: newPolicy,
    addComponent: addComponentHandler,
    removeComponent: removeComponentHandler,
    addConstraint: addConstraintHandler,
    removeConstraint: removeConstraintHandler,
    updateField: fieldValueChangeHandler,
    updateOperand: operandValueChangeHandler,
    resetPolicy,
  };
};
