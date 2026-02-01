import { createFileRoute } from "@tanstack/react-router";
import {
  Form,
} from "shared/src/components/ui/form";
import { SubmitHandler, useForm } from "react-hook-form";
import { Button } from "shared/src/components/ui/button.tsx";
import { useContext, useEffect, useState } from "react"; // Import useEffect
import { ConsumerParticipantSelector } from "../../components/form/ConsumerParticipantSelector";
import { getParticipants } from "shared/src/data/participant-queries.ts";
import { getPoliciesByDatasetId } from "shared/src/data/policy-queries.ts";
import { usePostContractNegotiationRPCOffer } from "shared/src/data/contract-mutations.ts";
import { GlobalInfoContext, GlobalInfoContextType } from "shared/src/context/GlobalInfoContext.tsx";
import { Badge } from "shared/src/components/ui/badge";
import PolicyComponent from "shared/src/components/PolicyComponent.tsx";
import { formatUrn } from "shared/src/lib/utils.ts";

type Inputs = {
  consumerParticipantId: UUID;
  id: UUID; // This seems to be used for dataset ID, consider renaming for clarity
  catalog: UUID;
  target: UUID;
  odrl: string;
};

/**
 * Component for creating a new contract negotiation offer.
 */
export const RouteComponent = ({ catalog, dataset }: { catalog: Catalog, dataset: Dataset }) => {
  const { mutateAsync: sendOfferAsync, isPending } = usePostContractNegotiationRPCOffer();
  const { api_gateway } = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;

  // --- State Management ---
  // Consumer Participant
  const [consumerParticipantOpen, setConsumerParticipantOpen] = useState(false);
  const [_consumerSelectedParticipant, setConsumerSelectedParticipant] =
    useState<Participant | null>(null);
  const [consumerParticipants, setConsumerParticipants] = useState<Participant[]>([]);

  // Catalog
  const [_catalogOpen, setCatalogOpen] = useState(false);
  const [_selectedCatalog, setSelectedCatalog] = useState(catalog || null); // Initialize with catalog prop
  const [catalogs, _setCatalogs] = useState<Catalog[]>([]);

  // Dataset (used for 'id' field in Inputs)
  const [_datasetOpen, setDatasetOpen] = useState(false);
  const [selectedDataset, setSelectedDataset] = useState(dataset || null);
  const [datasets, _setDatasets] = useState<Dataset[]>([] /* Initial empty array */);

  // Policies (used for 'target' field in Inputs)
  const [_policiesOpen, setPoliciesOpen] = useState(false);
  const [selectedPolicy, setSelectedPolicy] = useState<OdrlOffer | null>(null);
  const [policies, setPolicies] = useState<OdrlOffer[]>([]);

  // --- Form Setup ---
  const form = useForm<Inputs>({
    defaultValues: {
      consumerParticipantId: "",
      id: dataset["@id"], // Dataset ID
      catalog: catalog["@id"], // Catalog ID
      target: "", // Policy ID
    },
  });

  // Destructure form methods for easier use
  const { handleSubmit, control, setValue, getValues } = form;

  // --- Effects for Initializing Selected Values ---
  // Initialize consumerSelectedParticipant if default value exists
  useEffect(() => {
    const defaultId = getValues("consumerParticipantId");
    if (defaultId && consumerParticipants.length > 0) {
      const participant = consumerParticipants.find((p) => p.participant_id === defaultId);
      if (participant) {
        setConsumerSelectedParticipant(participant);
      }
    }
  }, [getValues, consumerParticipants]); // Add getValues to dependency array

  // Initialize selectedCatalog if default value exists
  useEffect(() => {
    setSelectedCatalog(catalog);
  }, [getValues, catalogs, catalog]);

  // Initialize selectedDataset if default value exists
  useEffect(() => {
    setSelectedDataset(dataset);
    form.setValue("target", dataset["@id"]);
  }, [getValues, datasets, dataset, form, selectedDataset]);

  // Initialize selectedPolicy if default value exists
  useEffect(() => {
    const defaultId = ""; // 'target' field is for policy

    if (defaultId && policies.length > 0) {
      const policy = policies.find((p) => p["@id"] === defaultId);
      if (policy) {
        setSelectedPolicy(policy);
      }
    }
  }, [getValues, policies]);

  // Load policies on mount
  useEffect(() => {
    const loadPolicies = async () => {
      if (selectedDataset) {
        try {
          const fetchedPolicies = await getPoliciesByDatasetId(api_gateway, selectedDataset["@id"]);
          setPolicies(fetchedPolicies);
        } catch (error) {
          console.error("Failed to fetch policies:", error);
        }
      }
    };
    loadPolicies();
  }, [selectedDataset, api_gateway]);

  // --- Helper to Clear Subsequent Fields ---
  const _clearFields = (fieldsToClear: Array<keyof Inputs>) => {
    fieldsToClear.forEach((fieldName) => {
      setValue(fieldName, "", { shouldValidate: true }); // Clear form value
      // Clear associated local states
      if (fieldName === "catalog") {
        // setSelectedCatalog(null);
        // setCatalogs([]); // Clear catalog options if needed
      } else if (fieldName === "target") {
        // Dataset field
        // setSelectedDataset(null);
        // setDatasets([]); // Clear dataset options
      } else if (fieldName === "id") {
        // Policy field
        setSelectedPolicy(null);
        setPolicies([]); // Clear policy options
      }
      // Add more conditions for other fields if they have specific local states
    });
  };

  // --- Form Submission Handler ---
  const onSubmit: SubmitHandler<Inputs> = async (data) => {
    await sendOfferAsync(
      {
        //@ts-ignore
        content: {
          consumerParticipantId: data.consumerParticipantId,
          // @ts-ignore - Simplified offer DTO for API
          offer: {
            "@id": data.id,
            "@type": "Offer",
            target: data.target,
            permission: [
              {
                action: "use",
                constraint: [],
              },
            ],
            obligation: undefined,
            prohibition: undefined,
            profile: "",
          },
        },
        api_gateway: api_gateway,
      },
      {},
    );

    form.reset();
    // Reset all local states when the form is fully reset
    setConsumerSelectedParticipant(null);
    setConsumerParticipantOpen(false);
    // setSelectedCatalog(null);
    setCatalogOpen(false);
    // setSelectedDataset(null);
    setDatasetOpen(false);
    setSelectedPolicy(null);
    setPoliciesOpen(false);
  };

  // --- Popover Open/Change Handlers ---
  const handleConsumerParticipantOpenChange = async (newOpenState: boolean) => {
    setConsumerParticipantOpen(newOpenState);
    if (newOpenState && consumerParticipants.length === 0) {
      try {
        const participants = await getParticipants(api_gateway);
        setConsumerParticipants(participants);
      } catch (error) {
        console.error("Failed to fetch participants:", error);
      }
    }
  };


  return (
    <div className="max-w-[500px] w-full m-auto">
      {/* <Heading level="h3">New Contract Negotiation Offer</Heading> */}
      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
          {/* Catalog Field */}
          <div>
            {" "}
            <p>{catalog.title}</p>{" "}
            <Badge variant="info">{formatUrn(catalog["@id"])}</Badge>
          </div>



          {/* Dataset Field (mapped to 'id' in Inputs) */}

          <div>
            Chosen dataset: {dataset.title} {dataset["@id"]}
          </div>
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
          <div> POLICIES</div>
          {policies &&
            policies.map((policy) => (
              <div
                className={selectedPolicy === policy ? `border-white border-2` : ""}
                onClick={() => {
                  setSelectedPolicy(policy);
                  form.setValue("id", policy["@id"]);
                }}
              >
                <PolicyComponent policyItem={policy.permission ?? null} variant={"permission"} />

                <PolicyComponent policyItem={policy.obligation ?? null} variant={"obligation"} />

                <PolicyComponent policyItem={policy.prohibition ?? null} variant={"prohibition"} />
              </div>
            ))}
          {/* Policy Target Field was here - removed commented code */}


          <Button type="submit" disabled={isPending} className="w-full">
            Submit Offer {isPending && <span className="ml-2">...</span>}
          </Button>
        </form>
      </Form>
    </div>
  );
};

/**
 * Route for creating a contract negotiation offer.
 */
export const Route = createFileRoute("/contract-negotiation/offer")({
  component: RouteComponent,
  pendingComponent: () => <div className="p-4 text-center text-gray-600">Loading...</div>,
});
