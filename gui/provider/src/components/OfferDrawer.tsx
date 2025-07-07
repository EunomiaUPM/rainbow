import { usePostContractNegotiationRPCOffer } from "shared/src/data/contract-mutations.ts";
import { useContext, useState } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "shared/src/context/GlobalInfoContext.tsx";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "shared/src/components/ui/form.tsx";
import { Popover, PopoverContent, PopoverTrigger } from "shared/src/components/ui/popover.tsx";
import { Button } from "shared/src/components/ui/button.tsx";
import { ChevronsUpDown } from "lucide-react";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "shared/src/components/ui/command.tsx";
import PolicyComponent from "shared/src/components/PolicyComponent.tsx";
import { SubmitHandler, useForm } from "react-hook-form";
import { getParticipants } from "shared/src/data/participant-queries.ts";
import { useGetPoliciesByDatasetId } from "shared/src/data/policy-queries.ts";
import { useGetDatahubDataset } from "../../../shared/src/data/datahub-catalog-queries.ts";
import { Badge } from "shared/src/components/ui/badge.tsx";
import { PolicyWrapperShow } from "shared/src/components/PolicyWrapperShow.tsx";

type Inputs = {
  consumerParticipantId: string;
  id: UUID; // This seems to be used for dataset ID, consider renaming for clarity
  catalog: UUID;
  target: UUID;
  odrl: string;
};

