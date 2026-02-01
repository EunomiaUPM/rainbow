/**
 * TransferProcessTerminationDialog.tsx
 *
 * Dialog for terminating a transfer process.
 * Available to both provider and consumer roles.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { usePostTransferRPCTermination } from "../../data/transfer-mutations";
import { BaseProcessDialog, mapTransferProcessToInfoItems } from "./base";

export const TransferProcessTerminationDialog = ({ process }: { process: TransferProcess }) => {
  const { api_gateway, dsrole } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: terminateAsync } = usePostTransferRPCTermination();

  /**
   * Handles the termination submission.
   * Payload structure differs based on the user's role.
   */
  const handleSubmit = async () => {
    const p = process as any;

    if (dsrole === "provider") {
      await terminateAsync({
        api_gateway,
        content: {
          consumerParticipantId: p.associated_consumer,
          consumerCallbackAddress: p.data_plane_id,
          consumerPid: p.consumer_pid,
          providerPid: p.provider_pid,
        },
      });
    } else if (dsrole === "consumer") {
      await terminateAsync({
        api_gateway,
        content: {
          providerParticipantId: p.associated_provider,
          consumerPid: p.consumer_pid,
          providerPid: p.provider_pid,
        },
      });
    }
  };

  return (
    <BaseProcessDialog
      title="Transfer Termination Dialog"
      description="You are about to terminate the transfer process."
      infoItems={mapTransferProcessToInfoItems(process, dsrole as "provider" | "consumer")}
      submitLabel="Terminate"
      submitVariant="destructive"
      onSubmit={handleSubmit}
    />
  );
};
