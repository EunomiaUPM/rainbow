/**
 * ContractNegotiationOfferDialog.tsx
 *
 * Dialog for making a counter-offer in a contract negotiation.
 * Allows provider to modify policy terms via PolicyWrapperEdit.
 *
 * @example
 * <ContractNegotiationOfferDialog process={cnProcess} />
 */

import React, { useContext, useState } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "shared/src/context/GlobalInfoContext";
import { useRpcSetupOffer } from "shared/src/data/orval/negotiation-rp-c/negotiation-rp-c";
import { useGetOffersByProcessId } from "shared/src/data/orval/negotiations/negotiations";
import { NegotiationProcessDto } from "shared/src/data/orval/model/negotiationProcessDto";
import { OdrlOffer } from "shared/src/data/orval/model/odrlOffer";
import { OdrlInfo } from "shared/src/data/orval/model/odrlInfo";
import { PolicyWrapperEdit } from "../PolicyWrapperEdit";
import { BaseProcessDialog } from "./base";
import { mapCNProcessToInfoItemsForProvider } from "./base/infoItemMappers";
import Heading from "../ui/heading";

// =============================================================================
// TYPES
// =============================================================================

/**
 * Props for the ContractNegotiationOfferDialog component.
 */
export interface ContractNegotiationOfferDialogProps {
  /** The contract negotiation process */
  process: NegotiationProcessDto;
}

// =============================================================================
// COMPONENT
// =============================================================================

/**
 * Dialog for making a contract negotiation counter-offer.
 *
 * Features:
 * - Displays current CN process information
 * - Allows policy modification via PolicyWrapperEdit
 * - Submits counter-offer to consumer
 */
export const ContractNegotiationOfferDialog = ({
  process,
}: ContractNegotiationOfferDialogProps) => {
  // ---------------------------------------------------------------------------
  // State & Hooks
  // ---------------------------------------------------------------------------

  /** Current edited policy state (declarative pattern) */
  const [currentPolicy, setCurrentPolicy] = useState<OdrlInfo | null>(null); // OdrlInfo matches OdrlOffer structure roughly

  const { mutateAsync: setupOffer } = useRpcSetupOffer();
  const { data: offersData } = useGetOffersByProcessId(process.id);
  const lastOffer = offersData?.status === 200 ? offersData.data.at(-1) : undefined;

  // ---------------------------------------------------------------------------
  // Info Items
  // ---------------------------------------------------------------------------

  const infoItems = mapCNProcessToInfoItemsForProvider(process);

  // ---------------------------------------------------------------------------
  // Submit Handler
  // ---------------------------------------------------------------------------

  const handleSubmit = async () => {
    if (currentPolicy && lastOffer) {
      // Construct the new offer based on the last offer and edits
      const outputOffer: OdrlOffer = {
        ...lastOffer, // Keep other properties from last offer
        // Overwrite policy parts
        prohibition:
          currentPolicy.prohibition && currentPolicy.prohibition.length > 0
            ? currentPolicy.prohibition
            : undefined,
        permission:
          currentPolicy.permission && currentPolicy.permission.length > 0
            ? currentPolicy.permission
            : undefined,
        obligation:
          currentPolicy.obligation && currentPolicy.obligation.length > 0
            ? currentPolicy.obligation
            : undefined,
      };

      await setupOffer({
        data: {
          processId: process.id,
          offer: outputOffer,
        },
      });
    }
  };

  // ---------------------------------------------------------------------------
  // After Info Content (Policy Editor)
  // ---------------------------------------------------------------------------

  const policyEditorContent = lastOffer ? (
    <div className="pt-4">
      <Heading level="h6" className="mb-2">
        Counter Offer Policy
      </Heading>
      {/* Ensure lastOffer satisfies OdrlInfo/OdrlOffer which PolicyWrapperEdit expects */}
      <PolicyWrapperEdit policy={lastOffer as unknown as OdrlInfo} onChange={setCurrentPolicy} />
    </div>
  ) : null;

  // ---------------------------------------------------------------------------
  // Render
  // ---------------------------------------------------------------------------

  return (
    <BaseProcessDialog
      title="Contract Negotiation Offer"
      description="Make changes to the Contract Negotiation Offer and submit a counter-offer."
      infoItems={infoItems}
      afterInfoContent={policyEditorContent}
      submitLabel="Counter Offer"
      submitVariant="outline"
      onSubmit={handleSubmit}
      scrollable
    />
  );
};

