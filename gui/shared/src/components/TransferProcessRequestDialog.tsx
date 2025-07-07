import {
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "shared/src/components/ui/dialog";
import { Badge } from "shared/src/components/ui/badge";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "shared/src/components/ui/select";
import { Button } from "shared/src/components/ui/button";
import React, { useContext } from "react";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormMessage,
} from "shared/src/components/ui/form";
import { useForm } from "react-hook-form";
import { usePostTransferRPCRequest } from "@/data/transfer-mutations.ts";
import { GlobalInfoContext, GlobalInfoContextType } from "@/context/GlobalInfoContext.tsx";

type Inputs = {
  method: "PULL" | "PUSH";
  transfer_protocol: string;
};

const transferMethods = ["PULL", "PUSH"];
const transferProtocols = ["http", "kafka", "ftp"];

export const TransferProcessRequestDialog = ({ agreement }: { agreement: Agreement }) => {
  const { mutateAsync: requestAsync } = usePostTransferRPCRequest();
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const form = useForm<Inputs>({
    defaultValues: {
      method: "PULL",
      transfer_protocol: "http",
    },
  });
  const onSubmit = async (data: Inputs) => {
    await requestAsync({
      api_gateway,
      content: {
        providerParticipantId: agreement.provider_participant_id,
        agreementId: agreement.agreement_id,
        format: `${data.transfer_protocol}+${data.method}`.toLowerCase(),
      },
    });
  };

  return (
    <DialogContent>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className="flex flex-col gap-4">
          <DialogHeader>
            <DialogTitle>Transfer request dialog</DialogTitle>
            <DialogDescription className="max-w-full flex flex-wrap break-all">
              <span className="max-w-full flex flex-wrap">
                Select the transference's parameters for
                <Badge variant="info">{agreement.agreement_id.slice(9, 30) + "[...]"}</Badge>
              </span>
            </DialogDescription>
          </DialogHeader>
          <div className="grid grid-cols-2 gap-4">
            <FormField
              control={form.control}
              name="method"
              render={({ field }) => (
                <FormItem>
                  <label htmlFor="method" className="text-sm mb-1 text-inherit">
                    Method
                  </label>
                  <FormControl>
                    <Select
                      {...field}
                      defaultValue={transferMethods[0]}
                      onValueChange={field.onChange}
                    >
                      <SelectTrigger className="w-[240px]">
                        <SelectValue placeholder="Select a transfer protocol" />
                      </SelectTrigger>
                      <SelectContent>
                        {transferMethods.map((transferMethod) => (
                          <SelectItem value={transferMethod} key={transferMethod}>
                            {transferMethod}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </FormControl>
                  <FormDescription>This is your public display name.</FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="transfer_protocol"
              render={({ field }) => (
                <FormItem>
                  <label htmlFor="transfer_protocol" className="text-sm mb-1">
                    Transfer protocol
                  </label>
                  <FormControl>
                    <Select
                      {...field}
                      defaultValue={transferProtocols[0]}
                      onValueChange={field.onChange}
                    >
                      <SelectTrigger className="w-[240px]">
                        <SelectValue placeholder="Select a transfer protocol" />
                      </SelectTrigger>
                      <SelectContent>
                        {transferProtocols.map((transferProtocol) => (
                          <SelectItem value={transferProtocol} key={transferProtocol}>
                            {transferProtocol}
                          </SelectItem>
                        ))}
                      </SelectContent>
                    </Select>
                  </FormControl>
                  <FormDescription>This is your public display name.</FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
          </div>
          <DialogFooter className="[&>*]:w-full">
            <DialogClose asChild>
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
