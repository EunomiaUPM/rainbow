import {DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle} from "./ui/dialog";
import {Button} from "shared/src/components/ui/button";
import React, {useContext, useRef} from "react";
import {Form} from "shared/src/components/ui/form";
import {useForm} from "react-hook-form";
import {GlobalInfoContext, GlobalInfoContextType} from "shared/src/context/GlobalInfoContext";
import {usePostNewBusinessRequest} from "shared/src/data/business-mutations";
import {AuthContext, AuthContextType} from "shared/src/context/AuthContext";
import {PolicyWrapperShow} from "shared/src/components/PolicyWrapperShow";

export const BusinessRequestAccessDialog = ({policy, catalogId, datasetId}: {
    policy: OdrlOffer,
    catalogId: UUID,
    datasetId: UUID
}) => {
    // --- Form Setup ---
    const closeDialogRef = useRef<HTMLButtonElement>(null);
    const form = useForm({});
    const {handleSubmit, control, setValue, getValues} = form;
    const {mutateAsync: requestAsync} = usePostNewBusinessRequest()
    const {api_gateway} = useContext<GlobalInfoContextType>(GlobalInfoContext)
    const {participant} = useContext<AuthContextType | null>(AuthContext)!

    const onSubmit = async () => {
        await requestAsync({
            api_gateway,
            content: {
                consumerParticipantId: participant?.participant_id,
                offer: {
                    "@id": policy["@id"]
                }

            }
        })
        form.reset()
        closeDialogRef.current?.click();
    }

    return (
        <DialogContent className="max-w-fit sm:max-w-fi ">
            <DialogHeader>
                <DialogTitle>Dataset access dialog</DialogTitle>
                <DialogDescription className="max-w-full flex flex-col break-all gap-2">
                    <div className="max-w-full flex">
                        You are to request access to dataset under policy.
                    </div>
                    <div>
                        {catalogId}
                    </div>
                    <div>
                        {datasetId}
                    </div>
                    <div>
                        <PolicyWrapperShow policy={policy}/>
                    </div>
                </DialogDescription>
            </DialogHeader>

            <Form {...form}>
                <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
                    <DialogFooter className="[&>*]:w-full">
                        <DialogClose asChild ref={closeDialogRef}>
                            <Button variant="ghost" type="reset">
                                Cancel
                            </Button>
                        </DialogClose>
                        <Button variant="default" type="submit">
                            Request access
                        </Button>
                    </DialogFooter>
                </form>
            </Form>
        </DialogContent>
    );
};
