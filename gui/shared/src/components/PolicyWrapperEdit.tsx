import React, { useEffect, useState } from "react";
import Heading from "@/components/ui/heading.tsx";
import { Badge } from "@/components/ui/badge.tsx";
import { List, ListItem, ListItemKey } from "@/components/ui/list.tsx";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion.tsx";
import { Button } from "@/components/ui/button.tsx";
import { Plus, Trash } from "lucide-react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select.tsx";
import { leftOperands, odrlActions, operators } from "@/odrl_actions";
import { Input } from "@/components/ui/input.tsx";

type ComponentType = "permission" | "obligation" | "prohibition";
type OperandType = "leftOperand" | "rightOperand" | "operator";

export const PolicyWrapperEdit = ({
  policy,
  onSubmit,
}: {
  policy: OdrlOffer;
  onSubmit: any;
}) => {
  const [newPolicy, setNewPolicy] = useState<OdrlInfo>({
    obligation: [],
    permission: [],
    prohibition: [],
  });

  useEffect(() => {
    console.log(policy);
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
    console.log(newPolicy);
  };
  const removeComponentHandler = (
    componentType: ComponentType,
    index: number
  ) => {
    const _newPolicy = { ...newPolicy };
    _newPolicy[componentType].splice(index, 1);
    console.log(_newPolicy);
    setNewPolicy(_newPolicy);
    console.log(newPolicy);
  };

  const addConstraintHandler = (
    componentType: ComponentType,
    componentIndex: number
  ) => {
    const _newPolicy = { ...newPolicy };
    _newPolicy[componentType][componentIndex].constraint.push({
      leftOperand: "aa",
      operator: "eq",
      rightOperand: "bbb",
    });
    setNewPolicy(_newPolicy);
    console.log(newPolicy);
  };

  const removeConstraintHandler = (
    componentType: ComponentType,
    componentIndex: number,
    constraintIndex: number
  ) => {
    const _newPolicy = { ...newPolicy };
    _newPolicy[componentType][componentIndex].constraint.splice(
      constraintIndex,
      1
    );
    setNewPolicy(_newPolicy);
    console.log(newPolicy);
  };

  const fieldValueChangeHandler = (
    componentType: ComponentType,
    componentIndex: number,
    value: string
  ) => {
    const _newPolicy = { ...newPolicy };
    _newPolicy[componentType][componentIndex].action = value;
    setNewPolicy(_newPolicy);
    console.log(newPolicy);
  };

  const operandValueChangeHandler = (
    componentType: ComponentType,
    componentIndex: number,
    constraintIndex: number,
    operand: OperandType,
    value: string
  ) => {
    const _newPolicy = { ...newPolicy };
    _newPolicy[componentType][componentIndex].constraint[constraintIndex][
      operand
    ] = value;
    setNewPolicy(_newPolicy);
    console.log(newPolicy);
  };

  const submitHandler = () => {
    onSubmit(newPolicy);
    setNewPolicy({
      permission: [],
      prohibition: [],
      obligation: [],
    });
  };

  return (
    <div>
      <List className=" border border-white/30 bg-white/10 px-4 py-2 rounded-md justify-start">
        <div className="flex">
          <Heading level="h5" className="flex gap-3">
            <div>Policy with ID</div>
            <Badge variant="info" className="h-6">
              {policy["@id"].slice(9, 29) + "[...]"}
            </Badge>
          </Heading>
        </div>
        <ListItem>
          <ListItemKey>Policy Target</ListItemKey>
          <p>{policy["@type"]}</p>
        </ListItem>
        <ListItem>
          <ListItemKey> Profile</ListItemKey>
          <p className="whitespace-normal"> {JSON.stringify(policy.profile)}</p>
        </ListItem>
        <ListItem>
          <ListItemKey> Target</ListItemKey>
          <p> {policy.target.slice(9)}</p>
        </ListItem>
        <div className="h-5"></div>
        <Heading level="h6"> ODRL CONTENT</Heading>
        <div className="flex flex-col gap-2">
          <div className="flex flex-col gap-4">
            <Accordion type="single" collapsible className="w-full">
              <AccordionItem
                value="item-1"
                className="bg-success-500/10 border border-success-600/20"
              >
                <AccordionTrigger className="text-white/70 flex bg-success-400/25 uppercase overflow-hidden rounded-md data-[state=open]:rounded-b-none">
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
                    <Plus />
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
                            className="ml-4 border"
                            onClick={() =>
                              removeComponentHandler("permission", i)
                            }
                          >
                            <Trash className="mb-0.5" />
                            Remove permission
                          </Button>
                        </div>
                        <Select
                          onValueChange={(value: string) =>
                            fieldValueChangeHandler("permission", i, value)
                          }
                          value={permission.action}
                        >
                          <SelectTrigger className="w-[240px]">
                            <SelectValue placeholder="Select action" />
                          </SelectTrigger>
                          <SelectContent>
                            {odrlActions.map((odrlAction) => (
                              <SelectItem value={odrlAction}>
                                {odrlAction}
                              </SelectItem>
                            ))}
                          </SelectContent>
                        </Select>
                        <div className="h-6"></div>
                        <p className="mb-2"> Constraints: </p>
                        {permission.constraint.map((constraint, j) => (
                          <div className="flex flex-col gap-2">
                            <div className="constraint-create flex gap-3 items-end">
                              <Select
                                onValueChange={(value: string) =>
                                  operandValueChangeHandler(
                                    "permission",
                                    i,
                                    j,
                                    "leftOperand",
                                    value
                                  )
                                }
                                value={constraint.leftOperand}
                              >
                                <div className="flex flex-col">
                                  <p className="text-xs text-gray-400 mb-1">
                                    Left Operand:
                                  </p>
                                  <SelectTrigger className="w-[180px]">
                                    <SelectValue placeholder="Select item" />
                                  </SelectTrigger>
                                  <SelectContent>
                                    {leftOperands.map((leftOperand) => (
                                      <SelectItem value={leftOperand}>
                                        {leftOperand}
                                      </SelectItem>
                                    ))}
                                  </SelectContent>
                                </div>
                              </Select>
                              <Select
                                onValueChange={(value: string) =>
                                  operandValueChangeHandler(
                                    "permission",
                                    i,
                                    j,
                                    "operator",
                                    value
                                  )
                                }
                                value={constraint.operator}
                              >
                                <div className="flex flex-col">
                                  <p className="text-xs text-gray-400 mb-1">
                                    Operator:
                                  </p>
                                  <SelectTrigger className="w-[140px]">
                                    <SelectValue placeholder="Select operator" />
                                  </SelectTrigger>
                                  <SelectContent>
                                    {operators.map((operator) => (
                                      <SelectItem value={operator}>
                                        {operator}
                                      </SelectItem>
                                    ))}
                                  </SelectContent>
                                </div>
                              </Select>
                              <div className="flex flex-col">
                                <p className="text-xs text-gray-400 mb-1">
                                  Right Operand:
                                </p>
                                <Input
                                  placeholder="Type value"
                                  value={constraint.rightOperand}
                                  onChange={(ev) =>
                                    operandValueChangeHandler(
                                      "permission",
                                      i,
                                      j,
                                      "rightOperand",
                                      ev.currentTarget.value
                                    )
                                  }
                                />
                              </div>
                              <div>
                                <Button
                                  variant="icon_destructive"
                                  size="icon"
                                  onClick={() =>
                                    removeConstraintHandler("permission", i, j)
                                  }
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
                          onClick={() => addConstraintHandler("permission", i)}
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
            <Accordion type="single" collapsible className="w-full">
              <AccordionItem
                value="item-1"
                className="bg-warn-500/10 border border-warn-600/20"
              >
                <AccordionTrigger className="text-white/70 flex bg-warn-400/25 uppercase overflow-hidden rounded-md data-[state=open]:rounded-b-none">
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
                    <Plus />
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
                            onClick={() =>
                              removeComponentHandler("obligation", i)
                            }
                          >
                            <Trash className="mb-0.5 text-danger" />
                            Remove obligation
                          </Button>
                        </div>
                        <Select
                          onValueChange={(value: string) =>
                            fieldValueChangeHandler("obligation", i, value)
                          }
                          value={obligation.action}
                        >
                          <SelectTrigger className="w-[240px]">
                            <SelectValue placeholder="Select action" />
                          </SelectTrigger>
                          <SelectContent>
                            {odrlActions.map((odrlAction) => (
                              <SelectItem value={odrlAction}>
                                {odrlAction}
                              </SelectItem>
                            ))}
                          </SelectContent>
                        </Select>
                        <div className="h-6"></div>
                        <p className="mb-2"> Constraints: </p>
                   
                        {obligation.constraint.map((constraint, j) => (
                          <div className="flex flex-col gap-2 mb-[8px]">
                            <div className="constraint-create flex gap-3 items-end">
                              <Select
                                onValueChange={(value: string) =>
                                  operandValueChangeHandler(
                                    "obligation",
                                    i,
                                    j,
                                    "leftOperand",
                                    value
                                  )
                                }
                                value={constraint.leftOperand}
                              >
                                <div className="flex flex-col">
                                  <p className="text-xs text-gray-400 mb-1">
                                    Left Operand:
                                  </p>
                                  <SelectTrigger className="w-[180px]">
                                    <SelectValue placeholder="Select item" />
                                  </SelectTrigger>
                                  <SelectContent>
                                    {leftOperands.map((leftOperand) => (
                                      <SelectItem value={leftOperand}>
                                        {leftOperand}
                                      </SelectItem>
                                    ))}
                                  </SelectContent>
                                </div>
                              </Select>
                              <Select
                                onValueChange={(value: string) =>
                                  operandValueChangeHandler(
                                    "obligation",
                                    i,
                                    j,
                                    "operator",
                                    value
                                  )
                                }
                                value={constraint.operator}
                              >
                                <div className="flex flex-col">
                                  <p className="text-xs text-gray-400 mb-1">
                                    Operator:
                                  </p>
                                  <SelectTrigger className="w-[140px]">
                                    <SelectValue placeholder="Select operator" />
                                  </SelectTrigger>
                                  <SelectContent>
                                    {operators.map((operator) => (
                                      <SelectItem value={operator}>
                                        {operator}
                                      </SelectItem>
                                    ))}
                                  </SelectContent>
                                </div>
                              </Select>
                              <div className="flex flex-col">
                                <p className="text-xs text-gray-400 mb-1">
                                  Right Operand:
                                </p>
                                <Input
                                  placeholder="Type value"
                                  value={constraint.rightOperand}
                                  onChange={(ev) =>
                                    operandValueChangeHandler(
                                      "obligation",
                                      i,
                                      j,
                                      "rightOperand",
                                      ev.currentTarget.value
                                    )
                                  }
                                />
                              </div>
                              <div>
                                   <Button
                                  variant="icon_destructive"
                                  size="icon"
                                  onClick={() =>
                                    removeConstraintHandler("obligation", i, j)
                                  }
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
                          onClick={() => addConstraintHandler("obligation", i)}
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
            <Accordion type="single" collapsible className="w-full">
              <AccordionItem
                value="item-1"
                className="bg-danger-500/10 border border-danger-600/20"
              >
                <AccordionTrigger className="text-white/70 flex bg-danger-500/25 uppercase overflow-hidden rounded-md data-[state=open]:rounded-b-none">
                  <div className="flex items-center w-full">
                    <p className="text-current">prohibition</p>
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
                    <Plus />
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
                            onClick={() =>
                              removeComponentHandler("prohibition", i)
                            }
                          >
                            <Trash className="mb-0.5" />
                            Remove prohibition
                          </Button>
                        </div>
                        <Select
                          onValueChange={(value: string) =>
                            fieldValueChangeHandler("prohibition", i, value)
                          }
                          value={prohibition.action}
                        >
                          <SelectTrigger className="w-[240px]">
                            <SelectValue placeholder="Select action" />
                          </SelectTrigger>
                          <SelectContent>
                            {odrlActions.map((odrlAction) => (
                              <SelectItem value={odrlAction}>
                                {odrlAction}
                              </SelectItem>
                            ))}
                          </SelectContent>
                        </Select>
                        <div className="h-6"></div>
                        <p className="mb-2"> Constraints: </p>
                        {prohibition.constraint.map((constraint, j) => (
                          <div className="flex flex-col gap-2">
                            <div className="constraint-create flex gap-3 items-end">
                              <Select
                                onValueChange={(value: string) =>
                                  operandValueChangeHandler(
                                    "prohibition",
                                    i,
                                    j,
                                    "leftOperand",
                                    value
                                  )
                                }
                                value={constraint.leftOperand}
                              >
                                <div className="flex flex-col">
                                  <p className="text-xs text-gray-400 mb-1">
                                    Left Operand:
                                  </p>
                                  <SelectTrigger className="w-[180px]">
                                    <SelectValue placeholder="Select item" />
                                  </SelectTrigger>
                                  <SelectContent>
                                    {leftOperands.map((leftOperand) => (
                                      <SelectItem value={leftOperand}>
                                        {leftOperand}
                                      </SelectItem>
                                    ))}
                                  </SelectContent>
                                </div>
                              </Select>
                              <Select
                                onValueChange={(value: string) =>
                                  operandValueChangeHandler(
                                    "prohibition",
                                    i,
                                    j,
                                    "operator",
                                    value
                                  )
                                }
                                value={constraint.operator}
                              >
                                <div className="flex flex-col">
                                  <p className="text-xs text-gray-400 mb-1">
                                    Operator:
                                  </p>
                                  <SelectTrigger className="w-[140px]">
                                    <SelectValue placeholder="Select operator" />
                                  </SelectTrigger>
                                  <SelectContent>
                                    {operators.map((operator) => (
                                      <SelectItem value={operator}>
                                        {operator}
                                      </SelectItem>
                                    ))}
                                  </SelectContent>
                                </div>
                              </Select>
                              <div className="flex flex-col">
                                <p className="text-xs text-gray-400 mb-1">
                                  Right Operand:
                                </p>
                                <Input
                                  placeholder="Type value"
                                  value={constraint.rightOperand}
                                  onChange={(ev) =>
                                    operandValueChangeHandler(
                                      "prohibition",
                                      i,
                                      j,
                                      "rightOperand",
                                      ev.currentTarget.value
                                    )
                                  }
                                />
                              </div>
                              <div>
                                <Button
                                  variant="icon_destructive"
                                  size="icon"
                   
                                  onClick={() =>
                                    removeConstraintHandler("prohibition", i, j)
                                  }
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
                          onClick={() => addConstraintHandler("prohibition", i)}
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
          </div>
        </div>
      </List>
    </div>
  );
};
