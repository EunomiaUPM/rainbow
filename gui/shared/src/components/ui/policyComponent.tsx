import React from 'react';

import { Policy, 
    policyVariants,
    PolicyItemContainer, 
    PolicyItem,
    PolicyItemKey,
    PolicyItemValue, 
    PolicyConstraint,
    PolicyConstraintsContainer,
    PolicyConstraintsWrapper  } from './policy';
import Heading from './heading';

const PolicyComponent = ({policyItem, variant}) => {
    return (
              <Policy className="" variant={variant} >
               
                  {/* COMPROBACIÓN SI HAY ALGUNA PROHIBICIÓN (ARRAY VACIO O NO) */}
                  {(policyItem?.length === 0 || policyItem == null ) ? (
                    <div> No {variant}s </div>
                  ) : (
                         <div className="flex flex-col">
                      {policyItem?.map((prohib) => (
                        <PolicyItemContainer>
                          {/* // <div> {JSON.stringify(policy.prohibition)}</div> */}
                          <PolicyItem>
                            <PolicyItemKey>action:</PolicyItemKey>
                            <PolicyItemValue>{prohib.action}</PolicyItemValue>
                          </PolicyItem>
                          <PolicyItem>
                            <PolicyItemKey>constraints:</PolicyItemKey>
                            <PolicyConstraintsWrapper>
                              {/* comprobar que el constraint no sea null o un array vacio. 
                                Si no lo es, pintar los rightoperand, leftoperand, operator */}
                              {prohib.constraint == null ||
                              prohib.constraint.length === 0 ? (
                                "No constraints"
                              ) : (
                                <>
                                  {/* {console.log(prohib.constraint, " prohib constr")} */}
                                  {prohib.constraint.map((constr) => (
                                    <PolicyConstraintsContainer>
                                      <PolicyConstraint type="rightOperand">
                                        {/* {console.log(constr, "constrrrr")} */}
                                        {JSON.stringify(constr.rightOperand)}
                                      </PolicyConstraint>
                                      <PolicyConstraint type="operator">
                                        {" "}
                                        {JSON.stringify(constr.operator)}
                                      </PolicyConstraint>
                                      <PolicyConstraint type="leftOperand">
                                        {JSON.stringify(constr.leftOperand)}
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
    )
}

export default PolicyComponent;
