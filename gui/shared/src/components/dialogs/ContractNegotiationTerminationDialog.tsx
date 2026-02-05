/**
 * ContractNegotiationTerminationDialog.tsx
 *
 * Dialog for terminating a contract negotiation process.
 * Uses the base dialog component for consistent structure.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { usePostContractNegotiationRPCTermination } from "../../data/contract-mutations";
import { BaseProcessDialog, mapCNProcessToFullInfoItems } from "./base";

export const ContractNegotiationTerminationDialog = ({
  process,
}: {
  process: NegotiationProcessDto;
}) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: terminateAsync } = usePostContractNegotiationRPCTermination();

  /**
   * Handles the termination submission.
   * The payload differs based on whether the user is a provider or consumer.
   */
  const handleSubmit = async () => {
    if (process.role === "Consumer") {
      await terminateAsync({
        api_gateway,
        content: {
          providerParticipantId: process.associatedAgentPeer!,
          consumerPid: process.identifiers.consumer_id,
          providerPid: process.identifiers.provider_id,
        },
      });
    } else if (process.role === "Provider") {
      await terminateAsync({
        api_gateway,
        content: {
          consumerParticipantId: process.associatedAgentPeer!,
          consumerPid: process.identifiers.consumer_id,
          providerPid: process.identifiers.provider_id,
        },
      });
    }
  };

  return (
    <BaseProcessDialog
      title="Termination Dialog"
      description={
        <>
          You are about to terminate the contract negotiation.
          <br />
          Please review the details carefully before proceeding.
        </>
      }
      infoItems={mapCNProcessToFullInfoItems(process)}
      submitLabel="Terminate"
      submitVariant="destructive"
      onSubmit={handleSubmit}
    />
  );
};
