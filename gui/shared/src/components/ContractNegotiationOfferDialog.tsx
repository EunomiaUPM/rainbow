import {DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle} from "./ui/dialog";
import {Button} from "./ui/button";
import React, {useContext} from "react";
import {Form, FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage} from "./ui/form";
import {useForm} from "react-hook-form";
import {Textarea} from "./ui/textarea";
import {usePostContractNegotiationRPCOffer} from "./../data/contract-mutations";
import {GlobalInfoContext, GlobalInfoContextType} from "./../context/GlobalInfoContext";

export const ContractNegotiationOfferDialog = ({process}: { process: CNProcess }) => {
    // --- Form Setup ---
    const form = useForm({
        defaultValues: {
            odrl: "{\"@id\":\"urn:uuid:071c9c85-cddd-4cb8-9b9b-fcacf88d4687\",\"@type\":\"Offer\",\"permission\":[{\"action\":\"use\",\"constraint\":[{\"leftOperand\":\"did:web:hola.es\",\"operator\":\"eq\",\"rightOperand\":\"user\"}]}],\"target\":\"urn:uuid:c9d4516d-dc86-4662-931b-12f92edfe94b\"}",
        },
    });
    const {handleSubmit, control, setValue, getValues} = form;
    const {mutateAsync: dataOfferAsync} = usePostContractNegotiationRPCOffer()
    const {api_gateway} = useContext<GlobalInfoContextType>(GlobalInfoContext)
    const onSubmit = (data: any) => {
        console.log("Form submitted with data:", data);
        dataOfferAsync({
            api_gateway: api_gateway,
            content: {
                consumerParticipantId: process.associated_consumer,
                offer: JSON.parse(data.odrl),
                consumerPid: process.consumer_id,
                providerPid: process.provider_id
            }
        })
    }


    return <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
            <DialogTitle>Contract Negotiation Offer</DialogTitle>
            <DialogDescription className="break-all">
                <p>Make changes on the Contract Negotiation Offer.</p>
                <p>{JSON.stringify(process)}</p>
            </DialogDescription>
        </DialogHeader>
        <Form {...form}>
            <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
                <FormField
                    control={form.control}
                    name="odrl"
                    render={({field}) => (
                        <FormItem>
                            <FormLabel>Odrl</FormLabel>
                            <FormControl>
                                <Textarea {...field} />
                            </FormControl>
                            <FormDescription>Provide the ODRL policy content</FormDescription>
                            <FormMessage/>
                        </FormItem>
                    )}
                />
                <DialogFooter>
                    <DialogClose asChild>
                        <Button variant="outline" type="reset">Cancel</Button>
                    </DialogClose>
                    <Button type="submit">Offer</Button>
                </DialogFooter>
            </form>
        </Form>
    </DialogContent>
}