import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "../ui/dialog";
import {Button} from "../ui/button";
import React, {useContext} from "react";
import {Form} from "../ui/form";
import {useForm} from "react-hook-form";
import {GlobalInfoContext, GlobalInfoContextType} from "../../context/GlobalInfoContext";
import { formatUrn } from "shared/src/lib/utils";
import {useDeleteBusinessNewPolicyInDataset} from "shared/src/data/business-mutations";
import {Badge} from "shared/src/components/ui/badge";

export const BusinessRemovePolicyDialog = ({
                                             policy,
                                             catalogId,
                                             datasetId,
                                           }: {
  policy: OdrlOffer;
  catalogId: UUID;
  datasetId: UUID;
}) => {
  // --- Form Setup ---
  const form = useForm({});
  const {handleSubmit, control, setValue, getValues} = form;
  const {mutateAsync: deleteAsync} = useDeleteBusinessNewPolicyInDataset();
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const onSubmit = () => {
    deleteAsync({
      api_gateway,
      catalogId,
      datasetId,
      policyId: policy["@id"],
    });
  };

  return (
    <DialogContent className="max-w-fit sm:max-w-fit ">
      <DialogHeader>
        <DialogTitle>Delete policy dialog</DialogTitle>
        <DialogDescription className="max-w-full flex flex-wrap break-all">
          <span className="max-w-full flex flex-wrap">
            Are you sure you want to delete Policy with ID{" "}
            <Badge variant="info">{formatUrn(policy["@id"])}</Badge>
          </span>
        </DialogDescription>
      </DialogHeader>

      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          <DialogFooter className="[&>*]:w-full">
            <DialogClose asChild>
              <Button variant="ghost" type="reset">
                Cancel
              </Button>
            </DialogClose>
            <Button variant="destructive" type="submit">
              Remove policy
            </Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
