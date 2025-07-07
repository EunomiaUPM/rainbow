import React, { useContext } from "react";
import { AuthContext, AuthContextType } from "shared/src/context/AuthContext";
import { Dialog, DialogTrigger } from "shared/src/components/ui/dialog";
import { Button } from "shared/src/components/ui/button";
import { BusinessRequestTerminationDialog } from "shared/src/components/BusinessRequestTerminationDialog";
import { BusinessRequestAcceptanceDialog } from "@/components/BusinessRequestAcceptanceDialog.tsx";
import NoFurtherActions from "@/components/ui/noFurtherActions.tsx";

const BusinessActionsForProvider = ({ process }: { process: CNProcess }) => {
  return (
    <div className="space-x-2 min-w-[260px]">
      {process.state == "REQUESTED" ? (
        <>
          <Dialog>
            <DialogTrigger asChild>
              <Button variant="default">Accept</Button>
            </DialogTrigger>
            <BusinessRequestAcceptanceDialog process={process} />
          </Dialog>
          <Dialog>
            <DialogTrigger asChild>
              <Button variant="destructive">Reject</Button>
            </DialogTrigger>
            <BusinessRequestTerminationDialog process={process} />
          </Dialog>
        </>
      ) : (
        <NoFurtherActions />
      )}
    </div>
  );
};

const BusinessActionsForConsumer = ({ process }: { process: CNProcess }) => {
  return (
    <div className="space-x-2 min-w-[260px]">
      {process.state == "REQUESTED" ? (
        <>
          <Dialog>
            <DialogTrigger asChild>
              <Button variant="destructive">Cancel</Button>
            </DialogTrigger>
            <BusinessRequestTerminationDialog process={process} />
          </Dialog>
        </>
      ) : (
        <NoFurtherActions />
      )}
    </div>
  );
};

export const BusinessActions = ({ process }: { process: CNProcess }) => {
  const { participant } = useContext<AuthContextType | null>(AuthContext)!;
  return (
    <div className="inline-flex items-center">
      {participant?.participant_type == "Provider" && (
        <BusinessActionsForProvider process={process} />
      )}
      {participant?.participant_type == "Consumer" && (
        <BusinessActionsForConsumer process={process} />
      )}
    </div>
  );
};
