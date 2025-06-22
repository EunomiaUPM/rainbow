import {createFileRoute} from "@tanstack/react-router";
import {
    Form,
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "shared/src/components/ui/form"
import {SubmitHandler, useForm} from "react-hook-form";
import {Button} from "shared/src/components/ui/button.tsx";
import {
    Command,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
} from "shared/src/components/ui/command"
import {Popover, PopoverContent, PopoverTrigger,} from "shared/src/components/ui/popover"
import {ChevronsUpDown} from "lucide-react";
import {useContext, useEffect, useState} from "react"; // Import useEffect
import {getParticipants} from "shared/src/data/participant-queries.ts";
import {getCatalogs, getDatasetsByCatalogId} from "shared/src/data/catalog-queries.ts";
import {getPoliciesByDatasetId} from "shared/src/data/policy-queries.ts";
import {Textarea} from "shared/src/components/ui/textarea.tsx";
import {usePostContractNegotiationRPCOffer} from "shared/src/data/contract-mutations.ts";
import {GlobalInfoContext, GlobalInfoContextType} from "shared/src/context/GlobalInfoContext.tsx";
import Heading from "../../../../shared/src/components/ui/heading.tsx";
import {getDatahubCatalogs, getDatahubDatasetsByCatalogId} from "shared/src/data/datahub-catalog-queries.ts";

type Inputs = {
  consumerParticipantId: UUID;
  id: UUID; // This seems to be used for dataset ID, consider renaming for clarity
  catalog: UUID;
  target: UUID;
  odrl: string;
};

const RouteComponent = () => {
    const {mutateAsync: sendOfferAsync, isPending} = usePostContractNegotiationRPCOffer()
    // @ts-ignore
    const {api_gateway, catalog_type} = useContext<GlobalInfoContextType>(GlobalInfoContext)

    // --- State Management ---
    // Consumer Participant
    const [consumerParticipantOpen, setConsumerParticipantOpen] = useState(false);
    const [_consumerSelectedParticipant, setConsumerSelectedParticipant] = useState<Participant | null>(null);
    const [consumerParticipants, setConsumerParticipants] = useState<Participant[]>([]);

    // Catalog
    const [catalogOpen, setCatalogOpen] = useState(false);
    const [selectedCatalog, setSelectedCatalog] = useState<Catalog | DatahubDomain | null>(null);
    const [catalogs, setCatalogs] = useState<Catalog[] | DatahubDomain[]>([]);

    // Dataset (used for 'id' field in Inputs)
    const [datasetOpen, setDatasetOpen] = useState(false);
    const [selectedDataset, setSelectedDataset] = useState<Dataset | DatahubDataset | null>(null);
    const [datasets, setDatasets] = useState<Dataset[] | DatahubDataset[]>([] /* Initial empty array */);

    // Policies (used for 'target' field in Inputs)
    const [policiesOpen, setPoliciesOpen] = useState(false);
    const [selectedPolicy, setSelectedPolicy] = useState<OdrlOffer | null>(null);
    const [policies, setPolicies] = useState<OdrlOffer[]>([]);

    // --- Form Setup ---
    const form = useForm<Inputs>({
        defaultValues: {
            consumerParticipantId: "",
            id: "", // Dataset ID
            catalog: "",
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
            const participant = consumerParticipants.find(p => p.participant_id === defaultId);
            if (participant) {
                setConsumerSelectedParticipant(participant);
            }
        }
    }, [getValues, consumerParticipants]); // Add getValues to dependency array

  // Initialize selectedCatalog if default value exists
  useEffect(() => {
    const defaultId = getValues("catalog");
    // catalog rainbow
        if (catalog_type === "rainbow") {
            if (defaultId && catalogs.length > 0) {
                const catalog = catalogs.find(
                    (c) => (c as Catalog)["@id"] === defaultId
                );
                if (catalog) {
                    setSelectedCatalog(catalog);
                }
            }
        }
        // catalog datahub
        if (catalog_type === "datahub") {
            if (defaultId && catalogs.length > 0) {
                const catalog = catalogs.find(
                    c => (c as DatahubDomain).urn === defaultId
                );
                if (catalog) {
                    setSelectedCatalog(catalog);
                }
      }
    }
  }, [getValues, catalogs]);

  // Initialize selectedDataset if default value exists
  useEffect(() => {
    const defaultId = getValues("id"); // 'id' field is for dataset
    // catalog rainbow
        if (catalog_type === "rainbow") {
            if (defaultId && datasets.length > 0) {
                const dataset = datasets.find(
                    (d) => (d as Dataset)["@id"] === defaultId
                );
                if (dataset) {
                    setSelectedDataset(dataset);
                }
            }
        }
        if (catalog_type === "datahub") {
            if (defaultId && datasets.length > 0) {
                const dataset = datasets.find(
                    d => (d as DatahubDataset).urn === defaultId
                );
                if (dataset) {
                    setSelectedDataset(dataset);
                }
      }
    }
  }, [getValues, datasets]);

    // Initialize selectedPolicy if default value exists
    useEffect(() => {
        const defaultId = getValues("target"); // 'target' field is for policy
        if (defaultId && policies.length > 0) {
            const policy = policies.find(p => p["@id"] === defaultId);
            if (policy) {
                setSelectedPolicy(policy);
            }
        }
    }, [getValues, policies]);


    // --- Helper to Clear Subsequent Fields ---
    const clearFields = (fieldsToClear: Array<keyof Inputs>) => {
        fieldsToClear.forEach(fieldName => {
            setValue(fieldName, "", {shouldValidate: true}); // Clear form value
            // Clear associated local states
            if (fieldName === "catalog") {
                setSelectedCatalog(null);
                setCatalogs([]); // Clear catalog options if needed
            } else if (fieldName === "target") { // Dataset field
                setSelectedDataset(null);
                setDatasets([]); // Clear dataset options
            } else if (fieldName === "id") { // Policy field
                setSelectedPolicy(null);
                setPolicies([]); // Clear policy options
            }
            // Add more conditions for other fields if they have specific local states
        });
    };

    // --- Form Submission Handler ---
    const onSubmit: SubmitHandler<Inputs> = async data => {
        console.log("Form data submitted:", data);
        await sendOfferAsync({
            content: {
                consumerParticipantId: data.consumerParticipantId,
                offer: {
                    "@id": data.id,
                    "@type": "Offer",
                    target: data.target,
                    permission: [
                        {
                            action: "use",
                            constraint: []
                        }
                    ],
                    // @ts-ignore
                    obligation: null,
                    // @ts-ignore
                    prohibition: null,
                    profile: ""
                }
            },
            api_gateway: api_gateway,
        }, {})

        form.reset();
        // Reset all local states when the form is fully reset
        setConsumerSelectedParticipant(null);
        setConsumerParticipantOpen(false);
        setSelectedCatalog(null);
        setCatalogOpen(false);
        setSelectedDataset(null);
        setDatasetOpen(false);
        setSelectedPolicy(null);
        setPoliciesOpen(false);
    }

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
    }
  };

  const handleCatalogOpenChange = async (newOpenState: boolean) => {
    setCatalogOpen(newOpenState);
    if (newOpenState) {
      try {
        if (catalog_type === "rainbow") {
                    const fetchedCatalogs = await getCatalogs(api_gateway);
                    setCatalogs(fetchedCatalogs.catalog);
                }
                if (catalog_type === "datahub") {
                    const fetchedCatalogs = await getDatahubCatalogs(api_gateway);
                    setCatalogs(fetchedCatalogs);
                }
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
        if (catalog_type === "rainbow") {
                    const fetchedDatasets = await getDatasetsByCatalogId(api_gateway, (selectedCatalog as Catalog)["@id"]
            );        setDatasets(fetchedDatasets);
                }
                if (catalog_type === "datahub") {
                    const fetchedDatasets = await getDatahubDatasetsByCatalogId(api_gateway, (selectedCatalog as DatahubDomain).urn);
                    setDatasets(fetchedDatasets);
                }

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
        if (newOpenState && selectedDataset) { // Only fetch if a dataset is selected
            try {
                if (catalog_type === "rainbow") {
                    const fetchedPolicies = await getPoliciesByDatasetId(api_gateway, (selectedCatalog as Catalog)["@id"]);
                    setPolicies(fetchedPolicies);
                }
                if (catalog_type === "datahub") {
                    const fetchedPolicies = await getPoliciesByDatasetId(api_gateway, (selectedDataset as DatahubDataset).urn);
                    setPolicies(fetchedPolicies);
                }
            } catch (error) {
                console.error("Failed to fetch policies:", error);
            }
        } else if (newOpenState && !selectedDataset) {
            console.warn("Please select a dataset first.");
            setPoliciesOpen(false);
        }
    }
  };

    return (
        <div className="w-[500px]">
            <Heading level="h3">New Contract Negotiation Offer</Heading>
            <Form {...form}>
                <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">

                    {/* Consumer Participant Field */}
                    <FormField
                        control={control}
                        name="consumerParticipantId"
                        render={({field}) => (
                            <FormItem>
                                <FormLabel>Consumer Participant Id</FormLabel>
                                <div>
                                    <FormControl>
                                        <Popover open={consumerParticipantOpen}
                                                 onOpenChange={handleConsumerParticipantOpenChange}>
                                            <PopoverTrigger asChild>
                                                <Button
                                                    variant="outline"
                                                    role="combobox"
                                                    aria-expanded={consumerParticipantOpen}
                                                    className="w-full justify-between font-normal text-gray-300  transition-colors"
                                                >
                                                    {field.value
                                                        ? consumerParticipants.find((p) => p.participant_id === field.value)?.participant_id
                                                        : "Select participant..."}
                                                    <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-80"/>
                                                </Button>
                                            </PopoverTrigger>
                                            <PopoverContent className="w-[--radix-popover-trigger-width] p-0">
                                                <Command>
                                                    <CommandInput placeholder="Search participant..."/>
                                                    <CommandList>
                                                        <CommandEmpty>No participant found.</CommandEmpty>
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
                                                                    className={field.value === consumerParticipant.participant_id ? "text-blue-300 font-medium" : ""}
                                                                >
                                                                    {consumerParticipant.participant_id}
                                                                    <span
                                                                        className="text-gray-400 ml-2 text-sm">({consumerParticipant.base_url})</span>
                                                                </CommandItem>
                                                            ))}
                                                        </CommandGroup>
                                                    </CommandList>
                                                </Command>
                                            </PopoverContent>
                                        </Popover>
                                    </FormControl>
                                    <FormDescription className="text-sm text-gray-400 mt-1">Provide the ID of the
                                        consumer participant for the negotiation.</FormDescription>
                                    <FormMessage/>
                                </div>
                            </FormItem>
                        )}
                    />

                    {/* Catalog Field */}
                    <FormField
                        control={control}
                        name="catalog"
                        render={({field}) => (
                            <FormItem>
                                <FormLabel>Catalog</FormLabel>
                                <div>
                                    <FormControl>
                                        <Popover open={catalogOpen} onOpenChange={handleCatalogOpenChange}>
                                            <PopoverTrigger asChild>
                                                <Button
                                                    variant="outline"
                                                    role="combobox"
                                                    aria-expanded={catalogOpen}
                                                    className="w-full justify-between font-normal text-gray-600 hover:text-gray-800 transition-colors"
                                                >
                                                    {catalog_type === "rainbow" && (<>
                                                            {field.value
                                                                ? (catalogs as Catalog[]).find((c) => c["@id"] === field.value)?.title || field.value // Display title if available, otherwise ID
                                                                : "Select catalog..."
                                                            }
                                                        </>
                                                    )}
                                                    {catalog_type === "datahub" && (<>
                                                            {field.value
                                                                ? (catalogs as DatahubDomain[]).find((c) => c.properties.name === field.value)?.properties.name || field.value // Display title if available, otherwise ID
                                                                : "Select catalog..."
                                                            }
                                                        </>
                                                    )}

                                                    <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50"/>
                                                </Button>
                                            </PopoverTrigger>
                                            <PopoverContent className="w-[--radix-popover-trigger-width] p-0">
                                                <Command>
                                                    <CommandInput placeholder="Search catalog..."/>
                                                    <CommandList>
                                                        <CommandEmpty>No catalog found.</CommandEmpty>
                                                        <CommandGroup>
                                                            {catalogs.map((catalog) => (
                                                                <>
                                                                    {catalog_type === "rainbow" && (
                                                                        <CommandItem
                                                                            key={(catalog as Catalog)["@id"]}
                                                                            value={(catalog as Catalog).title || (catalog as Catalog)["@id"]} // Use title for search, ID for actual value
                                                                            onSelect={() => {
                                                                                if (field.value !== (catalog as Catalog)["@id"]) { // Clear subsequent fields ONLY if catalog changes
                                                                                    field.onChange((catalog as Catalog)["@id"]);
                                                                                    setSelectedCatalog(catalog);
                                                                                    setCatalogOpen(false);
                                                                                    // Clear Dataset and Policy fields
                                                                                    clearFields(["id", "target"]);
                                                                                } else {
                                                                                    setCatalogOpen(false); // Just close if same value
                                                                                }
                                                                            }}
                                                                            className={field.value === (catalog as Catalog)["@id"] ? "bg-blue-50 text-blue-700 font-medium" : ""}
                                                                        >
                                                                            {(catalog as Catalog).title || (catalog as Catalog)["@id"]}
                                                                        </CommandItem>
                                                                    )}
                                                                    {catalog_type === "datahub" && (
                                                                        <CommandItem
                                                                            key={(catalog as DatahubDomain).properties.name}
                                                                            value={(catalog as DatahubDomain).properties.name || (catalog as DatahubDomain).urn} // Use title for search, ID for actual value
                                                                            onSelect={() => {
                                                                                if (field.value !== (catalog as DatahubDomain).urn) { // Clear subsequent fields ONLY if catalog changes
                                                                                    field.onChange((catalog as DatahubDomain).urn);
                                                                                    setSelectedCatalog(catalog);
                                                                                    setCatalogOpen(false);
                                                                                    // Clear Dataset and Policy fields
                                                                                    clearFields(["id", "target"]);
                                                                                } else {
                                                                                    setCatalogOpen(false); // Just close if same value
                                                                                }
                                                                            }}
                                                                            className={field.value === (catalog as DatahubDomain).urn ? "bg-blue-50 text-blue-700 font-medium" : ""}
                                                                        >
                                                                            {(catalog as DatahubDomain).properties.name || (catalog as DatahubDomain).urn}
                                                                        </CommandItem>
                                                                    )}

                                                                </>
                                                            ))}
                                                        </CommandGroup>
                                                    </CommandList>
                                                </Command>
                                            </PopoverContent>
                                        </Popover>
                                    </FormControl>
                                    <FormDescription className="text-sm text-gray-400 mt-1">Select a catalog to browse
                                        available datasets.</FormDescription>
                                    <FormMessage/>
                                </div>
                            </FormItem>
                        )}
                    />

                    {/* Dataset Field (mapped to 'id' in Inputs) */}
                    <FormField
                        control={control}
                        name="target" // This is the dataset ID field
                        render={({field}) => (
                            <FormItem>
                                <FormLabel>Dataset</FormLabel>
                                <div>
                                    <FormControl>
                                        <Popover open={datasetOpen} onOpenChange={handleDatasetOpenChange}>
                                            <PopoverTrigger asChild>
                                                <Button
                                                    variant="outline"
                                                    role="combobox"
                                                    aria-expanded={datasetOpen}
                                                    className="w-full justify-between font-normal text-gray-600 hover:text-gray-800 transition-colors"
                                                    disabled={!selectedCatalog} // Disable if no catalog selected
                                                >

                                                    {catalog_type === "rainbow" && (<>
                                                            {field.value
                                                                ? (datasets as Dataset[]).find((d) => d["@id"] === field.value)?.title || field.value
                                                                : selectedCatalog ? "Select dataset..." : "Select a catalog first"
                                                            }
                                                        </>
                                                    )}
                                                    {catalog_type === "datahub" && (<>
                                                            {field.value
                                                                ? (datasets as DatahubDataset[]).find((d) => d.urn === field.value)?.name || field.value
                                                                : selectedCatalog ? "Select dataset..." : "Select a catalog first"
                                                            }
                                                        </>
                                                    )}
                                                    <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50"/>
                                                </Button>
                                            </PopoverTrigger>
                                            <PopoverContent className="w-[--radix-popover-trigger-width] p-0">
                                                <Command>
                                                    <CommandInput placeholder="Search dataset..."/>
                                                    <CommandList>
                                                        <CommandEmpty>No dataset found.</CommandEmpty>
                                                        <CommandGroup>
                                                            {datasets.map((dataset) => (
                                                                <>
                                                                    {catalog_type === "rainbow" && (
                                                                        <CommandItem
                                                                            key={(dataset as Dataset)["@id"]}
                                                                            value={(dataset as Dataset).title || (dataset as Dataset)["@id"]}
                                                                            onSelect={() => {
                                                                                if (field.value !== (dataset as Dataset)["@id"]) { // Clear subsequent fields ONLY if dataset changes
                                                                                    field.onChange((dataset as Dataset)["@id"]);
                                                                                    setSelectedDataset(dataset);
                                                                                    setDatasetOpen(false);
                                                                                    // Clear Policy field
                                                                                    clearFields(["id"]);
                                                                                } else {
                                                                                    setDatasetOpen(false); // Just close if same value
                                                                                }
                                                                            }}
                                                                            className={field.value === (dataset as Dataset)["@id"]
                                      ? "bg-blue-50 text-blue-700 font-medium"
                                                                        : ""
                                          }
                                >
                                  {(dataset as Dataset).title || (dataset as Dataset)["@id"]}
                                                                        </CommandItem>
                                                                    )}
                                                                    {catalog_type === "datahub" && (
                                                                        <CommandItem
                                                                            key={(dataset as DatahubDataset).urn}
                                                                            value={(dataset as DatahubDataset).name || (dataset as DatahubDataset).urn}
                                                                            onSelect={() => {
                                                                                if (field.value !== (dataset as DatahubDataset).urn) { // Clear subsequent fields ONLY if dataset changes
                                                                                    field.onChange((dataset as DatahubDataset).urn);
                                                                                    setSelectedDataset(dataset);
                                                                                    setDatasetOpen(false);
                                                                                    // Clear Policy field
                                                                                    clearFields(["id"]);
                                                                                } else {
                                                                                    setDatasetOpen(false); // Just close if same value
                                                                                }
                                                                            }}
                                                                            className={field.value === (dataset as DatahubDataset).urn ? "bg-blue-50 text-blue-700 font-medium" : ""}
                                                                        >
                                                                            {(dataset as DatahubDataset).name || (dataset as DatahubDataset).urn}
                                                                        </CommandItem>
                                                                    )}
                                                                </>

                                                            ))}
                                                        </CommandGroup>
                                                    </CommandList>
                                                </Command>
                                            </PopoverContent>
                                        </Popover>
                                    </FormControl>
                                    <FormDescription className="text-sm text-gray-400 mt-1">Choose a specific dataset
                                        from the selected catalog.</FormDescription>
                                    <FormMessage/>
                                </div>
                            </FormItem>
                        )}
                    />

          {/* Policy Target Field (mapped to 'target' in Inputs) */}
          <FormField
            control={control}
            name="id" // This is the policy ID field
            render={({ field }) => (
              <FormItem>
                <FormLabel>Policy Id</FormLabel>
                <div>
                  <FormControl>
                    <Popover
                      open={policiesOpen}
                      onOpenChange={handlePoliciesOpenChange}
                    >
                      <PopoverTrigger asChild>
                        <Button
                          variant="outline"
                          role="combobox"
                          aria-expanded={policiesOpen}
                          className="w-full justify-between font-normal text-gray-600 hover:text-gray-800 transition-colors"
                          disabled={!selectedDataset} // Disable if no dataset selected
                        >
                          {field.value
                            ? policies.find((p) => p["@id"] === field.value)
                                ?.["@id"] || field.value
                            : selectedDataset
                              ? "Select policy..."
                              : "Select a dataset first"}
                          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                        </Button>
                      </PopoverTrigger>
                      <PopoverContent className="w-[--radix-popover-trigger-width] p-0">
                        <Command>
                          <CommandInput placeholder="Search policies..." />
                          <CommandList>
                            <CommandEmpty>No policies found.</CommandEmpty>
                            <CommandGroup>
                              {policies.map((policy) => (
                                <CommandItem
                                  key={policy["@id"]}
                                  value={policy.target || policy["@id"]}
                                  onSelect={() => {
                                    field.onChange(policy["@id"]);
                                    setSelectedPolicy(policy);
                                    setPoliciesOpen(false);
                                    // No fields follow this one that need clearing based on its change
                                  }}
                                  className={
                                    field.value === policy["@id"]
                                      ? "bg-blue-50 text-blue-700 font-medium"
                                      : ""
                                  }
                                >
                                  {policy["@id"]}
                                </CommandItem>
                              ))}
                            </CommandGroup>
                          </CommandList>
                        </Command>
                      </PopoverContent>
                    </Popover>
                  </FormControl>
                  <FormDescription className="text-sm text-gray-400 mt-1">
                    Select the policy target for the negotiation.
                  </FormDescription>
                  <FormMessage />
                </div>
              </FormItem>
            )}
          />

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
  );
};

export const Route = createFileRoute("/contract-negotiation/offer")({
  component: RouteComponent,
  pendingComponent: () => (
    <div className="p-4 text-center text-gray-600">Loading...</div>
  ),
});
