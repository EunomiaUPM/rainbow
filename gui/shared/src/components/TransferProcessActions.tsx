import React, { FC, useContext } from "react";
import { cva } from "class-variance-authority";
import { GlobalInfoContext, GlobalInfoContextType } from "shared/src/context/GlobalInfoContext";
import { TransferProcessStartDialog } from "shared/src/components/TransferProcessStartDialog";
import { TransferProcessTerminationDialog } from "shared/src/components/TransferProcessTerminationDialog";
import NoFurtherActions from "./ui/noFurtherActions";
import { TransferProcessSuspensionDialog } from "shared/src/components/TransferProcessSuspensionDialog";
import { TransferProcessCompletionDialog } from "shared/src/components/TransferProcessCompletionDialog";
import { ProcessActionDialog } from "./common/ProcessActionDialog";

export const TransferProcessActions: FC<{
  process: TransferProcess;
  tiny: boolean;
}> = ({
  process,
  tiny = false,
}) => {
  const { dsrole } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;

  const containerClassName = cva("", {
    variants: {
      tiny: {
        true: "inline-flex items-center ",
        false:
          "bg-background  w-full p-6 pt-4 fixed bottom-0 left-0 md:left-[223px] bg-primary-950/10 border border-t-stroke [&>*>button]:min-w-20",
      },
    },
  });

  const getActions = () => {
    if (dsrole === "provider") {
      switch (process.state) {
        case "REQUESTED":
          return [
            { label: "Terminate", variant: "destructive", Component: TransferProcessTerminationDialog },
            { label: "Start", variant: "default", Component: TransferProcessStartDialog },
          ];
        case "STARTED":
          return [
            { label: "Terminate", variant: "destructive", Component: TransferProcessTerminationDialog },
            { label: "Suspend", variant: "outline", Component: TransferProcessSuspensionDialog },
            { label: "Complete", variant: "outline", Component: TransferProcessCompletionDialog },
          ];
        case "SUSPENDED":
          const actions = [
            { label: "Terminate", variant: "destructive", Component: TransferProcessTerminationDialog },
          ];
          if (process.stateAttribute && process.stateAttribute !== "BY_CONSUMER") {
            actions.push({ label: "Start", variant: "default", Component: TransferProcessStartDialog });
          }
          actions.push({ label: "Complete", variant: "outline", Component: TransferProcessCompletionDialog });
          return actions;
        case "COMPLETED":
        case "TERMINATED":
             return [];
        default:
          return [];
      }
    } else if (dsrole === "consumer") {
      switch (process.state) {
        case "REQUESTED":
          return [
            { label: "Terminate", variant: "destructive", Component: TransferProcessTerminationDialog },
          ];
        case "STARTED":
          return [
            { label: "Terminate", variant: "destructive", Component: TransferProcessTerminationDialog },
            { label: "Suspend", variant: "outline", Component: TransferProcessSuspensionDialog },
            { label: "Complete", variant: "outline", Component: TransferProcessCompletionDialog },
          ];
        case "SUSPENDED":
          const actions = [
             { label: "Terminate", variant: "destructive", Component: TransferProcessTerminationDialog },
          ];
           if (process.stateAttribute && process.stateAttribute !== "BY_PROVIDER") {
             actions.push({ label: "Start", variant: "default", Component: TransferProcessStartDialog });
           }
           actions.push({ label: "Complete", variant: "outline", Component: TransferProcessCompletionDialog });
           return actions;
        case "COMPLETED":
        case "TERMINATED":
            return [];
        default:
          return [];
      }
    }
    return [];
  };

  const actions = getActions();
  
  const showNoFurtherActions = () => {
      if (process.state === "COMPLETED" || process.state === "TERMINATED") return true;
      return false;
  }

  return (
    <div className={containerClassName({ tiny })}>
       <div className="space-x-2">
      {actions.map((action, idx) => (
        <ProcessActionDialog
          key={idx}
          label={action.label}
          variant={action.variant as any}
          tiny={tiny}
          DialogComponent={action.Component}
          process={process}
        />
      ))}
      {showNoFurtherActions() && <NoFurtherActions />}
      </div>
    </div>
  );
};
