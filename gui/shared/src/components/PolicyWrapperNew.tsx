import React, {FC, useState} from "react";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "shared/src/components/ui/accordion";
import {Button} from "shared/src/components/ui/button";
import {Plus, Trash} from "lucide-react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "shared/src/components/ui/select";
import {Input} from "shared/src/components/ui/input";
import {leftOperands, odrlActions, operators} from "shared/src/odrl_actions";
import {SubmitHandler} from "react-hook-form";

type ComponentType = "permission" | "obligation" | "prohibition";
type OperandType = "leftOperand" | "rightOperand" | "operator";

export const PolicyWrapperNew: FC<{
  onSubmit: SubmitHandler<any>;
}> = ({onSubmit}) => {
  const [newPolicy, setNewPolicy] = useState<OdrlInfo>({
    obligation: [],
    permission: [],
    prohibition: [],
  });

  const addComponentHandler = (componentType: ComponentType) => {
    const newComponent: OdrlPermission = {
      action: "",
      constraint: [],
    };
    const _newPolicy = {...newPolicy};
    _newPolicy[componentType].push(newComponent);
    setNewPolicy(_newPolicy);
    console.log(newPolicy);
  };
  const removeComponentHandler = (componentType: ComponentType, index: number) => {
    const _newPolicy = {...newPolicy};
    _newPolicy[componentType].splice(index, 1);
    console.log(_newPolicy);
    setNewPolicy(_newPolicy);
    console.log(newPolicy);
  };

  const addConstraintHandler = (componentType: ComponentType, componentIndex: number) => {
    const _newPolicy = {...newPolicy};
    _newPolicy[componentType][componentIndex].constraint.push({
      leftOperand: "",
      operator: "",
      rightOperand: "",
    });
    setNewPolicy(_newPolicy);
    console.log(newPolicy);
  };

  const removeConstraintHandler = (
    componentType: ComponentType,
    componentIndex: number,
    constraintIndex: number,
  ) => {
    const _newPolicy = {...newPolicy};
    _newPolicy[componentType][componentIndex].constraint.splice(constraintIndex, 1);
    setNewPolicy(_newPolicy);
    console.log(newPolicy);
  };

  const fieldValueChangeHandler = (
    componentType: ComponentType,
    componentIndex: number,
    value: string,
  ) => {
    const _newPolicy = {...newPolicy};
    _newPolicy[componentType][componentIndex].action = value;
    setNewPolicy(_newPolicy);
    console.log(newPolicy);
  };

  const operandValueChangeHandler = (
    componentType: ComponentType,
    componentIndex: number,
    constraintIndex: number,
    operand: OperandType,
    value: string,
  ) => {
    const _newPolicy = {...newPolicy};
    _newPolicy[componentType][componentIndex].constraint[constraintIndex][operand] = value;
    setNewPolicy(_newPolicy);
    console.log(newPolicy);
  };

  const submitHandler = () => {
    onSubmit(newPolicy);
    setNewPolicy({
      permission: [],
      obligation: [],
      prohibition: [],
    });
  };

  return (
    <div className="h-screen flex flex-col">
      <div className="flex flex-col gap-3 overflow-y-scroll h-[calc(100vh-180px)]  p-8 ">
        <Accordion type="single" collapsible className="w-full">
          <AccordionItem value="item-1" className="bg-success-500/10 border border-success-600/20">
            <AccordionTrigger
              className="text-white/70 flex bg-success-400/25 uppercase overflow-hidden rounded-md data-[state=open]:rounded-b-none">
              <div className="flex items-center w-full">
                <p className="text-current">permission</p>
              </div>
            </AccordionTrigger>
            <AccordionContent className="relative">
              <Button
                className="border-b border-white/15"
                policy="permission"
                variant="outline"
                size="xs"
                onClick={() => addComponentHandler("permission")}
              >
                <Plus/>
                Add permission
              </Button>
              {newPolicy.permission.map((permission, i) => (
                <div>
                  <div className="policy-item-create">
                    <div className="flex justify-between">
                      <p className="mb-2"> Action: </p>
                      <Button
                        variant="icon_destructive"
                        size="xs"
                        className="ml-4 "
                        onClick={() => removeComponentHandler("permission", i)}
                      >
                        <Trash className="mb-0.5"/>
                        Remove permission
                      </Button>
                    </div>
                    <Select
                      value={permission.action}
                      onValueChange={(value: string) =>
                        fieldValueChangeHandler("permission", i, value)
                      }
                    >
                      <SelectTrigger className="w-[240px]">
                        <SelectValue placeholder="Select action"/>
                      </SelectTrigger>
                      <SelectContent>
                        {odrlActions.map((odrlAction) => (
                          <SelectItem value={odrlAction}>{odrlAction}</SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    <div className="h-6"></div>
                    <p className="mb-2"> Constraints: </p>
                    {permission.constraint.map((constraint, j) => (
                      <div className="flex flex-col gap-2">
                        <div className="constraint-create mb-2 flex gap-3 justify-end items-end">
                          <Select
                            value={constraint.leftOperand}
                            onValueChange={(value: string) =>
                              operandValueChangeHandler("permission", i, j, "leftOperand", value)
                            }
                          >
                            <div className="flex flex-col">
                              <p className="text-xs text-gray-400 mb-1">Left Operand:</p>
                              <SelectTrigger className="w-[180px]">
                                <SelectValue placeholder="Select item"/>
                              </SelectTrigger>
                              <SelectContent>
                                {leftOperands.map((leftOperand) => (
                                  <SelectItem value={leftOperand}>{leftOperand}</SelectItem>
                                ))}
                              </SelectContent>
                            </div>
                          </Select>
                          <Select
                            value={constraint.operator}
                            onValueChange={(value: string) =>
                              operandValueChangeHandler("permission", i, j, "operator", value)
                            }
                          >
                            <div className="flex flex-col">
                              <p className="text-xs text-gray-400 mb-1">Operator:</p>
                              <SelectTrigger className="w-[140px]">
                                <SelectValue placeholder="Select operator"/>
                              </SelectTrigger>
                              <SelectContent>
                                {operators.map((operator) => (
                                  <SelectItem value={operator}>{operator}</SelectItem>
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
                                operandValueChangeHandler(
                                  "permission",
                                  i,
                                  j,
                                  "rightOperand",
                                  ev.currentTarget.value,
                                )
                              }
                            />
                          </div>
                          <div>
                            {/*  importante este div sino se deforma el boton */}
                            <Button
                              variant="icon_destructive"
                              size="icon"
                              onClick={() => removeConstraintHandler("permission", i, j)}
                            >
                              <Trash className="mb-0.5 "/>
                            </Button>
                          </div>
                        </div>
                      </div>
                    ))}
                    <Button
                      size="xs"
                      variant="outline"
                      className="mt-3"
                      onClick={() => addConstraintHandler("permission", i)}
                    >
                      <Plus/>
                      Add constraint
                    </Button>
                  </div>
                </div>
              ))}
            </AccordionContent>
          </AccordionItem>
        </Accordion>
        <Accordion type="single" collapsible className="w-full">
          <AccordionItem value="item-1" className="bg-warn-500/10 border border-warn-600/20">
            <AccordionTrigger
              className="text-white/70 flex bg-warn-400/25 uppercase overflow-hidden rounded-md data-[state=open]:rounded-b-none">
              <div className="flex items-center w-full">
                <p className="text-current">obligation</p>
              </div>
            </AccordionTrigger>
            <AccordionContent className="relative">
              <Button
                className="border-b border-white/15"
                policy="obligation"
                variant="outline"
                size="xs"
                onClick={() => addComponentHandler("obligation")}
              >
                <Plus/>
                Add obligation
              </Button>
              {newPolicy.obligation.map((obligation, i) => (
                <div>
                  <div className="policy-item-create">
                    <div className="flex justify-between">
                      <p className="mb-2"> Action: </p>
                      <Button
                        variant="icon_destructive"
                        size="xs"
                        className="ml-4"
                        onClick={() => removeComponentHandler("obligation", i)}
                      >
                        <Trash className="mb-0.5"/>
                        Remove obligation
                      </Button>
                    </div>
                    <Select
                      value={obligation.action}
                      onValueChange={(value: string) =>
                        fieldValueChangeHandler("obligation", i, value)
                      }
                    >
                      <SelectTrigger className="w-[240px]">
                        <SelectValue placeholder="Select action"/>
                      </SelectTrigger>
                      <SelectContent>
                        {odrlActions.map((odrlAction) => (
                          <SelectItem value={odrlAction}>{odrlAction}</SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    <div className="h-6"></div>
                    <p className="mb-2"> Constraints: </p>
                    {obligation.constraint.map((constraint, j) => (
                      <div className="flex flex-col gap-2">
                        <div className="constraint-create mb-2 flex gap-3 justify-end items-end">
                          <Select
                            value={constraint.leftOperand}
                            onValueChange={(value: string) =>
                              operandValueChangeHandler("obligation", i, j, "leftOperand", value)
                            }
                          >
                            <div className="flex flex-col">
                              <p className="text-xs text-gray-400 mb-1">Left Operand:</p>
                              <SelectTrigger className="w-[180px]">
                                <SelectValue placeholder="Select item"/>
                              </SelectTrigger>
                              <SelectContent>
                                {leftOperands.map((leftOperand) => (
                                  <SelectItem value={leftOperand}>{leftOperand}</SelectItem>
                                ))}
                              </SelectContent>
                            </div>
                          </Select>
                          <Select
                            value={constraint.operator}
                            onValueChange={(value: string) =>
                              operandValueChangeHandler("obligation", i, j, "operator", value)
                            }
                          >
                            <div className="flex flex-col">
                              <p className="text-xs text-gray-400 mb-1">Operator:</p>
                              <SelectTrigger className="w-[140px]">
                                <SelectValue placeholder="Select operator"/>
                              </SelectTrigger>
                              <SelectContent>
                                {operators.map((operator) => (
                                  <SelectItem value={operator}>{operator}</SelectItem>
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
                                operandValueChangeHandler(
                                  "obligation",
                                  i,
                                  j,
                                  "rightOperand",
                                  ev.currentTarget.value,
                                )
                              }
                            />
                          </div>
                          <div>
                            <Button
                              variant="icon_destructive"
                              size="icon"
                              onClick={() => removeConstraintHandler("obligation", i, j)}
                            >
                              <Trash className="mb-0.5"/>
                            </Button>
                          </div>
                        </div>
                      </div>
                    ))}
                    <Button
                      size="xs"
                      variant="outline"
                      className="mt-3"
                      onClick={() => addConstraintHandler("obligation", i)}
                    >
                      <Plus/>
                      Add constraint
                    </Button>
                  </div>
                </div>
              ))}
            </AccordionContent>
          </AccordionItem>
        </Accordion>
        <Accordion type="single" collapsible className="w-full">
          <AccordionItem
            value="item-1"
            className="bg-danger-500/10 border border-danger-600/20 mb-24"
          >
            <AccordionTrigger
              className="text-white/70 flex bg-danger-500/25 uppercase overflow-hidden rounded-md data-[state=open]:rounded-b-none">
              <div className="flex items-center w-full">
                <p className="text-current"> prohibition </p>
              </div>
            </AccordionTrigger>
            <AccordionContent className="relative">
              <Button
                className="border-b border-white/15"
                policy="prohibition"
                variant="outline"
                size="xs"
                onClick={() => addComponentHandler("prohibition")}
              >
                <Plus/>
                Add prohibition
              </Button>
              {newPolicy.prohibition.map((prohibition, i) => (
                <div>
                  <div className="policy-item-create">
                    <div className="flex justify-between">
                      <p className="mb-2"> Action: </p>
                      <Button
                        variant="icon_destructive"
                        size="xs"
                        className="ml-4"
                        onClick={() => removeComponentHandler("prohibition", i)}
                      >
                        <Trash className="mb-0.5"/>
                        Remove prohibition
                      </Button>
                    </div>
                    <Select
                      value={prohibition.action}
                      onValueChange={(value: string) =>
                        fieldValueChangeHandler("prohibition", i, value)
                      }
                    >
                      <SelectTrigger className="w-[240px]">
                        <SelectValue placeholder="Select action"/>
                      </SelectTrigger>
                      <SelectContent>
                        {odrlActions.map((odrlAction) => (
                          <SelectItem value={odrlAction}>{odrlAction}</SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                    <div className="h-6"></div>
                    <p className="mb-2"> Constraints: </p>
                    {prohibition.constraint.map((constraint, j) => (
                      <div className="flex flex-col gap-2">
                        <div className="constraint-create flex gap-3 justify-end items-end mb-2">
                          <Select
                            value={constraint.leftOperand}
                            onValueChange={(value: string) =>
                              operandValueChangeHandler("prohibition", i, j, "leftOperand", value)
                            }
                          >
                            <div className="flex flex-col">
                              <p className="text-xs text-gray-400 mb-1">Left Operand:</p>
                              <SelectTrigger className="w-[180px]">
                                <SelectValue placeholder="Select item"/>
                              </SelectTrigger>
                              <SelectContent>
                                {leftOperands.map((leftOperand) => (
                                  <SelectItem value={leftOperand}>{leftOperand}</SelectItem>
                                ))}
                              </SelectContent>
                            </div>
                          </Select>
                          <Select
                            value={constraint.operator}
                            onValueChange={(value: string) =>
                              operandValueChangeHandler("prohibition", i, j, "operator", value)
                            }
                          >
                            <div className="flex flex-col">
                              <p className="text-xs text-gray-400 mb-1">Operator:</p>
                              <SelectTrigger className="w-[140px]">
                                <SelectValue placeholder="Select operator"/>
                              </SelectTrigger>
                              <SelectContent>
                                {operators.map((operator) => (
                                  <SelectItem value={operator}>{operator}</SelectItem>
                                ))}
                              </SelectContent>
                            </div>
                          </Select>
                          <div className="flex flex-col">
                            <p className="text-xs text-gray-400 mb-1">Right Operand:</p>
                            <Input
                              value={constraint.rightOperand}
                              placeholder="Type value"
                              onChange={(ev) =>
                                operandValueChangeHandler(
                                  "prohibition",
                                  i,
                                  j,
                                  "rightOperand",
                                  ev.currentTarget.value,
                                )
                              }
                            />
                          </div>
                          <div>
                            <Button
                              variant="icon_destructive"
                              size="icon"
                              onClick={() => removeConstraintHandler("prohibition", i, j)}
                            >
                              <Trash className="mb-0.5"/>
                            </Button>
                          </div>
                        </div>
                      </div>
                    ))}
                    <Button
                      size="xs"
                      variant="outline"
                      className="mt-3"
                      onClick={() => addConstraintHandler("prohibition", i)}
                    >
                      <Plus/>
                      Add constraint
                    </Button>
                  </div>
                </div>
              ))}
            </AccordionContent>
          </AccordionItem>
        </Accordion>
      </div>
      <div className="h-24  bottom-0 bg-background border-t border-white/30 w-full p-6">
        <Button onClick={() => submitHandler()}>Add policy</Button>
      </div>
    </div>
  );
};
