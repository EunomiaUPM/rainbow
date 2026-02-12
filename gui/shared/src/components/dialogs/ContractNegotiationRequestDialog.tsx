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
import { PolicyWrapperEdit } from "../PolicyWrapperEdit";
import { BaseProcessDialog } from "./base";
import { mapCNProcessToInfoItemsForConsumer } from "./base/infoItemMappers";
import Heading from "../ui/heading";
import { OdrlInfo, NegotiationProcessDto } from "../../data/orval/model";
import { useRpcSetupRequestInit } from "../../data/orval/negotiation-rp-c/negotiation-rp-c";

// =============================================================================
// TYPES
// =============================================================================

/**
 * Props for the ContractNegotiationRequestDialog component.
 */
export interface ContractNegotiationRequestDialogProps {
  /** The contract negotiation process */
  process: NegotiationProcessDto;
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

  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: dataRequestAsync } = useRpcSetupRequestInit();

  // ---------------------------------------------------------------------------
  // Info Items
  // ---------------------------------------------------------------------------

  const infoItems = mapCNProcessToInfoItemsForConsumer(process);

  // ---------------------------------------------------------------------------
  // Submit Handler
  // ---------------------------------------------------------------------------

  const handleSubmit = async () => {
    await dataRequestAsync({
      data: {
        associatedAgentPeer: process.associatedAgentPeer,
        providerAddress: "process.providerAddress", // TODO: get from process
        callbackAddress: "process.callbackAddress", // TODO: get from process
        offer: currentPolicy,
      },
    });

  };

  // ---------------------------------------------------------------------------
  // After Info Content (Policy Editor)
  // ---------------------------------------------------------------------------

  // const policyEditorContent = lastOffer ? (
  //   <div className="pt-4">
  //     <Heading level="h6" className="mb-2">
  //       New Policy Request
  //     </Heading>
  //     <PolicyWrapperEdit policy={lastOffer.offer_content} onChange={setCurrentPolicy} />
  //   </div>
  // ) : null;

  // ---------------------------------------------------------------------------
  // Render
  // ---------------------------------------------------------------------------

  return (
    <BaseProcessDialog
      title="Contract Negotiation Request"
      description="Make changes to the Contract Negotiation Request and submit a counter-request."
      infoItems={infoItems}
      afterInfoContent={null}
      submitLabel="Request"
      submitVariant="outline"
      onSubmit={handleSubmit}
      scrollable
    />
  );
};
