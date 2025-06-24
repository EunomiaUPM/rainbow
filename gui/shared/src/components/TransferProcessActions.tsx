import React, { useContext } from "react";
import { cva } from "class-variance-authority";
import {
  GlobalInfoContext,
  GlobalInfoContextType,
} from "shared/src/context/GlobalInfoContext";
import { Dialog, DialogTrigger } from "shared/src/components/ui/dialog";
import { Button } from "shared/src/components/ui/button";
import { TransferProcessStartDialog } from "shared/src/components/TransferProcessStartDialog";
import { TransferProcessTerminationDialog } from "shared/src/components/TransferProcessTerminationDialog";

export const TransferProcessActions = ({
  process,
  tiny = false,
}: {
  process: TransferProcess;
  tiny: boolean;
}) => {
  const { role } = useContext<GlobalInfoContextType>(GlobalInfoContext)!;
  const h2ClassName = cva("font-semibold mb-4", {
    variants: {
      tiny: {
        true: "hidden",
        false: null,
      },
    },
  });
  const containerClassName = cva("", {
    variants: {
      tiny: {
        true: "inline-flex items-center ",
        false:
          "w-full p-6 pt-4 fixed bottom-0 left-0 md:left-[223px] bg-primary-950/10 border border-t-stroke [&>*>button]:min-w-20",
      },
    },
  });

  return (
    <div className={containerClassName({ tiny })}>
      <h2 className={h2ClassName({ tiny })}>Actions</h2>
      {process.state === "REQUESTED" && (
        <div className="space-x-2">
          {role === "provider" && (
            <>
              <Dialog>
                <DialogTrigger asChild>
                  <Button variant="destructive" size={tiny ? "sm" : ""}>Terminate</Button>
                </DialogTrigger>
                <TransferProcessTerminationDialog process={process} />
              </Dialog>
              <Dialog>
                <DialogTrigger asChild>
                  <Button size={tiny ? "sm" : ""}>Start</Button>
                </DialogTrigger>
                <TransferProcessStartDialog process={process} />
              </Dialog>
            </>
          )}
          {role === "consumer" && <></>}
        </div>
      )}
      {process.state === "STARTED" && (
        <div className="space-x-2">
          {role === "provider" && <></>}
          {role === "consumer" && <></>}
        </div>
      )}
      {process.state === "SUSPENDED" && (
        <div className="space-x-2">
          {role === "provider" && <></>}
          {role === "consumer" && <></>}
        </div>
      )}
      {process.state === "COMPLETED" && (
        <div className="space-x-2">
          {role === "provider" && <></>}
          {role === "consumer" && <></>}
        </div>
      )}
      {process.state === "TERMINATED" && (
        <div className="space-x-2">
          {role === "provider" && <></>}
          {role === "consumer" && (
            <>
              <div>No further actions</div>
            </>
          )}
        </div>
      )}
    </div>
  );
};
