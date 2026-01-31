import React, { FC } from "react";

import {
  Policy,
  PolicyConstraint,
  PolicyConstraintsContainer,
  PolicyConstraintsWrapper,
  PolicyItem,
  PolicyItemContainer,
  PolicyItemKey,
  PolicyItemValue,
  PolicyVariants,
} from "shared/src/components/ui/policy";

/**
 * Component for displaying policy items recursively.
 */
const PolicyComponent: FC<{
  policyItem: OdrlPermission[] | null,
  variant: PolicyVariants
}> = ({ policyItem, variant }) => {
  return (
    <Policy className="" variant={variant}>
      {policyItem?.length === 0 || policyItem == null ? (
        <p className="text-xs mt-0.5 text-white/70"> No {variant}s </p>
      ) : (
        <div className="flex flex-col">
          {policyItem?.map((item, i: number) => (
            <PolicyItemContainer key={i}>
              <PolicyItem>
                <PolicyItemKey>action:</PolicyItemKey>
                <PolicyItemValue>{item.action}</PolicyItemValue>
              </PolicyItem>
              <PolicyItem>
                <PolicyItemKey>constraints:</PolicyItemKey>
                <PolicyConstraintsWrapper>
                  {item.constraint == null || item.constraint.length === 0 ? (
                    <p className="text-xs mt-0.5 ">No constraints </p>
                  ) : (
                    <>
                      {item.constraint.map((constr, j: number) => (
                        <PolicyConstraintsContainer key={j}>
                          <PolicyConstraint type="leftOperand" className="">
                            {JSON.stringify(constr.leftOperand)}
                          </PolicyConstraint>
                          <PolicyConstraint type="operator" className="">
                            {JSON.stringify(constr.operator)}
                          </PolicyConstraint>
                          <PolicyConstraint type="rightOperand" className="">
                            {JSON.stringify(constr.rightOperand)}
                          </PolicyConstraint>
                        </PolicyConstraintsContainer>
                      ))}
                    </>
                  )}
                </PolicyConstraintsWrapper>
              </PolicyItem>
            </PolicyItemContainer>
          ))}
        </div>
      )}
    </Policy>
  );
};

export default PolicyComponent;
