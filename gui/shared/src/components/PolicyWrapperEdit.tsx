/**
 * PolicyWrapperEdit.tsx
 *
 * Wrapper component for editing an existing ODRL policy.
 * Provides a complete UI for modifying permissions, obligations, and prohibitions
 * with their associated constraints.
 *
 * Exposes the current policy state via an imperative handle for parent components
 * to retrieve the edited policy data on form submission.
 *
 * @example
 * const policyRef = useRef<PolicyEditorHandle>(null);
 *
 * // In parent component
 * <PolicyWrapperEdit policy={existingPolicy} ref={policyRef} />
 *
 * // On submit
 * const editedPolicy = policyRef.current?.getPolicy();
 */

import React, { forwardRef, useEffect, useImperativeHandle, useState } from "react";
import Heading from "shared/src/components/ui/heading";
import { Badge } from "shared/src/components/ui/badge";
import { InfoList } from "shared/src/components/ui/info-list";
import { Accordion } from "shared/src/components/ui/accordion";
import {
  ComponentType,
  OperandType,
  PolicyEditSection,
} from "shared/src/components/policy/PolicyEditSection";

// =============================================================================
// TYPES
// =============================================================================

/**
 * Imperative handle exposed to parent components.
 * Allows retrieving the current edited policy state.
 */
export type PolicyEditorHandle = {
  /** Returns the current policy state with all edits applied */
  getPolicy: () => OdrlInfo;
};

/**
 * Props for the PolicyWrapperEdit component.
 */
export interface PolicyWrapperEditProps {
  /** The ODRL policy to edit */
  policy: OdrlOffer;
}

// =============================================================================
// COMPONENT
// =============================================================================

/**
 * Full-featured policy editor with state management.
 *
 * Features:
 * - Displays policy metadata (ID, type, target)
 * - Three collapsible sections for permission/obligation/prohibition
 * - Add/remove policy items and constraints
 * - Exposes edited state via ref handle
 *
 * @param props - PolicyWrapperEdit properties
 * @param ref - Ref to access the policy editor handle
 * @returns A complete policy editing interface
 */
