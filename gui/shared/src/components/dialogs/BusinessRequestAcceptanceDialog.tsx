/**
 * BusinessRequestAcceptanceDialog.tsx
 *
 * Dialog for accepting a business request.
 * Provider-side action to approve access requests.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { usePostBusinessAcceptationRequest } from "../../data/business-mutations";
import { BaseProcessDialog, mapCNProcessToFullInfoItems } from "./base";

export const BusinessRequestAcceptanceDialog = ({ process }: { process: CNProcess }) => {
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: acceptAsync } = usePostBusinessAcceptationRequest();

  /**
   * Handles the acceptance submission.
   */
  const handleSubmit = async () => {
    await acceptAsync({
      api_gateway,
      content: {
        consumerParticipantId: process.associated_consumer,
        consumerPid: process.consumer_id,
        providerPid: process.provider_id,
      },
    });
  };

  return (
    <BaseProcessDialog
      title="Request Acceptance Dialog"
      description={
        <>
          You are about to accept a request of contract negotiation.
          <br />
          Please review the details carefully before proceeding.
        </>
      }
      infoItems={mapCNProcessToFullInfoItems(process)}
      submitLabel="Approve"
      submitVariant="default"
      onSubmit={handleSubmit}
    />
  );
};
