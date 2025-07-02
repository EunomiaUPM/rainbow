import React from 'react';

import {
    Policy,
    PolicyConstraint,
    PolicyConstraintsContainer,
    PolicyConstraintsWrapper,
    PolicyItem,
    PolicyItemContainer,
    PolicyItemKey,
    PolicyItemValue
} from 'shared/src/components/ui/policy';

const PolicyComponent = ({policyItem, variant}) => {
    return (
        <Policy className="" variant={variant}>

            {/* COMPROBACIÓN SI HAY ALGUNA PROHIBICIÓN (ARRAY VACIO O NO) */}
            {(policyItem?.length === 0 || policyItem == null) ? (
                <p className='text-xs mt-0.5 text-white/70'> No {variant}s </p>
            ) : (
                <div className="flex flex-col">
                    {policyItem?.map((item) => (
                        <PolicyItemContainer>
                            {/* // <div> {JSON.stringify(policy.prohibition)}</div> */}
                            <PolicyItem>
                                <PolicyItemKey>action:</PolicyItemKey>
                                <PolicyItemValue>{item.action}</PolicyItemValue>
                            </PolicyItem>
                            <PolicyItem>
                                <PolicyItemKey>constraints:</PolicyItemKey>
                                <PolicyConstraintsWrapper>
                                    {/* comprobar que el constraint no sea null o un array vacio.
                                Si no lo es, pintar los rightoperand, leftoperand, operator */}
                                    {item.constraint == null ||
                                    item.constraint.length === 0 ? (
                                        <p className="text-xs mt-0.5 ">No constraints </p>
                                    ) : (
                                        <>
                                            {/* {console.log(prohib.constraint, " prohib constr")} */}
                                            {item.constraint.map((constr) => (
                                                <PolicyConstraintsContainer>
                                                     <PolicyConstraint type="leftOperand">
                                                        {JSON.stringify(constr.leftOperand)}
                                                    </PolicyConstraint>
                                                    <PolicyConstraint type="operator">
                                                        {" "}
                                                        {JSON.stringify(constr.operator)}
                                                    </PolicyConstraint>
                                                  
                                                     <PolicyConstraint type="rightOperand">
                                                        {/* {console.log(constr, "constrrrr")} */}
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
    )
}

export default PolicyComponent;
