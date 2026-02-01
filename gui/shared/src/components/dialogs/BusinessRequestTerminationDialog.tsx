/**
 * BusinessRequestTerminationDialog.tsx
 *
 * Dialog for terminating a business request.
 * Available to both provider and consumer roles.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { usePostBusinessTerminationRequest } from "../../data/business-mutations";
import { BaseProcessDialog, mapCNProcessToFullInfoItems } from "./base";

export const BusinessRequestTerminationDialog = ({ process }: { process: CNProcess }) => {
  const { api_gateway, dsrole } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: terminateAsync } = usePostBusinessTerminationRequest();

  /**
   * Handles the termination submission.
   * Payload structure differs based on the user's role.
   */
  const handleSubmit = async () => {
    if (dsrole === "consumer") {
      await terminateAsync({
        api_gateway,
        content: {
          providerParticipantId: process.associated_provider,
          consumerPid: process.consumer_id,
          providerPid: process.provider_id,
        },
      });
    } else if (dsrole === "provider") {
      await terminateAsync({
        api_gateway,
        content: {
          consumerParticipantId: process.associated_consumer,
          consumerPid: process.consumer_id,
          providerPid: process.provider_id,
        },
      });
    }
  };

  return (
    <BaseProcessDialog
      title="Request Termination Dialog"
      description={
        <>
          You are about to terminate the business request.
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
