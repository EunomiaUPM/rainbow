/**
 * TransferProcessStartDialog.tsx
 *
 * Dialog for starting a transfer process.
 * Available to both provider and consumer roles.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { usePostTransferRPCStart } from "../../data/transfer-mutations";
import { BaseProcessDialog, mapTransferProcessToInfoItems } from "./base";

export const TransferProcessStartDialog = ({ process }: { process: TransferProcess }) => {
  const { api_gateway, dsrole } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: startAsync } = usePostTransferRPCStart();

  /**
   * Handles the start submission.
   * Payload structure differs based on the user's role.
   */
  const handleSubmit = async () => {
    const p = process as any;

    if (dsrole === "provider") {
      await startAsync({
        api_gateway,
        content: {
          consumerParticipantId: p.associated_consumer,
          consumerCallbackAddress: p.data_plane_id,
          consumerPid: p.consumer_pid,
          providerPid: p.provider_pid,
        } as any,
      });
    } else if (dsrole === "consumer") {
      await startAsync({
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
      title="Transfer Start Dialog"
      description="You are about to start the transfer process."
      infoItems={mapTransferProcessToInfoItems(process, dsrole as "provider" | "consumer")}
      submitLabel="Start"
      submitVariant="default"
      onSubmit={handleSubmit}
    />
  );
};
