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
import { usePostContractNegotiationRPCOffer } from "shared/src/data/contract-mutations";
import { useGetLastContractNegotiationOfferByCNMessageId } from "shared/src/data/contract-queries";
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
  process: CNProcess;
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
  const [currentPolicy, setCurrentPolicy] = useState<OdrlInfo | null>(null);

  const { mutateAsync: dataOfferAsync } = usePostContractNegotiationRPCOffer();
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data: lastOffer } = useGetLastContractNegotiationOfferByCNMessageId(process.provider_id);

  // ---------------------------------------------------------------------------
  // Info Items
  // ---------------------------------------------------------------------------

  const infoItems = mapCNProcessToInfoItemsForProvider(process);

  // ---------------------------------------------------------------------------
  // Submit Handler
  // ---------------------------------------------------------------------------

  const handleSubmit = async () => {
    if (currentPolicy && lastOffer) {
      const outputOffer = {
        ...lastOffer.offer_content,
        prohibition:
          currentPolicy.prohibition && currentPolicy.prohibition.length > 0
            ? currentPolicy.prohibition
            : null,
        permission:
          currentPolicy.permission && currentPolicy.permission.length > 0
            ? currentPolicy.permission
            : null,
        obligation:
          currentPolicy.obligation && currentPolicy.obligation.length > 0
            ? currentPolicy.obligation
            : null,
      };

      await dataOfferAsync({
        api_gateway,
        content: {
          consumerParticipantId: process.associated_consumer,
          offer: outputOffer,
          consumerPid: process.consumer_id,
          providerPid: process.provider_id,
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
      <PolicyWrapperEdit
        policy={lastOffer.offer_content}
        onChange={setCurrentPolicy}
      />
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
