import { Button } from "./ui/button";
import React, { useContext } from "react";
import { cva } from "class-variance-authority";
import { Dialog, DialogTrigger } from "./ui/dialog";
import { ContractNegotiationOfferDialog } from "./ContractNegotiationOfferDialog";
import { ContractNegotiationAgreementDialog } from "./ContractNegotiationAgreementDialog";
import { ContractNegotiationTerminationDialog } from "./ContractNegotiationTerminationDialog";
import { ContractNegotiationFinalizationDialog } from "./ContractNegotiationFinalizationDialog";
import {
  GlobalInfoContext,
  GlobalInfoContextType,
} from "shared/src/context/GlobalInfoContext";
import { ContractNegotiationRequestDialog } from "./ContractNegotiationRequestDialog";
import { ContractNegotiationAcceptanceDialog } from "./ContractNegotiationAcceptanceDialog";
import { ContractNegotiationVerificationDialog } from "./ContractNegotiationVerificationDialog";

export const ContractNegotiationActions = ({
  process,
  tiny = false,
}: {
  process: CNProcess;
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
        true: "inline-flex items-center",
        false:
          " w-full p-6 pt-4 fixed bottom-0 left-0 md:left-[223px] bg-primary-950/10 border border-t-stroke [&>button]:uppercase [&>button]md:min-w-40",
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
                  <Button variant="destructive">Terminate</Button>
                </DialogTrigger>
                <ContractNegotiationTerminationDialog process={process} />
              </Dialog>
              <Dialog>
                <DialogTrigger asChild>
                  <Button variant="outline">Offer</Button>
                </DialogTrigger>
                <ContractNegotiationOfferDialog process={process} />
              </Dialog>
              <Dialog>
                <DialogTrigger asChild>
                  <Button>Agree</Button>
                </DialogTrigger>
                <ContractNegotiationAgreementDialog process={process} />
              </Dialog>
            </>
          )}
          {role === "consumer" && (
            <Dialog>
              <DialogTrigger asChild>
                <Button>Terminate</Button>
              </DialogTrigger>
              <ContractNegotiationTerminationDialog process={process} />
            </Dialog>
          )}
        </div>
      )}
      {process.state === "OFFERED" && (
        <div className="space-x-2">
          {role === "provider" && (
            <Dialog>
              <DialogTrigger asChild>
                <Button variant="destructive">Terminate</Button>
              </DialogTrigger>
              <ContractNegotiationTerminationDialog process={process} />
            </Dialog>
          )}
          {role === "consumer" && (
            <>
              <Dialog>
                <DialogTrigger asChild>
                  <Button>Accept</Button>
                </DialogTrigger>
                <ContractNegotiationAcceptanceDialog process={process} />
              </Dialog>
              <Dialog>
                <DialogTrigger asChild>
                  <Button>Request</Button>
                </DialogTrigger>
                <ContractNegotiationRequestDialog process={process} />
              </Dialog>
              <Dialog>
                <DialogTrigger asChild>
                  <Button variant="destructive">Terminate</Button>
                </DialogTrigger>
                <ContractNegotiationTerminationDialog process={process} />
              </Dialog>
            </>
          )}
        </div>
      )}
      {process.state === "ACCEPTED" && (
        <div className="space-x-2">
          {role === "provider" && (
            <>
              <Dialog>
                <DialogTrigger asChild>
                  <Button>Agree</Button>
                </DialogTrigger>
                <ContractNegotiationAgreementDialog process={process} />
              </Dialog>
              {/*<Dialog>*/}
              {/*    <DialogTrigger asChild>*/}
              {/*        <Button variant="destructive">Terminate</Button>*/}
              {/*    </DialogTrigger>*/}
              {/*    <ContractNegotiationTerminationDialog process={process}/>*/}
              {/*</Dialog>*/}
            </>
          )}
          {role === "consumer" && (
            <>
              <div>No further actions</div>
              {/*<Dialog>*/}
              {/*    <DialogTrigger asChild>*/}
              {/*        <Button variant="destructive">Terminate</Button>*/}
              {/*    </DialogTrigger>*/}
              {/*    <ContractNegotiationTerminationDialog process={process}/>*/}
              {/*</Dialog>*/}
            </>
          )}
        </div>
      )}
      {process.state === "AGREED" && (
        <div className="space-x-2">
          {role === "provider" && (
            <>
              <Dialog>
                <DialogTrigger asChild>
                  <Button variant="destructive">Terminate</Button>
                </DialogTrigger>
                <ContractNegotiationTerminationDialog process={process} />
              </Dialog>
            </>
          )}
          {role === "consumer" && (
            <>
              <Dialog>
                <DialogTrigger asChild>
                  <Button>Verify</Button>
                </DialogTrigger>
                <ContractNegotiationVerificationDialog process={process} />
              </Dialog>
            </>
          )}
        </div>
      )}
      {process.state === "VERIFIED" && (
        <div className="space-x-2">
          {role === "provider" && (
            <>
              <Dialog>
                <DialogTrigger asChild>
                  <Button>Finalize</Button>
                </DialogTrigger>
                <ContractNegotiationFinalizationDialog process={process} />
              </Dialog>
              <Dialog>
                <DialogTrigger asChild>
                  <Button variant="destructive">Terminate</Button>
                </DialogTrigger>
                <ContractNegotiationTerminationDialog process={process} />
              </Dialog>
            </>
          )}
          {role === "consumer" && (
            <>
              <div>No further actions</div>
            </>
          )}
        </div>
      )}
      {process.state === "FINALIZED" && <div>No further actions</div>}
      {process.state === "TERMINATED" && <div>No further actions</div>}
    </div>
  );
};
