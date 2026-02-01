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

export const ContractNegotiationTerminationDialog = ({ process }: { process: CNProcess }) => {
  const { api_gateway, dsrole } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: terminateAsync } = usePostContractNegotiationRPCTermination();

  /**
   * Handles the termination submission.
   * The payload differs based on whether the user is a provider or consumer.
   */
  const handleSubmit = async () => {
    if (dsrole === "consumer") {
      await terminateAsync({
        api_gateway,
        content: {
          providerParticipantId: process.associated_provider!,
          consumerPid: process.consumer_id,
          providerPid: process.provider_id,
        },
      });
    } else if (dsrole === "provider") {
      await terminateAsync({
        api_gateway,
        content: {
          consumerParticipantId: process.associated_consumer!,
          consumerPid: process.consumer_id,
          providerPid: process.provider_id,
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
