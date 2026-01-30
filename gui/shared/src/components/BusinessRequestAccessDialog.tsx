import {
  DialogBody,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "./ui/dialog";
import { formatUrn } from "shared/src/lib/utils";
import {Button} from "shared/src/components/ui/button";
import {Badge} from "shared/src/components/ui/badge";
import React, {useContext, useRef} from "react";
import {Form} from "shared/src/components/ui/form";
import {useForm} from "react-hook-form";
import {GlobalInfoContext, GlobalInfoContextType} from "shared/src/context/GlobalInfoContext";
import {usePostNewBusinessRequest} from "shared/src/data/business-mutations";
import {AuthContext, AuthContextType} from "shared/src/context/AuthContext";
import {PolicyWrapperShow} from "shared/src/components/PolicyWrapperShow";

export const BusinessRequestAccessDialog = ({
                                              policy,
                                              catalogId,
                                              datasetId,
                                              datasetName,
                                            }: {
  policy: OdrlOffer;
  catalogId: UUID;
  datasetId: UUID;
  datasetName: string;
}) => {
  // --- Form Setup ---
  const closeDialogRef = useRef<HTMLButtonElement>(null);
  const form = useForm({});
  const {handleSubmit, control, setValue, getValues} = form;
  const {mutateAsync: requestAsync} = usePostNewBusinessRequest();
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const {participant} = useContext<AuthContextType | null>(AuthContext)!;

  const onSubmit = async () => {
    await requestAsync({
      api_gateway,
      content: {
        //@ts-ignore
        consumerParticipantId: participant?.participant_id,
        offer: {
          "@id": policy["@id"],
        },
      },
    });
    form.reset();
    closeDialogRef.current?.click();
  };

  return (
    <DialogContent className=" p-0 flex flex-col h-fit max-h-[90dvh]">
      <DialogHeader className="px-6 pt-6">
        <DialogTitle>Dataset access dialog</DialogTitle>
        <DialogDescription className="max-w-full flex flex-col break-all gap-2">
          <span className="max-w-full flex flex-col gap-0 mb-0">
            <span>
              {" "}
              You are about to request access to dataset
              <Badge variant="infoLighter" className="mx-1 mt-1">
                {datasetName}
              </Badge>
              in catalog{" "}
              <Badge variant="infoLighter" className="mx-1 mt-1">
                {formatUrn(catalogId)}
              </Badge>
              under policy with ID{" "}
              <Badge variant="infoLighter" className="mx-1 mt-1">
                {formatUrn(policy["@id"])}
              </Badge>
            </span>
          </span>
        </DialogDescription>
      </DialogHeader>
      <DialogBody>
        <PolicyWrapperShow policy={policy} datasetId={undefined} catalogId={undefined}
                           participant={undefined} datasetName={""}/>
      </DialogBody>

      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          <DialogFooter className="[&>*]:w-full p-6 pt-0">
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
