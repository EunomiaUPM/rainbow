/**
 * ContractNegotiationRequestDialog.tsx
 *
 * Dialog for making a contract negotiation request/counter-request.
 * Allows consumer to modify policy terms via PolicyWrapperEdit.
 *
 * @example
 * <ContractNegotiationRequestDialog process={cnProcess} />
 */

import React, { useContext, useState } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "shared/src/context/GlobalInfoContext";
import { usePostContractNegotiationRPCRequest } from "shared/src/data/contract-mutations";
import { useGetLastContractNegotiationOfferByCNMessageId } from "shared/src/data/contract-queries";
import { PolicyWrapperEdit } from "../PolicyWrapperEdit";
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
  // ---------------------------------------------------------------------------
  // State & Hooks
  // ---------------------------------------------------------------------------

  /** Current edited policy state (declarative pattern) */
  const [currentPolicy, setCurrentPolicy] = useState<OdrlInfo | null>(null);

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
    if (currentPolicy && lastOffer) {
      const outputOffer = {
        ...lastOffer.offer_content,
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

      await dataRequestAsync({
        api_gateway,
        content: {
          providerParticipantId: process.associated_provider!,
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
