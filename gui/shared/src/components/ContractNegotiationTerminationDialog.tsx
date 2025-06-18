import {DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle} from "./ui/dialog";
import {Button} from "./ui/button";
import React, {useContext} from "react";
import {Form} from "./ui/form";
import {useForm} from "react-hook-form";
import {usePostContractNegotiationRPCTermination} from "./../data/contract-mutations";
import {GlobalInfoContext, GlobalInfoContextType} from "./../context/GlobalInfoContext";

export const ContractNegotiationTerminationDialog = ({process}: { process: CNProcess }) => {
    // --- Form Setup ---
    const form = useForm({});
    const {handleSubmit, control, setValue, getValues} = form;
    const {mutateAsync: terminateAsync} = usePostContractNegotiationRPCTermination()
    const {api_gateway, role} = useContext<GlobalInfoContextType>(GlobalInfoContext)
    const onSubmit = () => {
        if (role === "consumer") {
            terminateAsync({
                api_gateway: api_gateway,
                content: {
                    providerParticipantId: process.associated_provider,
                    consumerPid: process.consumer_id,
                    providerPid: process.provider_id
                }
            })
        }
        if (role === "provider") {
            terminateAsync({
                api_gateway: api_gateway,
                content: {
                    consumerParticipantId: process.associated_consumer,
                    consumerPid: process.consumer_id,
                    providerPid: process.provider_id
                }
            })
        }
    }


    return <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
            <DialogTitle>Termination dialog</DialogTitle>
            <DialogDescription className="break-all">
                <p> You are about to terminate to the terms of the contract negotiation. Please review the details
                    carefully
                    before proceeding.</p>
                <p>{JSON.stringify(process)}</p>
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