export const OfferDrawer = ({
  catalogId,
  datasetId,
  closeDrawer,
}: {
  catalogId: string;
  datasetId: string;
  closeDrawer: () => void;
}) => {
  const { mutateAsync: sendOfferAsync, isPending } = usePostContractNegotiationRPCOffer();
  const { data: policies } = useGetPoliciesByDatasetId(datasetId);
  const { data: datasetInfo } = useGetDatahubDataset(datasetId);

  // @ts-ignore
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);

  // --- State Management ---
  // Consumer Participant
  const [consumerParticipantOpen, setConsumerParticipantOpen] = useState(false);
  const [_consumerSelectedParticipant, setConsumerSelectedParticipant] =
    useState<Participant | null>(null);
  const [consumerParticipants, setConsumerParticipants] = useState<Participant[]>([]);
  const [selectedPolicy, setSelectedPolicy] = useState<OdrlOffer | null>(null);

  // --- Form Setup ---
  const form = useForm<Inputs>({
    defaultValues: {
      consumerParticipantId: "",
      id: datasetId, // Policy ID
      catalog: catalogId, // Catalog ID
      target: datasetId, // Dataset ID
    },
  });
  const { handleSubmit, control } = form;

  // --- Popover Open/Change Handlers ---
  const handleConsumerParticipantOpenChange = async (newOpenState: boolean) => {
    setConsumerParticipantOpen(newOpenState);
    if (newOpenState && consumerParticipants.length === 0) {
      try {
        const participants = await getParticipants(api_gateway);
        const participantsFiltered = participants.filter((p) => p.participant_type == "Consumer");
        setConsumerParticipants(participantsFiltered);
      } catch (error) {
        console.error("Failed to fetch participants:", error);
      }
    }
  };

  const onSubmit: SubmitHandler<Inputs> = async (data) => {
    console.log("Form data submitted:", data);
    const policy = policies.find((p) => p["@id"] == data.id)!;
    await sendOfferAsync(
      {
        content: {
          consumerParticipantId: data.consumerParticipantId,
          offer: {
            "@id": policy["@id"],
            "@type": "Offer",
            target: policy.target,
            //@ts-ignore
            permission:
              policy.permission == undefined || policy.permission.length == 0
                ? null
                : policy.permission,
            //@ts-ignore
            obligation:
              policy.obligation == undefined || policy.obligation.length == 0
                ? null
                : policy.obligation,
            //@ts-ignore
            prohibition:
              policy.prohibition == undefined || policy.prohibition.length == 0
                ? null
                : policy.prohibition,
            profile: policy.profile,
          },
        },
        api_gateway: api_gateway,
      },
      {},
    );
  };

  return (
    <div className="max-w-[500px] w-full px-6">
      {/* <Heading level="h3">New Contract Negotiation Offer</Heading> */}
      <div className=" flex gap-4">
        <div>
          <p className="text-sm text-gray-200 mb-1"> Selected catalog</p>

          <Badge variant="info">{catalogId.slice(14)}</Badge>
        </div>
        <div className="h-3"></div>
        <div>
          <p className="text-sm text-gray-200 mb-1"> Selected dataset</p>
          <Badge variant="info">{datasetInfo.name}</Badge>
        </div>
        {/* {JSON.stringify(datasetInfo)} */}
      </div>
      <div className="h-5"></div>
      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          {/* Consumer Participant Field */}
          <FormField
            control={control}
            name="consumerParticipantId"
            render={({ field }) => (
              <FormItem>
                <div>
                  <FormLabel className="text-gray-200 font-normal">
                    Consumer Participant Id
                  </FormLabel>
                  <p className="text-xs text-gray-500 mb-1">
                    Select the ID of the consumer participant for the negotiation.
                  </p>
                </div>
                <div>
                  <FormControl>
                    <Popover
                      open={consumerParticipantOpen}
                      onOpenChange={handleConsumerParticipantOpenChange}
                    >
                      <PopoverTrigger asChild>
                        <Button
                          variant="outline"
                          role="combobox"
                          aria-expanded={consumerParticipantOpen}
                          className="w-full justify-between font-normal text-gray-300  transition-colors"
                        >
                          {field.value
                            ? consumerParticipants
                                .find((p) => p.participant_id === field.value)
                                ?.participant_id.slice(4, 32) + "[...]"
                            : "Select participant..."}
                          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-80" />
                        </Button>
                      </PopoverTrigger>
                      <PopoverContent className="w-[--radix-popover-trigger-width] bg-background p-0">
                        <Command>
                          <CommandInput placeholder="Search participant..." />
                          <CommandList>
                            <CommandEmpty>No participants found.</CommandEmpty>
                            <CommandGroup>
                              {consumerParticipants.map((consumerParticipant) => (
                                <CommandItem
                                  key={consumerParticipant.participant_id}
                                  value={consumerParticipant.participant_id}
                                  onSelect={() => {
                                    field.onChange(consumerParticipant.participant_id);
                                    setConsumerSelectedParticipant(consumerParticipant);
                                    setConsumerParticipantOpen(false);
                                    // No fields follow this one that need clearing based on its change
                                  }}
                                  className={
                                    field.value === consumerParticipant.participant_id
                                      ? "text-role-consumer font-medium"
                                      : ""
                                  }
                                >
                                  {consumerParticipant.participant_id.slice(4, 32) + "[...]"}
                                  <span className="text-gray-400 ml-2 text-sm">
                                    ({consumerParticipant.base_url})
                                  </span>
                                </CommandItem>
                              ))}
                            </CommandGroup>
                          </CommandList>
                        </Command>
                      </PopoverContent>
                    </Popover>
                  </FormControl>

                  <FormMessage />
                </div>
              </FormItem>
            )}
          />
          <div>
            <p className="text-sm text-gray-200 "> Policies</p>
            <p className="text-xs text-gray-500">
              Select the policy from the dataset that best matches your needs.
            </p>
            <div className="flex flex-wrap gap-4 mt-3 mb-24">
              {policies &&
                policies.map((policy) => (
                  <div
                    className={
                      selectedPolicy === policy
                        ? ` outline outline-offset-2 outline-secondary-500 rounded-[6px] `
                        : "opacity-70"
                    }
                    onClick={() => {
                      setSelectedPolicy(policy);
                      form.setValue("id", policy["@id"]);
                    }}
                  >
                    <PolicyWrapperShow policy={policy} />
                  </div>
                ))}
            </div>
          </div>
          <div
            className="flex gap-2 border-t fixed bottom-0  p-6 w-full bg-background border-white/30"
            style={{ marginLeft: -48 }}
          >
            <Button type="submit" disabled={isPending} className="w-48">
              Submit Offer {isPending && <span className="ml-2">...</span>}
            </Button>
            <Button variant="ghost" onClick={closeDrawer}>
              Cancel
            </Button>
          </div>
        </form>
      </Form>
    </div>
  );
};
