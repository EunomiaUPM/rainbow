/**
 * ContractNegotiationNewRequestDialog.tsx
 *
 * Dialog for creating a new contract negotiation request.
 * Displays policy information and initiates a new CN process.
 *
 * @example
 * <ContractNegotiationNewRequestDialog
 *   policy={policyData}
 *   catalogId="catalog-123"
 *   datasetId="dataset-456"
 *   participantId="provider-participant-id"
 * />
 */

import React, { useContext, useRef } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "shared/src/context/GlobalInfoContext";
import { PolicyWrapperShow } from "shared/src/components/PolicyWrapperShow";
import { usePostContractNegotiationRPCRequest } from "shared/src/data/contract-mutations";
import { BaseProcessDialog } from "./base";
import { urnInfoItem } from "./base/infoItemMappers";
import { InfoItemProps } from "../ui/info-list";
import { DialogClose } from "../ui/dialog";

// =============================================================================
// TYPES
// =============================================================================

/**
 * Props for the ContractNegotiationNewRequestDialog component.
 */
export interface ContractNegotiationNewRequestDialogProps {
  /** The ODRL policy to negotiate */
  policy: OdrlOffer;

  /** ID of the parent catalog */
  catalogId: UUID;

  /** ID of the dataset */
  datasetId: UUID;

  /** Provider participant ID to negotiate with */
  participantId: string;
}

// =============================================================================
// COMPONENT
// =============================================================================

/**
 * Dialog for initiating a new contract negotiation request.
 *
 * Features:
 * - Displays catalog and dataset information
 * - Shows policy details via PolicyWrapperShow
 * - Initiates CN request with provider
 */
export const ContractNegotiationNewRequestDialog = ({
  policy,
  catalogId,
  datasetId,
  participantId,
}: ContractNegotiationNewRequestDialogProps) => {
  const closeDialogRef = useRef<HTMLButtonElement>(null);
  const { mutateAsync: requestAsync } = usePostContractNegotiationRPCRequest();
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;

  // ---------------------------------------------------------------------------
  // Info Items
  // ---------------------------------------------------------------------------

  const infoItems: InfoItemProps[] = [
    urnInfoItem("Catalog ID", catalogId),
    urnInfoItem("Dataset ID", datasetId),
    urnInfoItem("Provider", participantId),
  ].filter((item): item is InfoItemProps => item !== undefined);

  // ---------------------------------------------------------------------------
  // Submit Handler
  // ---------------------------------------------------------------------------

  const handleSubmit = async () => {
    await requestAsync({
      api_gateway,
      content: {
        providerParticipantId: participantId,
        //@ts-ignore - policy @id format
        offer: {
          "@id": policy["@id"],
        },
      },
    });
    closeDialogRef.current?.click();
  };

  // ---------------------------------------------------------------------------
  // After Info Content (Policy Display)
  // ---------------------------------------------------------------------------

  const policyContent = (
    <div className="pt-4">
      <PolicyWrapperShow
        policy={policy}
        datasetId={datasetId}
        catalogId={catalogId}
        participant={participantId}
        datasetName=""
      />
      {/* Hidden close button for programmatic dialog close */}
      <DialogClose ref={closeDialogRef} className="hidden" />
    </div>
  );

  // ---------------------------------------------------------------------------
  // Render
  // ---------------------------------------------------------------------------

  return (
    <BaseProcessDialog
      title="Contract Negotiation Request"
      description="You are about to request a contract negotiation for the selected dataset and policy."
      infoItems={infoItems}
      afterInfoContent={policyContent}
      submitLabel="Request Contract Negotiation"
      submitVariant="default"
      onSubmit={handleSubmit}
      scrollable
    />
  );
};
