import {createFileRoute} from "@tanstack/react-router";
import {getContractNegotiationProcessesOptions} from "@/data/contract-queries.ts";
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
import {useEffect, useState} from "react"; // Import useEffect
import {getParticipants} from "@/data/participant-queries.ts";
import {getCatalogs, getDatasetsByCatalogId} from "@/data/catalog-queries.ts";
import {getPoliciesByDatasetId} from "@/data/policy-queries.ts";
import {Textarea} from "shared/src/components/ui/textarea.tsx";


type Inputs = {
    consumerParticipantId: UUID,
    id: UUID, // This seems to be used for dataset ID, consider renaming for clarity
    catalog: UUID,
    target: UUID,
    odrl: string
}

const RouteComponent = () => {
    // --- State Management ---
    // Consumer Participant
    const [consumerParticipantOpen, setConsumerParticipantOpen] = useState(false);
    const [_consumerSelectedParticipant, setConsumerSelectedParticipant] = useState<Participant | null>(null);
    const [consumerParticipants, setConsumerParticipants] = useState<Participant[]>([]);

    // Catalog
    const [catalogOpen, setCatalogOpen] = useState(false);
    const [selectedCatalog, setSelectedCatalog] = useState<Catalog | null>(null);
    const [catalogs, setCatalogs] = useState<Catalog[]>([]);

    // Dataset (used for 'id' field in Inputs)
    const [datasetOpen, setDatasetOpen] = useState(false);
    const [selectedDataset, setSelectedDataset] = useState<Dataset | null>(null);
    const [datasets, setDatasets] = useState<Dataset[]>([] /* Initial empty array */);

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
    const {handleSubmit, control, setValue, getValues} = form;

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
        if (defaultId && catalogs.length > 0) {
            const catalog = catalogs.find(c => c["@id"] === defaultId);
            if (catalog) {
                setSelectedCatalog(catalog);
            }
        }
    }, [getValues, catalogs]);

    // Initialize selectedDataset if default value exists
    useEffect(() => {
        const defaultId = getValues("id"); // 'id' field is for dataset
        if (defaultId && datasets.length > 0) {
            const dataset = datasets.find(d => d["@id"] === defaultId);
            if (dataset) {
                setSelectedDataset(dataset);
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
            } else if (fieldName === "id") { // Dataset field
                setSelectedDataset(null);
                setDatasets([]); // Clear dataset options
            } else if (fieldName === "target") { // Policy field
                setSelectedPolicy(null);
                setPolicies([]); // Clear policy options
            }
            // Add more conditions for other fields if they have specific local states
        });
    };

    // --- Form Submission Handler ---
    const onSubmit: SubmitHandler<Inputs> = data => {
        console.log("Form data submitted:", data);
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
                const participants = await getParticipants();
                setConsumerParticipants(participants);
            } catch (error) {
                console.error("Failed to fetch participants:", error);
            }
        }
    }

    const handleCatalogOpenChange = async (newOpenState: boolean) => {
        setCatalogOpen(newOpenState);
        if (newOpenState) {
            try {
                const fetchedCatalogs = await getCatalogs();
                setCatalogs(fetchedCatalogs.catalog);
            } catch (error) {
                console.error("Failed to fetch catalogs:", error);
            }
        }
    }

    const handleDatasetOpenChange = async (newOpenState: boolean) => {
        setDatasetOpen(newOpenState);
        if (newOpenState && selectedCatalog) { // Only fetch if a catalog is selected
            try {
                const fetchedDatasets = await getDatasetsByCatalogId(selectedCatalog["@id"]);
                setDatasets(fetchedDatasets);
            } catch (error) {
                console.error("Failed to fetch datasets:", error);
            }
        } else if (newOpenState && !selectedCatalog) {
            // Optionally, show a warning or prevent opening if no catalog is selected
            console.warn("Please select a catalog first.");
            setDatasetOpen(false); // Prevent popover from opening
        }
    }

    const handlePoliciesOpenChange = async (newOpenState: boolean) => {
        setPoliciesOpen(newOpenState);
        if (newOpenState && selectedDataset) { // Only fetch if a dataset is selected
            try {
                const fetchedPolicies = await getPoliciesByDatasetId(selectedDataset["@id"]);
                setPolicies(fetchedPolicies);
            } catch (error) {
                console.error("Failed to fetch policies:", error);
            }
        } else if (newOpenState && !selectedDataset) {
            console.warn("Please select a dataset first.");
            setPoliciesOpen(false);
        }
    }

    return (
        <div className="">
            <h2 className="text-2xl font-bold mb-6 text-gray-800">New Contract Negotiation Offer</h2>
            <Form {...form}>
                <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">

                    {/* Consumer Participant Field */}
                    <FormField
                        control={control}
                        name="consumerParticipantId"
                        render={({field}) => (
                            <FormItem>
                                <FormLabel className="text-gray-700">Consumer Participant Id</FormLabel>
                                <div>
                                    <FormControl>
                                        <Popover open={consumerParticipantOpen}
                                                 onOpenChange={handleConsumerParticipantOpenChange}>
                                            <PopoverTrigger asChild>
                                                <Button
                                                    variant="outline"
                                                    role="combobox"
                                                    aria-expanded={consumerParticipantOpen}
                                                    className="w-full justify-between font-normal text-gray-600 hover:text-gray-800 transition-colors"
                                                >
                                                    {field.value
                                                        ? consumerParticipants.find((p) => p.participant_id === field.value)?.participant_id
                                                        : "Select participant..."}
                                                    <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50"/>
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
                                                                    className={field.value === consumerParticipant.participant_id ? "bg-blue-50 text-blue-700 font-medium" : ""}
                                                                >
                                                                    {consumerParticipant.participant_id}
                                                                    <span
                                                                        className="text-gray-500 ml-2 text-sm">({consumerParticipant.base_url})</span>
                                                                </CommandItem>
                                                            ))}
                                                        </CommandGroup>
                                                    </CommandList>
                                                </Command>
                                            </PopoverContent>
                                        </Popover>
                                    </FormControl>
                                    <FormDescription className="text-sm text-gray-500 mt-1">Provide the ID of the
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
                                <FormLabel className="text-gray-700">Catalog</FormLabel>
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
                                                    {field.value
                                                        ? catalogs.find((c) => c["@id"] === field.value)?.title || field.value // Display title if available, otherwise ID
                                                        : "Select catalog..."
                                                    }
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
                                                                <CommandItem
                                                                    key={catalog["@id"]}
                                                                    value={catalog.title || catalog["@id"]} // Use title for search, ID for actual value
                                                                    onSelect={() => {
                                                                        if (field.value !== catalog["@id"]) { // Clear subsequent fields ONLY if catalog changes
                                                                            field.onChange(catalog["@id"]);
                                                                            setSelectedCatalog(catalog);
                                                                            setCatalogOpen(false);
                                                                            // Clear Dataset and Policy fields
                                                                            clearFields(["id", "target"]);
                                                                        } else {
                                                                            setCatalogOpen(false); // Just close if same value
                                                                        }
                                                                    }}
                                                                    className={field.value === catalog["@id"] ? "bg-blue-50 text-blue-700 font-medium" : ""}
                                                                >
                                                                    {catalog.title || catalog["@id"]}
                                                                </CommandItem>
                                                            ))}
                                                        </CommandGroup>
                                                    </CommandList>
                                                </Command>
                                            </PopoverContent>
                                        </Popover>
                                    </FormControl>
                                    <FormDescription className="text-sm text-gray-500 mt-1">Select a catalog to browse
                                        available datasets.</FormDescription>
                                    <FormMessage/>
                                </div>
                            </FormItem>
                        )}
                    />

                    {/* Dataset Field (mapped to 'id' in Inputs) */}
                    <FormField
                        control={control}
                        name="id" // This is the dataset ID field
                        render={({field}) => (
                            <FormItem>
                                <FormLabel className="text-gray-700">Dataset</FormLabel>
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
                                                    {field.value
                                                        ? datasets.find((d) => d["@id"] === field.value)?.title || field.value
                                                        : selectedCatalog ? "Select dataset..." : "Select a catalog first"
                                                    }
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
                                                                <CommandItem
                                                                    key={dataset["@id"]}
                                                                    value={dataset.title || dataset["@id"]}
                                                                    onSelect={() => {
                                                                        if (field.value !== dataset["@id"]) { // Clear subsequent fields ONLY if dataset changes
                                                                            field.onChange(dataset["@id"]);
                                                                            setSelectedDataset(dataset);
                                                                            setDatasetOpen(false);
                                                                            // Clear Policy field
                                                                            clearFields(["target"]);
                                                                        } else {
                                                                            setDatasetOpen(false); // Just close if same value
                                                                        }
                                                                    }}
                                                                    className={field.value === dataset["@id"] ? "bg-blue-50 text-blue-700 font-medium" : ""}
                                                                >
                                                                    {dataset.title || dataset["@id"]}
                                                                </CommandItem>
                                                            ))}
                                                        </CommandGroup>
                                                    </CommandList>
                                                </Command>
                                            </PopoverContent>
                                        </Popover>
                                    </FormControl>
                                    <FormDescription className="text-sm text-gray-500 mt-1">Choose a specific dataset
                                        from the selected catalog.</FormDescription>
                                    <FormMessage/>
                                </div>
                            </FormItem>
                        )}
                    />

                    {/* Policy Target Field (mapped to 'target' in Inputs) */}
                    <FormField
                        control={control}
                        name="target" // This is the policy ID field
                        render={({field}) => (
                            <FormItem>
                                <FormLabel className="text-gray-700">Policy Target</FormLabel>
                                <div>
                                    <FormControl>
                                        <Popover open={policiesOpen} onOpenChange={handlePoliciesOpenChange}>
                                            <PopoverTrigger asChild>
                                                <Button
                                                    variant="outline"
                                                    role="combobox"
                                                    aria-expanded={policiesOpen}
                                                    className="w-full justify-between font-normal text-gray-600 hover:text-gray-800 transition-colors"
                                                    disabled={!selectedDataset} // Disable if no dataset selected
                                                >
                                                    {field.value
                                                        ? policies.find((p) => p["@id"] === field.value)?.target || field.value
                                                        : selectedDataset ? "Select policy..." : "Select a dataset first"
                                                    }
                                                    <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50"/>
                                                </Button>
                                            </PopoverTrigger>
                                            <PopoverContent className="w-[--radix-popover-trigger-width] p-0">
                                                <Command>
                                                    <CommandInput placeholder="Search policies..."/>
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
                                                                    className={field.value === policy["@id"] ? "bg-blue-50 text-blue-700 font-medium" : ""}
                                                                >
                                                                    {policy.target || policy["@id"]}
                                                                </CommandItem>
                                                            ))}
                                                        </CommandGroup>
                                                    </CommandList>
                                                </Command>
                                            </PopoverContent>
                                        </Popover>
                                    </FormControl>
                                    <FormDescription className="text-sm text-gray-500 mt-1">Select the policy target for
                                        the negotiation.</FormDescription>
                                    <FormMessage/>
                                </div>
                            </FormItem>
                        )}
                    />

                    <FormField
                        control={form.control}
                        name="odrl"
                        disabled={!selectedPolicy}
                        render={({field}) => (
                            <FormItem>
                                <FormLabel>Odrl</FormLabel>
                                <FormControl>
                                    <Textarea {...field} />
                                </FormControl>
                                <FormDescription>Provide the ODRL policy content</FormDescription>
                                <FormMessage/>
                            </FormItem>
                        )}
                    />

                    <Button type="submit"
                            className="w-full">
                        Submit Offer
                    </Button>
                </form>
            </Form>
        </div>
    );
};

export const Route = createFileRoute("/contract-negotiation/offer")({
    component: RouteComponent,
    pendingComponent: () => <div className="p-4 text-center text-gray-600">Loading...</div>,
    loader: async ({context: {queryClient}}) => {
        let cnProcesses = await queryClient.ensureQueryData(getContractNegotiationProcessesOptions());
        return {cnProcesses};
    },
});