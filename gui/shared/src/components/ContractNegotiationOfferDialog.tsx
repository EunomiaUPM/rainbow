import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "./ui/dialog";
import { Button } from "./ui/button";
import { Badge } from "./../components/ui/badge";
import { List, ListItem, ListItemKey } from "./ui/list";

import React, { useContext } from "react";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "./ui/form";
import { useForm } from "react-hook-form";
import { Textarea } from "./ui/textarea";
import { usePostContractNegotiationRPCOffer } from "./../data/contract-mutations";
import {
  GlobalInfoContext,
  GlobalInfoContextType,
} from "./../context/GlobalInfoContext";

export const ContractNegotiationOfferDialog = ({
  process,
}: {
  process: CNProcess;
}) => {
  // --- Form Setup ---
  const form = useForm({
    defaultValues: {
      odrl: '{"@id":"urn:uuid:071c9c85-cddd-4cb8-9b9b-fcacf88d4687","@type":"Offer","permission":[{"action":"use","constraint":[{"leftOperand":"did:web:hola.es","operator":"eq","rightOperand":"user"}]}],"target":"urn:uuid:c9d4516d-dc86-4662-931b-12f92edfe94b"}',
    },
  });
  const { handleSubmit, control, setValue, getValues } = form;
  const { mutateAsync: dataOfferAsync } = usePostContractNegotiationRPCOffer();
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);
  const onSubmit = (data: any) => {
    console.log("Form submitted with data:", data);
    dataOfferAsync({
      api_gateway: api_gateway,
      content: {
        consumerParticipantId: process.associated_consumer,
        offer: JSON.parse(data.odrl),
        consumerPid: process.consumer_id,
        providerPid: process.provider_id,
      },
    });
  };

  const scopedListItemKeyClasses = "basis-[30%]";

  return (
    <DialogContent className="sm:max-w-[425px]">
      <DialogHeader>
        <DialogTitle>Contract Negotiation Offer</DialogTitle>
        <DialogDescription className="break-all">
          <p>Make changes on the Contract Negotiation Offer.</p>
          {/* <p>{JSON.stringify(process)}</p> */}
        </DialogDescription>
      </DialogHeader>
      {/* List */}
      <List className="min-w-fit w-full">
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
          <Badge variant={"info"}>
            {process.associated_consumer.slice(9, -1)}
          </Badge>
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
          {process.created_at}
        </ListItem>
        {process.updated_at && (
          <ListItem>
            <ListItemKey className={scopedListItemKeyClasses}>Updated at:</ListItemKey>
            {process.updated_at}
          </ListItem>
        )}
      </List>
      {/* / List content */}
      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          <FormField
            control={form.control}
            name="odrl"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Odrl</FormLabel>
                <FormControl>
                  <Textarea {...field} />
                </FormControl>
                <FormDescription>
                  Provide the ODRL policy content
                </FormDescription>
                <FormMessage />
              </FormItem>
            )}
          />
          <DialogFooter className="[&>*]:w-full">
            <DialogClose asChild>
              <Button variant="ghost" type="reset">
                Cancel
              </Button>
            </DialogClose>
            <Button type="submit" variant="outline">Offer</Button>
          </DialogFooter>
        </form>
      </Form>
    </DialogContent>
  );
};
