import {
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from "shared/src/components/ui/dialog";
import {Button} from "shared/src/components/ui/button";
import React, {useContext, useRef} from "react";
import {Form} from "shared/src/components/ui/form";
import {useForm} from "react-hook-form";
import {usePostContractNegotiationRPCRequest} from "shared/src/data/contract-mutations";
import {GlobalInfoContext, GlobalInfoContextType,} from "shared/src/context/GlobalInfoContext";
import {Badge} from "shared/src/components/ui/badge";
import {List, ListItem, ListItemKey} from "shared/src/components/ui/list";
import dayjs from "dayjs";
import {useGetLastContractNegotiationOfferByCNMessageId} from "shared/src/data/contract-queries";
import {PolicyEditorHandle, PolicyWrapperEdit} from "shared/src/components/PolicyWrapperEdit";

export const ContractNegotiationRequestDialog = ({process}: {
    process: CNProcess;
}) => {
    const policyWrapperRef = useRef<PolicyEditorHandle>()
    const form = useForm();
    const {handleSubmit} = form;
    const {mutateAsync: dataRequestAsync} =
        usePostContractNegotiationRPCRequest();
    const {api_gateway} = useContext<GlobalInfoContextType>(GlobalInfoContext);
    const {data: offer} = useGetLastContractNegotiationOfferByCNMessageId(
        process.consumer_id
    );
    const {data: lastOffer} = useGetLastContractNegotiationOfferByCNMessageId(process.consumer_id)

    const onSubmit = async () => {
        if (policyWrapperRef.current) {
            const policy = policyWrapperRef.current.getPolicy()
            const outputOffer = {
                ...lastOffer.offer_content,
                prohibition: policy.prohibition && policy.prohibition.length > 0 ? policy.prohibition : null,
                permission: policy.permission && policy.permission.length > 0 ? policy.permission : null,
                obligation: policy.obligation && policy.obligation.length > 0 ? policy.obligation : null
            };
            await dataRequestAsync({
                api_gateway: api_gateway,
                content: {
                    providerParticipantId: process.associated_provider,
                    offer: outputOffer,
                    consumerPid: process.consumer_id,
                    providerPid: process.provider_id,
                },
            });
        }
    };

    const scopedListItemKeyClasses = "basis-[30%]";

    return (
        <DialogContent className="p-0   ">
            <Form {...form}>
                <form onSubmit={handleSubmit(onSubmit)} className="space-y-6 flex flex-col h-fit max-h-[90dvh]">
                    <DialogHeader className="px-6 pt-6">
                        <DialogTitle>Contract Negotiation Request</DialogTitle>
                        <DialogDescription className="break-all">
                            <p>Make changes on the Contract Negotiation Request.</p>
                        </DialogDescription>
                    </DialogHeader>
                    <div className=" overflow-y-scroll px-6">
                        <List className="w-fit  px-2">
                            <ListItem>
                                <ListItemKey className={scopedListItemKeyClasses}>
                                    Provider id:
                                </ListItemKey>
                                <Badge variant={"info"}>{process.provider_id.slice(9, -1)}</Badge>
                            </ListItem>
                            <ListItem>
                                <ListItemKey className={scopedListItemKeyClasses}>
                                    Consumer id:
                                </ListItemKey>
                                <Badge variant={"info"}>{process.consumer_id.slice(9, -1)}</Badge>
                            </ListItem>
                            {process.associated_consumer && (
                                // Provider GUI
                                <ListItem>
                                    <ListItemKey className={scopedListItemKeyClasses}>
                                        Associated Consumer:
                                    </ListItemKey>
                                    <Badge variant={"info"}>
                                        {process.associated_consumer.slice(9, -1)}
                                    </Badge>
                                </ListItem>
                            )}
                            {process.associated_provider && (
                                // Consumer GUI
                                <ListItem>
                                    <ListItemKey className={scopedListItemKeyClasses}>
                                        Associated Provider:
                                    </ListItemKey>
                                    <Badge variant={"info"}>
                                        {process.associated_provider.slice(9, 44) + "[...]"}
                                    </Badge>
                                </ListItem>
                            )}
                            <ListItem>
                                <ListItemKey className={scopedListItemKeyClasses}>
                                    State:
                                </ListItemKey>
                                <Badge variant={"status"} state={process.state}>
                                    {process.state}
                                </Badge>
                            </ListItem>
                            {process.initiated_by && (
                                <ListItem>
                                    <ListItemKey className={scopedListItemKeyClasses}>
                                        Iniciated by:
                                    </ListItemKey>
                                    <Badge variant={"role"} role={process.initiated_by}>
                                        {process.initiated_by}
                                    </Badge>
                                </ListItem>
                            )}
                            <ListItem>
                                <ListItemKey className={scopedListItemKeyClasses}>
                                    Created at:
                                </ListItemKey>
                                {dayjs(process.created_at).format("DD/MM/YY HH:mm")}
                            </ListItem>
                            {process.updated_at && (
                                <ListItem>
                                    <ListItemKey className={scopedListItemKeyClasses}>
                                        Updated at:
                                    </ListItemKey>
                                    {dayjs(process.updated_at).format("DD/MM/YY HH:mm")}
                                </ListItem>
                            )}
                        </List>
                        {/* / List content */}
                        <div className="h-6"></div>

                        {offer && (
                            <div className="flex w-full ">
                                <div className="w-full">
                                    <p className="mb-2">New policy request</p>
                                    <PolicyWrapperEdit
                                        policy={offer.offer_content}
                                        ref={policyWrapperRef}
                                    />
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
                        <Button variant="outline" type="submit">
                            Request
                        </Button>
                    </DialogFooter>
                </form>
            </Form>
        </DialogContent>
    );
};
