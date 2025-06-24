import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "./ui/dialog";
import { Button } from "./ui/button";
import React, { useContext } from "react";
import { Form } from "./ui/form";
import { useForm } from "react-hook-form";
import { usePostContractNegotiationRPCAgreement } from "./../data/contract-mutations";
import {
  GlobalInfoContext,
  GlobalInfoContextType,
} from "./../context/GlobalInfoContext";
import { Badge } from "./../components/ui/badge";
import { List, ListItem, ListItemKey } from "./../components/ui/list";



export const ContractNegotiationAgreementDialog = ({
  process,
}: {
  process: CNProcess;
}) => {
  // --- Form Setup ---
  const form = useForm({});
  const { handleSubmit, control, setValue, getValues } = form;
  const { mutateAsync: agreeAsync } = usePostContractNegotiationRPCAgreement();
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const onSubmit = () => {
    agreeAsync({
      api_gateway: api_gateway,
      content: {
        consumerParticipantId: process.associated_consumer,
        consumerPid: process.consumer_id,
        providerPid: process.provider_id,
      },
    });
  };
  const scopedListItemKeyClasses = "basis-[33%]";
  return (
    <DialogContent className="min-w-fit w-full">
      <DialogHeader>
        <DialogTitle>Agreement dialog</DialogTitle>
        <DialogDescription className="break-all">
          <span>
            You are about to agree to the terms of the contract negotiation.
            <br />
            Please review the details carefully before proceeding.
          </span>
          {/* <code>{JSON.stringify(process)}</code> */}
        </DialogDescription>
      </DialogHeader>

        {/* List JSON */}
        <List className="min-w-fit w-full">
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
          <ListItem>
            <ListItemKey className={scopedListItemKeyClasses}>
              Associated Consumer id:
            </ListItemKey>
            <Badge variant={"info"}>
              {process.associated_consumer.slice(9, -1)}
            </Badge>
          </ListItem>
          <ListItem>
            <ListItemKey className={scopedListItemKeyClasses}>Current state:</ListItemKey>
            <Badge variant={"status"} state={process.state}>
              {process.state}
            </Badge>
          </ListItem>
          <ListItem>
            <ListItemKey className={scopedListItemKeyClasses}>Initiated by:</ListItemKey>
            <Badge variant={"role"} role={process.initiated_by}>
              {process.initiated_by}
            </Badge>
          </ListItem>
          <ListItem>
            <ListItemKey className={scopedListItemKeyClasses}>Created at:</ListItemKey>
            <p>{process.created_at}</p>
          </ListItem>
          {process.updated_at && (
            <ListItem>
              <ListItemKey>Updated at:</ListItemKey>
              <p>{process.updated_at}</p>
            </ListItem>
          )}
        </List>
        {/* / List content */}
      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          <DialogFooter  className="[&>*]:w-full">
            <DialogClose asChild>
              <Button variant="ghost" type="reset">
                Cancel
              </Button>
            </DialogClose>
            <Button type="submit">Agree</Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
