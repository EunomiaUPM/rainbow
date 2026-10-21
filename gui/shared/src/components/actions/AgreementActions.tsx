import React, { FC } from "react";
import { ProcessActionDialog } from "./ProcessActionDialog";
import { AgreementDto } from "../../data/orval/model";
import { TransferProcessRequestDialog } from "../dialogs/TransferProcessRequestDialog";
import { useParticipantDirectory } from "../../data/useParticipantDirectory";

/** Actions on an agreement: its consumer can open transfers while it is active. */
export const AgreementActions: FC<{ agreement: AgreementDto; tiny?: boolean }> = ({
  agreement,
  tiny = false,
}) => {
  const { isSelf } = useParticipantDirectory();
  const canTransfer = agreement.state === "ACTIVE" && isSelf(agreement.consumerParticipantId);
  if (!canTransfer) return null;

  return (
    <ProcessActionDialog
      label="Request transfer"
      variant="default"
      tiny={tiny}
      DialogComponent={TransferProcessRequestDialog}
      process={agreement}
    />
  );
};
