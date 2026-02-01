/**
 * ContractNegotiationOfferDialog.tsx
 *
 * Dialog for making a counter-offer in a contract negotiation.
 * Allows provider to modify policy terms via PolicyWrapperEdit.
 *
 * @example
 * <ContractNegotiationOfferDialog process={cnProcess} />
 */

import React, { useContext, useRef } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "shared/src/context/GlobalInfoContext";
import { usePostContractNegotiationRPCOffer } from "shared/src/data/contract-mutations";
import { useGetLastContractNegotiationOfferByCNMessageId } from "shared/src/data/contract-queries";
import { PolicyEditorHandle, PolicyWrapperEdit } from "../PolicyWrapperEdit";
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
  const policyWrapperRef = useRef<PolicyEditorHandle>(null);
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
    if (policyWrapperRef.current) {
      const policy = policyWrapperRef.current.getPolicy();
      const outputOffer = {
        ...lastOffer?.offer_content,
        prohibition:
          policy.prohibition && policy.prohibition.length > 0 ? policy.prohibition : null,
        permission:
          policy.permission && policy.permission.length > 0 ? policy.permission : null,
        obligation:
          policy.obligation && policy.obligation.length > 0 ? policy.obligation : null,
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
      <PolicyWrapperEdit policy={lastOffer.offer_content} ref={policyWrapperRef} />
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
