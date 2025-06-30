import { createFileRoute } from "@tanstack/react-router";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "shared/src/components/ui/form";
import { set, SubmitHandler, useForm } from "react-hook-form";
import { Button } from "shared/src/components/ui/button.tsx";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "shared/src/components/ui/command";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "shared/src/components/ui/popover";
import { ChevronsUpDown } from "lucide-react";
import { useContext, useEffect, useState } from "react"; // Import useEffect
import { getParticipants } from "shared/src/data/participant-queries.ts";
import {
  getCatalogs,
  getDatasetsByCatalogId,
} from "shared/src/data/catalog-queries.ts";
import { getPoliciesByDatasetId } from "shared/src/data/policy-queries.ts";
import { Textarea } from "shared/src/components/ui/textarea.tsx";
import { usePostContractNegotiationRPCOffer } from "shared/src/data/contract-mutations.ts";
import {
  GlobalInfoContext,
  GlobalInfoContextType,
} from "shared/src/context/GlobalInfoContext.tsx";
import Heading from "../../../../shared/src/components/ui/heading.tsx";
import { Badge } from "shared/src/components/ui/badge";
import PolicyComponent from "shared/src/components/ui/policyComponent.tsx";

type Inputs = {
  consumerParticipantId: UUID;
  id: UUID; // This seems to be used for dataset ID, consider renaming for clarity
  catalog: UUID;
  target: UUID;
  odrl: string;
};

