import {DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle} from "./ui/dialog";
import {Button} from "./ui/button";
import React, {useContext} from "react";
import {Form} from "./ui/form";
import {useForm} from "react-hook-form";
import {usePostContractNegotiationRPCAgreement} from "./../data/contract-mutations";
import {GlobalInfoContext, GlobalInfoContextType} from "./../context/GlobalInfoContext";

export const ContractNegotiationAgreementDialog = ({process}: { process: CNProcess }) => {
    // --- Form Setup ---
    const form = useForm({});
    const {handleSubmit, control, setValue, getValues} = form;
    const {mutateAsync: agreeAsync} = usePostContractNegotiationRPCAgreement()
    const {api_gateway} = useContext<GlobalInfoContextType>(GlobalInfoContext)
    const onSubmit = () => {
        agreeAsync({
            api_gateway: api_gateway,
            content: {
                consumerParticipantId: process.associated_consumer,
                consumerPid: process.consumer_id,
                providerPid: process.provider_id
            }
        })
    }


    return <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
            <DialogTitle>Agreement dialog</DialogTitle>
            <DialogDescription className="break-all">
                <span>You are about to agree to the terms of the contract negotiation. Please review the details carefully
                    before proceeding.</span>
                <span>{JSON.stringify(process)}</span>
            </DialogDescription>
        </DialogHeader>
        <Form {...form}>
            <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
                <DialogFooter>
                    <DialogClose asChild>
                        <Button variant="outline" type="reset">Cancel</Button>
                    </DialogClose>
                    <Button type="submit">Agree</Button>
                </DialogFooter>
            </form>
        </Form>
    </DialogContent>
}