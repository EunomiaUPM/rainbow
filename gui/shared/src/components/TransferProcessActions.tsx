import React, {useContext} from "react";
import {cva} from "class-variance-authority";
import {GlobalInfoContext, GlobalInfoContextType} from "shared/src/context/GlobalInfoContext";
import {Dialog, DialogTrigger} from "shared/src/components/ui/dialog";
import {Button} from "shared/src/components/ui/button";
import {TransferProcessStartDialog} from "shared/src/components/TransferProcessStartDialog";
import {TransferProcessTerminationDialog} from "shared/src/components/TransferProcessTerminationDialog";
import {TransferProcessSuspensionDialog} from "shared/src/components/TransferProcessSuspensionDialog";
import {TransferProcessCompletionDialog} from "shared/src/TransferProcessCompletionDialog";

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
        true: "inline-flex items-center",
        false: "bg-gray-100 p-6",
      },
    },
  });

    return (
        <div className={containerClassName({tiny})}>
            <h2 className={h2ClassName({tiny})}>Actions</h2>
            {process.state === "REQUESTED" && (<div className="space-x-2">
                {role === "provider" && (<>
                    <Dialog>
                        <DialogTrigger asChild>
                            <Button>Start</Button>
                        </DialogTrigger>
                        <TransferProcessStartDialog process={process}/>
                    </Dialog>
                    <Dialog>
                        <DialogTrigger asChild>
                            <Button>Terminate</Button>
                        </DialogTrigger>
                        <TransferProcessTerminationDialog process={process}/>
                    </Dialog>
                </>)}
                {role === "consumer" && (<>
                        <Dialog>
                            <DialogTrigger asChild>
                                <Button>Terminate</Button>
                            </DialogTrigger>
                            <TransferProcessTerminationDialog process={process}/>
                        </Dialog>
                    </>
                )}
            </div>)}
            {process.state === "STARTED" && (<div className="space-x-2">
                {role === "provider" && (<>
                        {(process.state_attribute === "ON_REQUEST" || process.state_attribute === "BY_PROVIDER") && (<>

                        </>)}
                        <Dialog>
                            <DialogTrigger asChild>
                                <Button>Suspend</Button>
                            </DialogTrigger>
                            <TransferProcessSuspensionDialog process={process}/>
                        </Dialog>
                        <Dialog>
                            <DialogTrigger asChild>
                                <Button>Complete</Button>
                            </DialogTrigger>
                            <TransferProcessCompletionDialog process={process}/>
                        </Dialog>
                        <Dialog>
                            <DialogTrigger asChild>
                                <Button>Terminate</Button>
                            </DialogTrigger>
                            <TransferProcessTerminationDialog process={process}/>
                        </Dialog>
                    </>

                )}
                {role === "consumer" && (<>
                    <Dialog>
                        <DialogTrigger asChild>
                            <Button>Suspend</Button>
                        </DialogTrigger>
                        <TransferProcessSuspensionDialog process={process}/>
                    </Dialog>
                    <Dialog>
                        <DialogTrigger asChild>
                            <Button>Complete</Button>
                        </DialogTrigger>
                        <TransferProcessCompletionDialog process={process}/>
                    </Dialog>
                    <Dialog>
                        <DialogTrigger asChild>
                            <Button>Terminate</Button>
                        </DialogTrigger>
                        <TransferProcessTerminationDialog process={process}/>
                    </Dialog>
                </>)}
            </div>)}
            {process.state === "SUSPENDED" && (<div className="space-x-2">
                {role === "provider" && (<>
                    {process.state_attribute === "BY_PROVIDER" && (<>
                        <Dialog>
                            <DialogTrigger asChild>
                                <Button>Start</Button>
                            </DialogTrigger>
                            <TransferProcessStartDialog process={process}/>
                        </Dialog>
                        <Dialog>
                            <DialogTrigger asChild>
                                <Button>Complete</Button>
                            </DialogTrigger>
                            <TransferProcessCompletionDialog process={process}/>
                        </Dialog>
                    </>)}

                    <Dialog>
                        <DialogTrigger asChild>
                            <Button>Terminate</Button>
                        </DialogTrigger>
                        <TransferProcessTerminationDialog process={process}/>
                    </Dialog>
                </>)}
                {role === "consumer" && (<>
                    <Dialog>
                        <DialogTrigger asChild>
                            <Button>Start</Button>
                        </DialogTrigger>
                        <TransferProcessStartDialog process={process}/>
                    </Dialog>
                    <Dialog>
                        <DialogTrigger asChild>
                            <Button>Complete</Button>
                        </DialogTrigger>
                        <TransferProcessCompletionDialog process={process}/>
                    </Dialog>
                    <Dialog>
                        <DialogTrigger asChild>
                            <Button>Terminate</Button>
                        </DialogTrigger>
                        <TransferProcessTerminationDialog process={process}/>
                    </Dialog>
                </>)}
            </div>)}
            {process.state === "COMPLETED" && (<div className="space-x-2">
                {role === "provider" && (<>
                    <div>No further actions</div>
                </>)}
                {role === "consumer" && (<>

                </>)}

            </div>)}
            {process.state === "TERMINATED" && (<div className="space-x-2">
                {role === "provider" && (<>
                    <div>No further actions</div>
                </>)}
                {role === "consumer" && (<>
                    <div>No further actions</div>
                </>)}
            </div>)}
        </div>
    )
}