export const RouteComponent = ({ cnProcess, catalog, dataset }) => {
  const { mutateAsync: sendOfferAsync, isPending } =
    usePostContractNegotiationRPCOffer();
  // @ts-ignore
  const { api_gateway } = useContext<GlobalInfoContextType>(GlobalInfoContext);

  // --- State Management ---
  // Consumer Participant
  const [consumerParticipantOpen, setConsumerParticipantOpen] = useState(false);
  const [_consumerSelectedParticipant, setConsumerSelectedParticipant] =
    useState<Participant | null>(null);
  const [consumerParticipants, setConsumerParticipants] = useState<
    Participant[]
  >([]);

  // Catalog
  const [catalogOpen, setCatalogOpen] = useState(false);
  const [selectedCatalog, setSelectedCatalog] = useState(catalog || null); // Initialize with catalog prop
  const [catalogs, setCatalogs] = useState<Catalog[]>([]);

  // Dataset (used for 'id' field in Inputs)
  const [datasetOpen, setDatasetOpen] = useState(false);
  const [selectedDataset, setSelectedDataset] =  useState(dataset || null);
  const [datasets, setDatasets] = useState<Dataset[]>(
    [] /* Initial empty array */
  );

  // Policies (used for 'target' field in Inputs)
  const [policiesOpen, setPoliciesOpen] = useState(false);
  const [selectedPolicy, setSelectedPolicy] = useState<OdrlOffer | null>(null);
  const [policies, setPolicies] = useState<OdrlOffer[]>([]);

  // --- Form Setup ---
  const form = useForm<Inputs>({
    defaultValues: {
      consumerParticipantId: "",
      // id: dataset?.["@id"], // Dataset ID
      // catalog: catalog?.["@id"], // Catalog ID
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
      const participant = consumerParticipants.find(
        (p) => p.participant_id === defaultId
      );
      if (participant) {
        setConsumerSelectedParticipant(participant);
      }
    }
  }, [getValues, consumerParticipants]); // Add getValues to dependency array

  // Initialize selectedCatalog if default value exists
  useEffect(() => {
    const defaultId = catalog?.["@id"];
    // console.log(defaultId, "defaultId in offer form");
    // setSelectedCatalog(catalog);
    // console.log(selectedCatalog, " selectedCatalog__");
    /// !!!!! modificacion 1
    // if (defaultId && catalogs.length > 0) {
    //   const catalog = catalogs.find((c) => c["@id"] === defaultId);
    //   if (catalog) {
    //     setSelectedCatalog(catalog);
    //   }
    // }
  }, [getValues, catalogs]);

  // Initialize selectedDataset if default value exists
  useEffect(() => {
    const defaultId = dataset?.["@id"];
    console.log(defaultId, "defaultId dataset in offer form");
    setSelectedDataset(dataset);
    form.setValue("target", dataset?.["@id"]);
    console.log(selectedDataset, " selectedDataset__");
    // !!!!! modificacion 3
    // 'id' field is for dataset
    // if (defaultId && datasets.length > 0) {
    //   const dataset = datasets.find((d) => d["@id"] === defaultId);
    //   if (dataset) {
    //     setSelectedDataset(dataset);
    //   }
    // }
  }, [getValues, datasets]);

  // Initialize selectedPolicy if default value exists
  useEffect(() => {
    const defaultId = "" ; // 'target' field is for policy

    if (defaultId && policies.length > 0) {
      const policy = policies.find((p) => p?.["@id"] === defaultId);
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
          const fetchedPolicies = await getPoliciesByDatasetId(
            api_gateway,
            selectedDataset?.["@id"]
          );
          setPolicies(fetchedPolicies);
        } catch (error) {
          console.error("Failed to fetch policies:", error);
        }
      }
    };
    loadPolicies();
  }, [selectedDataset]);

  // --- Helper to Clear Subsequent Fields ---
  const clearFields = (fieldsToClear: Array<keyof Inputs>) => {
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
    console.log("Form data submitted:", data);
    await sendOfferAsync(
      {
        content: {
          consumerParticipantId: data.consumerParticipantId,
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
            // @ts-ignore
            obligation: null,
            // @ts-ignore
            prohibition: null,
            profile: "",
          },
        },
        api_gateway: api_gateway,
      },
      {}
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

  const handleCatalogOpenChange = async (newOpenState: boolean) => {
    setCatalogOpen(newOpenState);
    if (newOpenState) {
      try {
        const fetchedCatalogs = await getCatalogs(api_gateway);
        setCatalogs(fetchedCatalogs.catalog);
      } catch (error) {
        console.error("Failed to fetch catalogs:", error);
      }
    }
  };

  const handleDatasetOpenChange = async (newOpenState: boolean) => {
    setDatasetOpen(newOpenState);
    if (newOpenState && selectedCatalog) {
      // Only fetch if a catalog is selected
      try {
        const fetchedDatasets = await getDatasetsByCatalogId(
          api_gateway,
          selectedCatalog?.["@id"]
        );
        setDatasets(fetchedDatasets);
      } catch (error) {
        console.error("Failed to fetch datasets:", error);
      }
    } else if (newOpenState && !selectedCatalog) {
      // Optionally, show a warning or prevent opening if no catalog is selected
      console.warn("Please select a catalog first.");
      setDatasetOpen(false); // Prevent popover from opening
    }
  };

  const handlePoliciesOpenChange = async (newOpenState: boolean) => {
    setPoliciesOpen(newOpenState);
    if (newOpenState && selectedDataset) {
        // if (newOpenState && selectedDataset) {
      // Only fetch if a dataset is selected
      try {
        const fetchedPolicies = await getPoliciesByDatasetId(
          api_gateway,
          selectedDataset["@id"]
        );
        setPolicies(fetchedPolicies);
      } catch (error) {
        console.error("Failed to fetch policies:", error);
      }
    } else if (newOpenState && !selectedDataset) {
      console.warn("Please select a dataset first.");
      setPoliciesOpen(false);
    }
  };

  return (
     <div className="flex gap-4 justify-start items-start">
    <div className=" w-full m-auto">
      <Heading level="h3">CounterOffer</Heading>
      <Form {...form}>
        <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
             

          <div> POLICIES</div>
          {console.log(policies, "policiessss")}
          {policies.map((policy) => (
            
            <div className={selectedPolicy === policy  ? `border-white border-2` : ""}
              onClick={() => {
                setSelectedPolicy(policy);
                 form.setValue("id", policy?.["@id"])
                }}
                >
              <PolicyComponent
                policyItem={policy.permission}
                variant={"permission"}
            
              />
              
              <PolicyComponent
                policyItem={policy.obligation}
                variant={"obligation"}
              />

              <PolicyComponent
                policyItem={policy.prohibition}
                variant={"prohibition"}
              />
            
            </div>
          ))}

          <FormField
            control={form.control}
            name="odrl"
            disabled={!selectedPolicy}
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

          <Button type="submit" disabled={isPending} className="w-full">
            Submit Offer {isPending && <span className="ml-2">...</span>}
          </Button>
        </form>
      </Form>
    </div>
      <div className=" w-full m-auto">
          <Heading level="h3">CounterOffer</Heading>
      </div>
     </div>
  );
};

export const Route = createFileRoute("/contract-negotiation/offer copy")({
  component: RouteComponent,
  pendingComponent: () => (
    <div className="p-4 text-center text-gray-600">Loading...</div>
  ),
});
