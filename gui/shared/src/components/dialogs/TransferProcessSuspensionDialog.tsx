/**
 * TransferProcessSuspensionDialog.tsx
 *
 * Dialog for suspending a transfer process.
 * Available to both provider and consumer roles.
 */

import React, { useContext } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "../../context/GlobalInfoContext";
import { usePostTransferRPCSuspension } from "../../data/transfer-mutations";
import { BaseProcessDialog, mapTransferProcessToInfoItems } from "./base";

export const TransferProcessSuspensionDialog = ({ process }: { process: TransferProcess }) => {
  const { api_gateway, dsrole } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const { mutateAsync: suspendAsync } = usePostTransferRPCSuspension();

  /**
   * Handles the suspension submission.
   * Payload structure differs based on the user's role.
   */
  const handleSubmit = async () => {
    const p = process as any;

    if (dsrole === "provider") {
      await suspendAsync({
        api_gateway,
        content: {
          consumerParticipantId: p.associated_consumer,
          consumerCallbackAddress: p.data_plane_id,
          consumerPid: p.consumer_pid,
          providerPid: p.provider_pid,
        },
      });
    } else if (dsrole === "consumer") {
      await suspendAsync({
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
      title="Transfer Suspension Dialog"
      description="You are about to suspend the transfer process."
      infoItems={mapTransferProcessToInfoItems(process, dsrole as "provider" | "consumer")}
      submitLabel="Suspend"
      submitVariant="outline"
      onSubmit={handleSubmit}
    />
  );
};
