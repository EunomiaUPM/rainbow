import { usePostContractNegotiationRPCOffer } from "shared/src/data/contract-mutations.ts";
import { Dispatch, SetStateAction, useContext, useState } from "react";
import { GlobalInfoContext, GlobalInfoContextType } from "shared/src/context/GlobalInfoContext.tsx";
import { Form } from "shared/src/components/ui/form.tsx";
import { Button } from "shared/src/components/ui/button.tsx";
import { ConsumerParticipantSelector } from "../routes/contract-negotiation/../../components/form/ConsumerParticipantSelector";
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

/**
 * Drawer component for creating and sending a contract offer.
 */
export const OfferDrawer = ({
  catalogId,
  datasetId,
  closeDrawer,
}: {
  catalogId: string;
  datasetId: string;
  closeDrawer: Dispatch<SetStateAction<boolean>>;
}) => {
  const { mutateAsync: sendOfferAsync, isPending } = usePostContractNegotiationRPCOffer();
  const { data: policies } = useGetPoliciesByDatasetId(datasetId);
  const { data: datasetInfo } = useGetDatahubDataset(datasetId);

  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;

  // --- State Management ---
  // Consumer Participant
  const [consumerParticipantOpen, setConsumerParticipantOpen] = useState(false);
  const [_, setConsumerSelectedParticipant] = useState<Participant | null>(null);
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
    const policy = policies.find((p) => p["@id"] == data.id)!;
    await sendOfferAsync({
      // @ts-ignore - DTO does not match full OdrlOffer type
      content: {
        consumerParticipantId: data.consumerParticipantId,
        // @ts-ignore - Simplified offer DTO for API
        offer: {
          "@id": policy["@id"],
          "@type": "Offer",
          target: policy.target,
          permission:
            policy.permission == undefined || policy.permission.length == 0
              ? undefined
              : policy.permission,
          obligation:
            policy.obligation == undefined || policy.obligation.length == 0
              ? undefined
              : policy.obligation,
          prohibition:
            policy.prohibition == undefined || policy.prohibition.length == 0
              ? undefined
              : policy.prohibition,
          profile: policy.profile,
        },
      },
      api_gateway: api_gateway,
    });
    closeDrawer(true);
  };

  return (
    <div className="max-w-[500px] w-full px-6">
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
      </div>
      <div className="h-5"></div>
      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          {/* Consumer Participant Field */}
          <ConsumerParticipantSelector
            control={control}
            name="consumerParticipantId"
            participants={consumerParticipants}
            isOpen={consumerParticipantOpen}
            onOpenChange={handleConsumerParticipantOpenChange}
            onSelect={(participant) => {
              setConsumerSelectedParticipant(participant);
              setConsumerParticipantOpen(false);
            }}
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
                    key={policy["@id"]}
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
                    <PolicyWrapperShow
                      policy={policy}
                      datasetId={datasetId}
                      catalogId={catalogId}
                      datasetName={datasetInfo?.name || ""}
                      participant={undefined}
                    />
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
            <Button type="reset" variant="ghost" onClick={() => closeDrawer(true)}>
              Cancel
            </Button>
          </div>
        </form>
      </Form>
    </div>
  );
};
