import {DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle} from "./ui/dialog";
import {Button} from "./ui/button";
import React, {useContext} from "react";
import {Form} from "./ui/form";
import {useForm} from "react-hook-form";
import {GlobalInfoContext, GlobalInfoContextType} from "./../context/GlobalInfoContext";
import {usePostTransferRPCStart} from "shared/src/data/transfer-mutations";

export const TransferProcessTerminationDialog = ({process}: { process: TransferProcess }) => {
    // --- Form Setup ---
    const form = useForm({});
    const {handleSubmit, control, setValue, getValues} = form;
    const {mutateAsync: startAsync} = usePostTransferRPCStart()
    const {api_gateway} = useContext<GlobalInfoContextType>(GlobalInfoContext)
    const onSubmit = () => {
        startAsync({
            api_gateway: api_gateway,
            content: {
                consumerParticipantId: process.associated_consumer,
                consumerCallbackAddress: process.data_plane_id,
                consumerPid: process.consumer_pid,
                providerPid: process.provider_pid
            }
        })
    }

    return <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
            <DialogTitle>Transfer termination dialog</DialogTitle>
            <DialogDescription className="break-all">
                <span>You are about to terminate the transfer process.</span>
                <span>{JSON.stringify(process)}</span>
            </DialogDescription>
        </DialogHeader>
        <Form {...form}>
            <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
                <DialogFooter>
                    <DialogClose asChild>
                        <Button variant="outline" type="reset">Cancel</Button>
                    </DialogClose>
                    <Button type="submit">Terminate</Button>
                </DialogFooter>
            </form>
        </Form>
    </DialogContent>
}