/**
 * PolicyWrapperShow.tsx
 *
 * Read-only display component for ODRL policy details.
 * Shows policy metadata, target information, and ODRL rules
 * (permissions, prohibitions, obligations).
 *
 * Features:
 * - Policy ID badge with formatted URN
 * - Policy target and entity information
 * - ODRL content visualization for all rule types
 * - Context-aware actions (delete for providers, request access for consumers)
 *
 * @example
 * <PolicyWrapperShow
 *   policy={policyData}
 *   datasetId="dataset-123"
 *   catalogId="catalog-456"
 *   participant={currentParticipant}
 *   datasetName="My Dataset"
 * />
 */

import React from "react";
import { formatUrn } from "shared/src/lib/utils";
import { InfoList } from "shared/src/components/ui/info-list";
import Heading from "shared/src/components/ui/heading";
import { Badge } from "shared/src/components/ui/badge";
import PolicyComponent from "shared/src/components/PolicyComponent";
import { Trash } from "lucide-react";
import { Button } from "./ui/button";
import { useRouterState } from "@tanstack/react-router";
import { BusinessRemovePolicyDialog } from "./dialogs/BusinessRemovePolicyDialog";
import { Dialog, DialogTrigger } from "shared/src/components/ui/dialog";
import { BusinessRequestAccessDialog } from "./dialogs/BusinessRequestAccessDialog";

// =============================================================================
// TYPES
// =============================================================================

/**
 * Props for the PolicyWrapperShow component.
 */
export interface PolicyWrapperShowProps {
  /** The ODRL policy offer to display */
  policy: OdrlOffer;

  /** ID of the parent dataset (for actions) */
  datasetId: string | undefined;

  /** ID of the parent catalog (for actions) */
  catalogId: string | undefined;

  /** Current participant information (for role-based actions) */
  participant: { participant_type: "Provider" | "Consumer" } | undefined;

  /** Name of the dataset (for display in dialogs) */
  datasetName: string;
}

// =============================================================================
// COMPONENT
// =============================================================================

/**
 * Displays policy details in a read-only card format.
 *
 * The component adapts its behavior based on:
 * - Current route (shows different actions on datahub-catalog pages)
 * - Participant type (providers see delete, consumers see request access)
 *
 * @param props - PolicyWrapperShow properties
 * @returns A styled policy display card
 */
export const PolicyWrapperShow = ({
  policy,
  datasetId,
  catalogId,
  participant,
  datasetName,
}: PolicyWrapperShowProps) => {
  const routerState = useRouterState();

  // ---------------------------------------------------------------------------
  // Computed Values
  // ---------------------------------------------------------------------------

  /** Whether we're viewing a dataset in the datahub catalog */
  const isDatahubDatasetView =
    routerState.location.pathname.includes("datahub-catalog") &&
    routerState.location.pathname.includes("dataset");

  /** Whether current user is a provider */
  const isProvider = participant?.participant_type === "Provider";

  /** Whether current user is a consumer */
  const isConsumer = participant?.participant_type === "Consumer";

  // ---------------------------------------------------------------------------
  // Render
  // ---------------------------------------------------------------------------

  return (
    <div className="w-full">
      <div className="flex flex-col items-start justify-start border border-white/10 bg-white/5 p-3 rounded-md">
        {/* Header: Policy ID and actions */}
        <div className="flex justify-between items-center w-full mb-3">
          <div className="flex items-center gap-2">
            <span className="text-xs font-medium text-muted-foreground uppercase tracking-wider">
              Policy ID
            </span>
            <Badge variant="info" className="font-mono text-[10px]">
              {formatUrn(policy.id)}
            </Badge>
          </div>

          {/* Provider action: Delete policy */}
          {isDatahubDatasetView && isProvider && (
            <Dialog>
              <DialogTrigger asChild>
                <Button
                  variant="ghost"
                  size="icon"
                  className="h-6 w-6 text-muted-foreground hover:text-destructive transition-colors"
                >
                  <Trash className="h-4 w-4" />
                </Button>
              </DialogTrigger>
              <BusinessRemovePolicyDialog
                policy={policy}
                catalogId={catalogId}
                datasetId={datasetId}
              />
            </Dialog>
          )}
        </div>

        {/* Policy metadata */}
        <InfoList
          className="w-full mb-3"
          items={[
            { label: "Policy Target", value: policy.entityType },
            { label: "Target", value: policy.entity.slice(9) },
          ]}
        />

        {/* ODRL Content section */}
        <div className="w-full space-y-2">
          <Heading level="h6" className="text-muted-foreground/70 mb-1">
            ODRL Content
          </Heading>
          <div className="flex flex-col gap-2 w-full">
            <PolicyComponent policyItem={policy.odrlOffer.permission} variant="permission" />
            <PolicyComponent policyItem={policy.odrlOffer.prohibition} variant="prohibition" />
            <PolicyComponent policyItem={policy.odrlOffer.obligation} variant="obligation" />
          </div>
        </div>

        {/* Consumer action: Request access */}
        {isConsumer && (
          <div className="mt-4 w-full flex justify-end">
            <Dialog>
              <DialogTrigger asChild>
                <Button size="sm" variant="default" className="w-full sm:w-auto">
                  Request Access
                </Button>
              </DialogTrigger>
              <BusinessRequestAccessDialog
                policy={policy}
                catalogId={catalogId}
                datasetId={datasetId}
                datasetName={datasetName}
              />
            </Dialog>
          </div>
        )}
      </div>
    </div>
  );
};
