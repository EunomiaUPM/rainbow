import {DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,} from "./ui/dialog";
import {Button} from "./ui/button";
import {Badge} from "./../components/ui/badge";
import {List, ListItem, ListItemKey} from "./ui/list";

import React, {useContext, useRef} from "react";
import {Form} from "./ui/form";
import {useForm} from "react-hook-form";
import {usePostContractNegotiationRPCOffer} from "./../data/contract-mutations";
import {useGetLastContractNegotiationOfferByCNMessageId} from "@/data/contract-queries.ts";
import {GlobalInfoContext, GlobalInfoContextType} from "./../context/GlobalInfoContext";
import dayjs from "dayjs";
import {PolicyEditorHandle, PolicyWrapperEdit} from "./PolicyWrapperEdit";

export const ContractNegotiationOfferDialog = ({process}: { process: CNProcess }) => {
    const policyWrapperRef = useRef<PolicyEditorHandle>();
    const form = useForm();
    const {handleSubmit} = form;
    const {mutateAsync: dataOfferAsync} = usePostContractNegotiationRPCOffer();
    const {api_gateway} = useContext<GlobalInfoContextType>(GlobalInfoContext);
    const {data: lastOffer} = useGetLastContractNegotiationOfferByCNMessageId(process.provider_id);

    const onSubmit = async () => {
        if (policyWrapperRef.current) {
            const policy = policyWrapperRef.current.getPolicy();
            const outputOffer = {
                ...lastOffer.offer_content,
                prohibition:
                    policy.prohibition && policy.prohibition.length > 0 ? policy.prohibition : null,
                permission: policy.permission && policy.permission.length > 0 ? policy.permission : null,
                obligation: policy.obligation && policy.obligation.length > 0 ? policy.obligation : null,
            };
            await dataOfferAsync({
                api_gateway: api_gateway,
                content: {
                    consumerParticipantId: process.associated_consumer,
                    offer: outputOffer,
                    consumerPid: process.consumer_id,
                    providerPid: process.provider_id,
                },
            });
        }
    };

    const scopedListItemKeyClasses = "basis-[30%]";

    return (
        <DialogContent className="p-0">
            <Form {...form}>
                <form
                    onSubmit={handleSubmit(onSubmit)}
                    className="space-y-6 flex flex-col h-fit max-h-[90dvh]"
                >
                    <DialogHeader className="px-6 pt-6">
                        <DialogTitle>Contract Negotiation Offer</DialogTitle>
                        <DialogDescription>
                            <p>Make changes on the Contract Negotiation Offer.</p>
                            {/* <p>{JSON.stringify(process)}</p> */}
                        </DialogDescription>
                    </DialogHeader>
                    {/* List */}
                    <div className=" overflow-y-scroll px-6">
                        <List className="w-full  px-2">
                            <ListItem>
                                <ListItemKey className={scopedListItemKeyClasses}>Provider id:</ListItemKey>
                                <Badge variant={"info"}>{process.provider_id.slice(9, -1)}</Badge>
                            </ListItem>
                            <ListItem>
                                <ListItemKey className={scopedListItemKeyClasses}>Consumer id:</ListItemKey>
                                <Badge variant={"info"}>{process.consumer_id.slice(9, -1)}</Badge>
                            </ListItem>
                            <ListItem>
                                <ListItemKey className={scopedListItemKeyClasses}>Associated Consumer:</ListItemKey>
                                <Badge variant={"info"}>{process.associated_consumer.slice(9, 40) + "[...]"}</Badge>
                            </ListItem>
                            <ListItem>
                                <ListItemKey className={scopedListItemKeyClasses}>State:</ListItemKey>
                                <Badge variant={"status"} state={process.state}>
                                    {process.state}
                                </Badge>
                            </ListItem>
                            <ListItem>
                                <ListItemKey className={scopedListItemKeyClasses}>Iniciated by:</ListItemKey>
                                <Badge variant={"role"} role={process.initiated_by}>
                                    {process.initiated_by}
                                </Badge>
                            </ListItem>
                            <ListItem>
                                <ListItemKey className={scopedListItemKeyClasses}>Created at:</ListItemKey>
                                {dayjs(process.created_at).format("DD/MM/YY HH:mm")}
                            </ListItem>
                            {process.updated_at && (
                                <ListItem>
                                    <ListItemKey className={scopedListItemKeyClasses}>Updated at:</ListItemKey>
                                    {dayjs(process.updated_at).format("DD/MM/YY HH:mm")}
                                </ListItem>
                            )}
                        </List>

                        {/* / List content */}
                        <div className="h-6"></div>
                        {lastOffer && (
                            <div className="flex w-full ">
                                <div className="w-full">
                                    <p className="mb-2">New policy request</p>
                                    <PolicyWrapperEdit policy={lastOffer.offer_content} ref={policyWrapperRef}/>
                                </div>
                            </div>
                        )}
                    </div>
                    <DialogFooter className="[&>*]:w-full p-6 pt-0">
                        <DialogClose asChild>
                            <Button variant="ghost" type="reset">
                                Cancel
                            </Button>
                        </DialogClose>
                        <Button type="submit" variant="outline">
                            Counter offer
                        </Button>
                    </DialogFooter>
                </form>
            </Form>
        </DialogContent>
    );
};
