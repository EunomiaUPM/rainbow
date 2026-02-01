/**
 * ContractNegotiationRequestDialog.tsx
 *
 * Dialog for making a contract negotiation request/counter-request.
 * Allows consumer to modify policy terms via PolicyWrapperEdit.
 *
 * @example
 * <ContractNegotiationRequestDialog process={cnProcess} />
 */

import React, { useContext, useRef } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "shared/src/context/GlobalInfoContext";
import { usePostContractNegotiationRPCRequest } from "shared/src/data/contract-mutations";
import { useGetLastContractNegotiationOfferByCNMessageId } from "shared/src/data/contract-queries";
import { PolicyEditorHandle, PolicyWrapperEdit } from "../PolicyWrapperEdit";
import { BaseProcessDialog } from "./base";
import { mapCNProcessToInfoItemsForConsumer } from "./base/infoItemMappers";
import Heading from "../ui/heading";

// =============================================================================
// TYPES
// =============================================================================

/**
 * Props for the ContractNegotiationRequestDialog component.
 */
export interface ContractNegotiationRequestDialogProps {
  /** The contract negotiation process */
  process: CNProcess;
}

// =============================================================================
// COMPONENT
// =============================================================================

/**
 * Dialog for making a contract negotiation request/counter-request.
 *
 * Features:
 * - Displays current CN process information
 * - Allows policy modification via PolicyWrapperEdit
 * - Submits request/counter-request to provider
 */
export const ContractNegotiationRequestDialog = ({
  process,
}: ContractNegotiationRequestDialogProps) => {
  const policyWrapperRef = useRef<PolicyEditorHandle>(null);
  const { mutateAsync: dataRequestAsync } = usePostContractNegotiationRPCRequest();
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { data: lastOffer } = useGetLastContractNegotiationOfferByCNMessageId(process.consumer_id);

  // ---------------------------------------------------------------------------
  // Info Items
  // ---------------------------------------------------------------------------

  const infoItems = mapCNProcessToInfoItemsForConsumer(process);

  // ---------------------------------------------------------------------------
  // Submit Handler
  // ---------------------------------------------------------------------------

  const handleSubmit = async () => {
    if (policyWrapperRef.current && lastOffer) {
      const policy = policyWrapperRef.current.getPolicy();
      const outputOffer = {
        ...lastOffer.offer_content,
        prohibition:
          policy.prohibition && policy.prohibition.length > 0 ? policy.prohibition : null,
        permission:
          policy.permission && policy.permission.length > 0 ? policy.permission : null,
        obligation:
          policy.obligation && policy.obligation.length > 0 ? policy.obligation : null,
      };

      await dataRequestAsync({
        api_gateway,
        content: {
          providerParticipantId: process.associated_provider,
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
        New Policy Request
      </Heading>
      <PolicyWrapperEdit policy={lastOffer.offer_content} ref={policyWrapperRef} />
    </div>
  ) : null;

  // ---------------------------------------------------------------------------
  // Render
  // ---------------------------------------------------------------------------

  return (
    <BaseProcessDialog
      title="Contract Negotiation Request"
      description="Make changes to the Contract Negotiation Request and submit a counter-request."
      infoItems={infoItems}
      afterInfoContent={policyEditorContent}
      submitLabel="Request"
      submitVariant="outline"
      onSubmit={handleSubmit}
      scrollable
    />
  );
};
