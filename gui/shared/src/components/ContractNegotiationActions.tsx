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
import NoFurtherActions from "./ui/noFurtherActions";

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
        true: "inline-flex items-center ",
        false:
          " w-full p-6 fixed bottom-0 left-0 md:left-[223px] bg-background/80 backdrop-blur-sm border border-t-stroke [&>*>button]:min-w-20",
      },
    },
  });

  return (
    <div className={containerClassName({ tiny })}>
      {/* <h2 className={h2ClassName({ tiny })}></h2> */}
      {process.state === "REQUESTED" && (
        <div className="space-x-2 min-w-[260px]">
          {role === "provider" && (
            <>
              <Dialog>
                <DialogTrigger asChild>
                  <Button variant="destructive" size={tiny ? "sm" : ""}>
                    Terminate
                  </Button>
                </DialogTrigger>
                <ContractNegotiationTerminationDialog process={process} />
              </Dialog>
              <Dialog>
                <DialogTrigger asChild>
                  <Button variant="outline" size={tiny ? "sm" : ""}>
                    Counter offer
                    {/* Offer */}
                  </Button>
                </DialogTrigger>
                <ContractNegotiationOfferDialog process={process} />
              </Dialog>
              <Dialog>
                <DialogTrigger asChild>
                  <Button size={tiny ? "sm" : ""}>Agree</Button>
                </DialogTrigger>

                <ContractNegotiationAgreementDialog process={process} />
              </Dialog>
            </>
          )}
          {role === "consumer" && (
            <Dialog>
              <DialogTrigger asChild>
                <Button variant="destructive" size={tiny ? "sm" : ""}>
                  Terminate
                </Button>
              </DialogTrigger>
              <ContractNegotiationTerminationDialog process={process} />
            </Dialog>
          )}
        </div>
      )}
      {process.state === "OFFERED" && (
        <div className="flex justify-end flex-row-reverse gap-2">
          {role === "provider" && (
            <Dialog>
              <DialogTrigger asChild>
                <Button variant="destructive" size={tiny ? "sm" : ""}>
                  Terminate
                </Button>
              </DialogTrigger>
              <ContractNegotiationTerminationDialog process={process} />
            </Dialog>
          )}
          {role === "consumer" && (
            <>
              <Dialog>
                <DialogTrigger asChild>
                  <Button size={tiny ? "sm" : ""}>Accept</Button>
                </DialogTrigger>

                <ContractNegotiationAcceptanceDialog process={process} />
              </Dialog>
              <Dialog>
                <DialogTrigger asChild>
                  <Button variant="outline" size={tiny ? "sm" : ""}>
                    Counter request
                    {/* Request */}
                  </Button>
                </DialogTrigger>
                <ContractNegotiationRequestDialog process={process} />
              </Dialog>
              <Dialog>
                <DialogTrigger asChild>
                  <Button variant="destructive" size={tiny ? "sm" : ""}>
                    Terminate
                  </Button>
                </DialogTrigger>
                <ContractNegotiationTerminationDialog process={process} />
              </Dialog>
            </>
          )}
        </div>
      )}
      {process.state === "ACCEPTED" && (
        <div className="flex justify-end flex-row-reverse gap-2">
          {role === "provider" && (
            <>
              <Dialog>
                <DialogTrigger asChild>
                  <Button size={tiny ? "sm" : ""}>Agree</Button>
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
              <NoFurtherActions />
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
        <div className="flex justify-start gap-2">
          {role === "provider" && (
            <>
              <Dialog>
                <DialogTrigger asChild>
                  <Button variant="destructive" size={tiny ? "sm" : ""}>
                    Terminate
                  </Button>
                </DialogTrigger>
                <ContractNegotiationTerminationDialog process={process} />
              </Dialog>
            </>
          )}
          {role === "consumer" && (
            <>
              <Dialog>
                <DialogTrigger asChild>
                  <Button size={tiny ? "sm" : ""}>Verify</Button>
                </DialogTrigger>
                <ContractNegotiationVerificationDialog process={process} />
              </Dialog>
            </>
          )}
        </div>
      )}
      {process.state === "VERIFIED" && (
        <div className="flex justify-end flex-row-reverse gap-2">
          {role === "provider" && (
            <>
              <Dialog>
                <DialogTrigger asChild>
                  <Button size={tiny ? "sm" : ""}>Finalize</Button>
                </DialogTrigger>
                <ContractNegotiationFinalizationDialog process={process} />
              </Dialog>
              <Dialog>
                <DialogTrigger asChild>
                  <Button variant="destructive" size={tiny ? "sm" : ""}>
                    Terminate
                  </Button>
                </DialogTrigger>

                <ContractNegotiationTerminationDialog process={process} />
              </Dialog>
            </>
          )}
          {role === "consumer" && (
            <>
              <NoFurtherActions />
            </>
          )}
        </div>
      )}
      {process.state === "FINALIZED" && <NoFurtherActions />}
      {process.state === "TERMINATED" && <NoFurtherActions />}
    </div>
  );
};