export const PolicyWrapperEdit = forwardRef<PolicyEditorHandle, PolicyWrapperEditProps>(
  ({ policy }, ref) => {
    // -------------------------------------------------------------------------
    // State
    // -------------------------------------------------------------------------

    const [newPolicy, setNewPolicy] = useState<OdrlInfo>({
      obligation: [],
      permission: [],
      prohibition: [],
    });

    // -------------------------------------------------------------------------
    // Imperative Handle
    // -------------------------------------------------------------------------

    useImperativeHandle(ref, () => ({
      getPolicy: () => newPolicy,
    }));

    // -------------------------------------------------------------------------
    // Effects
    // -------------------------------------------------------------------------

    /** Initialize state from incoming policy prop */
    useEffect(() => {
      setNewPolicy({
        obligation: policy.obligation || [],
        permission: policy.permission || [],
        prohibition: policy.prohibition || [],
      });
    }, [policy]);

    // -------------------------------------------------------------------------
    // Event Handlers
    // -------------------------------------------------------------------------

    /** Add a new policy component (permission/obligation/prohibition) */
    const handleAddComponent = (componentType: ComponentType) => {
      const newComponent: OdrlPermission = {
        action: "",
        constraint: [],
      };
      setNewPolicy((prev) => ({
        ...prev,
        [componentType]: [...prev[componentType], newComponent],
      }));
    };

    /** Remove a policy component by index */
    const handleRemoveComponent = (componentType: ComponentType, index: number) => {
      setNewPolicy((prev) => ({
        ...prev,
        [componentType]: prev[componentType].filter((_, i) => i !== index),
      }));
    };

    /** Add a constraint to a policy component */
    const handleAddConstraint = (componentType: ComponentType, componentIndex: number) => {
      setNewPolicy((prev) => {
        const updated = [...prev[componentType]];
        updated[componentIndex] = {
          ...updated[componentIndex],
          constraint: [
            ...updated[componentIndex].constraint,
            { leftOperand: "", operator: "", rightOperand: "" },
          ],
        };
        return { ...prev, [componentType]: updated };
      });
    };

    /** Remove a constraint from a policy component */
    const handleRemoveConstraint = (
      componentType: ComponentType,
      componentIndex: number,
      constraintIndex: number
    ) => {
      setNewPolicy((prev) => {
        const updated = [...prev[componentType]];
        updated[componentIndex] = {
          ...updated[componentIndex],
          constraint: updated[componentIndex].constraint.filter(
            (_, i) => i !== constraintIndex
          ),
        };
        return { ...prev, [componentType]: updated };
      });
    };

    /** Update a policy component's action */
    const handleActionChange = (
      componentType: ComponentType,
      componentIndex: number,
      value: string
    ) => {
      setNewPolicy((prev) => {
        const updated = [...prev[componentType]];
        updated[componentIndex] = { ...updated[componentIndex], action: value };
        return { ...prev, [componentType]: updated };
      });
    };

    /** Update a constraint's operand value */
    const handleOperandChange = (
      componentType: ComponentType,
      componentIndex: number,
      constraintIndex: number,
      operand: OperandType,
      value: string
    ) => {
      setNewPolicy((prev) => {
        const updated = [...prev[componentType]];
        const constraints = [...updated[componentIndex].constraint];
        constraints[constraintIndex] = {
          ...constraints[constraintIndex],
          [operand]: value,
        };
        updated[componentIndex] = { ...updated[componentIndex], constraint: constraints };
        return { ...prev, [componentType]: updated };
      });
    };

    // -------------------------------------------------------------------------
    // Render
    // -------------------------------------------------------------------------

    return (
      <div className="w-full">
        <div className="border border-white/30 bg-white/10 p-3 pt-2 rounded-md justify-start">
          {/* Policy Header */}
          <div className="flex mb-4">
            <Heading level="h5" className="flex gap-3">
              <div>Policy with ID</div>
              <Badge variant="info" className="h-6">
                {policy["@id"].slice(9, 29) + "[...]"}
              </Badge>
            </Heading>
          </div>

          {/* Policy Metadata */}
          <InfoList
            items={[
              { label: "Policy Target", value: policy["@type"] },
              { label: "Profile", value: JSON.stringify(policy.profile) },
              { label: "Target", value: policy.target.slice(9) },
            ]}
          />

          {/* ODRL Content Editor */}
          <div className="h-5" />
          <Heading level="h6">ODRL CONTENT</Heading>

          <div className="flex flex-col gap-2">
            <div className="flex flex-col gap-4">
              {/* Permission Section */}
              <PolicyEditSection
                type="permission"
                items={newPolicy.permission}
                onAdd={handleAddComponent}
                onRemove={handleRemoveComponent}
                onActionChange={handleActionChange}
                onAddConstraint={handleAddConstraint}
                onRemoveConstraint={handleRemoveConstraint}
                onOperandChange={handleOperandChange}
              />

              {/* Obligation Section */}
              <PolicyEditSection
                type="obligation"
                items={newPolicy.obligation}
                onAdd={handleAddComponent}
                onRemove={handleRemoveComponent}
                onActionChange={handleActionChange}
                onAddConstraint={handleAddConstraint}
                onRemoveConstraint={handleRemoveConstraint}
                onOperandChange={handleOperandChange}
              />

              {/* Prohibition Section */}
              <PolicyEditSection
                type="prohibition"
                items={newPolicy.prohibition}
                onAdd={handleAddComponent}
                onRemove={handleRemoveComponent}
                onActionChange={handleActionChange}
                onAddConstraint={handleAddConstraint}
                onRemoveConstraint={handleRemoveConstraint}
                onOperandChange={handleOperandChange}
              />
            </div>
          </div>
        </div>
      </div>
    );
  }
);

PolicyWrapperEdit.displayName = "PolicyWrapperEdit";
