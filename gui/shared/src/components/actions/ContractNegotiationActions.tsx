import { ButtonSizes } from "../ui/button";
import React, { useContext } from "react";
import { cva } from "class-variance-authority";
import { ContractNegotiationOfferDialog } from "../dialogs/ContractNegotiationOfferDialog";
import { ContractNegotiationAgreementDialog } from "../dialogs/ContractNegotiationAgreementDialog";
import { ContractNegotiationTerminationDialog } from "../dialogs/ContractNegotiationTerminationDialog";
import { ContractNegotiationFinalizationDialog } from "../dialogs/ContractNegotiationFinalizationDialog";
import { GlobalInfoContext, GlobalInfoContextType } from "shared/src/context/GlobalInfoContext";
import { ContractNegotiationRequestDialog } from "../dialogs/ContractNegotiationRequestDialog";
import { ContractNegotiationAcceptanceDialog } from "../dialogs/ContractNegotiationAcceptanceDialog";
import { ContractNegotiationVerificationDialog } from "../dialogs/ContractNegotiationVerificationDialog";
import NoFurtherActions from "../ui/noFurtherActions";
import { ProcessActionDialog } from "./ProcessActionDialog";

/**
 * Actions available for a contract negotiation process.
 */
export const ContractNegotiationActions = ({
  process,
  tiny = false,
}: {
  process: CNProcess;
  tiny: boolean;
}) => {
  const { dsrole } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;

  // Define container class name with variants
  const containerClassName = cva("", {
    variants: {
      tiny: {
        true: "inline-flex items-center ",
        false:
          " w-full p-6 fixed bottom-0 left-0 md:left-[223px] bg-background/80 backdrop-blur-sm border border-t-stroke [&>*>button]:min-w-20",
      },
    },
  });

  // Determine available actions based on process state and user role
  const getActions = () => {
    if (dsrole === "provider") {
      switch (process.state) {
        case "REQUESTED":
          return [
            {
              label: "Terminate",
              variant: "destructive",
              Component: ContractNegotiationTerminationDialog,
            },
            {
              label: "Counter offer",
              variant: "outline",
              Component: ContractNegotiationOfferDialog,
            },
            { label: "Agree", variant: "default", Component: ContractNegotiationAgreementDialog },
          ];
        case "OFFERED":
          return [
            {
              label: "Terminate",
              variant: "destructive",
              Component: ContractNegotiationTerminationDialog,
            },
          ];
        case "ACCEPTED":
          return [
            { label: "Agree", variant: "default", Component: ContractNegotiationAgreementDialog },
          ];
        case "AGREED":
          return [
            {
              label: "Terminate",
              variant: "destructive",
              Component: ContractNegotiationTerminationDialog,
            },
          ];
        case "VERIFIED":
          return [
            {
              label: "Finalize",
              variant: "default",
              Component: ContractNegotiationFinalizationDialog,
            },
            {
              label: "Terminate",
              variant: "destructive",
              Component: ContractNegotiationTerminationDialog,
            },
          ];
        default:
          return [];
      }
    } else if (dsrole === "consumer") {
      switch (process.state) {
        case "REQUESTED":
          return [
            {
              label: "Terminate",
              variant: "destructive",
              Component: ContractNegotiationTerminationDialog,
            },
          ];
        case "OFFERED":
          return [
            { label: "Accept", variant: "default", Component: ContractNegotiationAcceptanceDialog },
            {
              label: "Counter request",
              variant: "outline",
              Component: ContractNegotiationRequestDialog,
            },
            {
              label: "Terminate",
              variant: "destructive",
              Component: ContractNegotiationTerminationDialog,
            },
          ];
        case "AGREED":
          return [
            {
              label: "Verify",
              variant: "default",
              Component: ContractNegotiationVerificationDialog,
            },
          ];
        default:
          return [];
      }
    }
    return [];
  };

  // Get the actions for the current process state and user role
  const actions = getActions();

  // Determine if no further actions are available
  const showNoFurtherActions = () => {
    if (process.state === "FINALIZED" || process.state === "TERMINATED") return true;
    if (dsrole === "consumer") {
      if (process.state === "ACCEPTED") return true;
      if (process.state === "VERIFIED") return true;
    }
    return false;
  };

  return (
    <div className={containerClassName({ tiny })}>
      <div
        className={
          process.state === "OFFERED" ||
          process.state === "ACCEPTED" ||
          process.state === "VERIFIED"
            ? "flex justify-end flex-row-reverse gap-2"
            : process.state === "REQUESTED"
              ? "space-x-2 min-w-[260px]"
              : "flex justify-start gap-2"
        }
      >
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
