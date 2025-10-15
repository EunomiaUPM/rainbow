import {createFileRoute} from "@tanstack/react-router";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "shared/src/components/ui/form";
import {SubmitHandler, useForm} from "react-hook-form";
import {Button} from "shared/src/components/ui/button.tsx";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "shared/src/components/ui/command";
import {Popover, PopoverContent, PopoverTrigger} from "shared/src/components/ui/popover";
import {ChevronsUpDown} from "lucide-react";
import {useContext, useEffect, useState} from "react"; // Import useEffect
import {getParticipants} from "shared/src/data/participant-queries.ts";
import {getPoliciesByDatasetId} from "shared/src/data/policy-queries.ts";
import {usePostContractNegotiationRPCOffer} from "shared/src/data/contract-mutations.ts";
import {GlobalInfoContext, GlobalInfoContextType} from "shared/src/context/GlobalInfoContext.tsx";
import {Badge} from "shared/src/components/ui/badge";
import PolicyComponent from "shared/src/components/PolicyComponent.tsx";

type Inputs = {
  consumerParticipantId: UUID;
  id: UUID; // This seems to be used for dataset ID, consider renaming for clarity
  catalog: UUID;
  target: UUID;
  odrl: string;
};

export const RouteComponent = ({catalog, dataset}: { catalog: Catalog, dataset: Dataset }) => {
  const {mutateAsync: sendOfferAsync, isPending} = usePostContractNegotiationRPCOffer();
  const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;

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
  const {handleSubmit, control, setValue, getValues} = form;

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
    // console.log(defaultId, "defaultId in offer form");
    setSelectedCatalog(catalog);
    // console.log(selectedCatalog, " selectedCatalog__");
    /// !!!!! modificacion 1
    // if (defaultId && catalogs.length > 0) {
    //   const catalog = catalogs.find((c) => c["@id"] === defaultId);
    //   if (catalog) {
    //     setSelectedCatalog(catalog);
    //   }
    // }
  }, [getValues, catalogs, catalog]);

  // Initialize selectedDataset if default value exists
  useEffect(() => {
    const defaultId = dataset["@id"];
    console.log(defaultId, "defaultId dataset in offer form");
    setSelectedDataset(dataset);
    form.setValue("target", dataset["@id"]);
    console.log(selectedDataset, " selectedDataset__");
    // !!!!! modificacion 3
    // 'id' field is for dataset
    // if (defaultId && datasets.length > 0) {
    //   const dataset = datasets.find((d) => d["@id"] === defaultId);
    //   if (dataset) {
    //     setSelectedDataset(dataset);
    //   }
    // }
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
      setValue(fieldName, "", {shouldValidate: true}); // Clear form value
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
            obligation: null,
            prohibition: null,
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
            {/* <p className="text-xs"> Chosen catalog:  </p> */}
            <p>{catalog.title}</p>{" "}
            <Badge variant="info">{catalog["@id"].slice(9, 29) + "[...]"}</Badge>
          </div>
          {/* {console.log(selectedCatalog, " selectedCatalogº")} */}
          <FormField
            control={control}
            name="catalog"
            render={() => (
              <FormItem>
                {/* <FormLabel>Catalog</FormLabel>
                <div>
                  <FormControl>
                    <Popover
                      open={catalogOpen}
                      onOpenChange={handleCatalogOpenChange}
                    >
                      <PopoverTrigger asChild>
                        <Button
                          variant="outline"
                          role="combobox"
                          aria-expanded={catalogOpen}
                          className="w-full justify-between font-normal text-gray-600 hover:text-gray-800 transition-colors"
                        >
                          {
                            field.value
                            // ? catalogs.find((c) => c["@id"] === field.value)
                            //     ?.title || field.value // Display title if available, otherwise ID
                            // : "Select catalog..."
                          }
                          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                        </Button>
                      </PopoverTrigger>
                      <PopoverContent className="w-[--radix-popover-trigger-width] p-0">
                        <Command>
                          <CommandInput placeholder="Search catalog..." />
                          <CommandList>
                            <CommandEmpty>No catalog found.</CommandEmpty>
                            <CommandGroup>
                              {catalogs.map((catalog) => (
                                <CommandItem
                                  key={catalog["@id"]}
                                  value={catalog.title || catalog["@id"]} // Use title for search, ID for actual value
                                  onSelect={() => {
                                    if (field.value !== catalog["@id"]) {
                                      // Clear subsequent fields ONLY if catalog changes
                                      field.onChange(catalog.title);
                                      setSelectedCatalog(catalog);
                                      setCatalogOpen(false);
                                      // Clear Dataset and Policy fields
                                      clearFields(["id", "target"]);
                                    } else {
                                      setCatalogOpen(false); // Just close if same value
                                    }
                                  }}
                                  className={
                                    field.value === catalog["@id"]
                                      ? "bg-blue-50 text-blue-700 font-medium"
                                      : ""
                                  }
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
                  <FormDescription className="text-sm text-gray-400 mt-1">
                    Select a catalog to browse available datasets.
                  </FormDescription>
                  <FormMessage />
                </div> */}
              </FormItem>
            )}
          />

          {/* Dataset Field (mapped to 'id' in Inputs) */}

          <div>
            Chosen dataset: {dataset.title} {dataset["@id"]}
          </div>

          {/*
          <FormField
            control={control}
            name="target" // This is the dataset ID field
            render={({ field }) => (
              <FormItem>
                <FormLabel>Dataset</FormLabel>
                <div>
                  {" "}
                  Chosen dataset: {dataset.title} {dataset["@id"]}
                </div>
                {console.log(selectedDataset, " selectedDatasetº")}
                <div>
                  <FormControl>
                    <Popover
                      open={datasetOpen}
                      onOpenChange={handleDatasetOpenChange}
                    >
                      <PopoverTrigger asChild>
                        <Button
                          variant="outline"
                          role="combobox"
                          aria-expanded={datasetOpen}
                          className="w-full justify-between font-normal text-gray-600 hover:text-gray-800 transition-colors"
                          disabled={!selectedCatalog} // Disable if no catalog selected
                        >
                          {
                            field.value
                            // ? datasets.find((d) => d["@id"] === field.value)
                            //     ?.title || field.value
                            // : selectedCatalog
                            //   ?   <div>{console.log(dataset, "alguien?")}</div>
                            //   : "Select a catalog first"
                          }
                          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                        </Button>
                      </PopoverTrigger>
                      <PopoverContent className="w-[--radix-popover-trigger-width] p-0">
                        <Command>
                          <CommandInput placeholder="Search dataset..." />
                          <CommandList>
                            <CommandEmpty>No dataset found.</CommandEmpty>
                            <CommandGroup>
                              {/* {datasets.map((dataset) => ( */}

          {/* </CommandGroup>
                          </CommandList>
                        </Command>
                      </PopoverContent>
                    </Popover>
                  </FormControl>
                  <FormDescription className="text-sm text-gray-400 mt-1">
                    Choose a specific dataset from the selected catalog.
                  </FormDescription>
                  <FormMessage />
                </div>
              </FormItem>
            )}
          /> */}
          {/* Consumer Participant Field */}
          <FormField
            control={control}
            name="consumerParticipantId"
            render={({field}) => (
              <FormItem>
                <FormLabel>Consumer Participant Id</FormLabel>
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
                            ? consumerParticipants.find((p) => p.participant_id === field.value)
                              ?.participant_id
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
                                  className={
                                    field.value === consumerParticipant.participant_id
                                      ? "text-role-consumer font-medium"
                                      : ""
                                  }
                                >
                                  {consumerParticipant.participant_id}
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
                  <FormDescription className="text-sm text-gray-400 mt-1">
                    Provide the ID of the consumer participant for the negotiation.
                  </FormDescription>
                  <FormMessage/>
                </div>
              </FormItem>
            )}
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
                <PolicyComponent policyItem={policy.permission} variant={"permission"}/>

                <PolicyComponent policyItem={policy.obligation} variant={"obligation"}/>

                <PolicyComponent policyItem={policy.prohibition} variant={"prohibition"}/>
              </div>
            ))}
          {/* Policy Target Field (mapped to 'target' in Inputs) */}
          {/* <FormField
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
                          {console.log(selectedPolicy?.target, "selectedPolicdy")}
                          {field.value
                            // ? policies.find((p) => p["@id"] === field.value)
                            //     ?.target || field.value
                            // : selectedDataset
                            //   ? "Select policy..."
                            //   : "Select a dataset first"
                              }
                          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
                        </Button>
                      </PopoverTrigger>
                      <PopoverContent className="w-[--radix-popover-trigger-width] p-0">
                        <Command>
                          <CommandInput placeholder="Search policies..." />
                          <CommandList>
                            <CommandEmpty>No policies found.</CommandEmpty>
                            <CommandGroup>
                              {/* {policies.map((policy) => ( */}

          {/* <CommandItem
                                  key={selectedPolicy?.["@id"]}
                                  value={selectedPolicy?.target ||selectedPolicy?.["@id"]}
                                  onSelect={() => {

                                    // field.onChange(policy["@id"]);
                                    // setSelectedPolicy(policy);
                                    // setPoliciesOpen(false);
                                    // No fields follow this one that need clearing based on its change
                                  }}
                                  className={
                                    field.value === selectedPolicy?.["@id"]
                                      ? "bg-blue-50 text-blue-700 font-medium"
                                      : ""
                                  }
                                >
                                       {console.log(field.value, "field value")}
                                  {selectedPolicy?.target || selectedPolicy?.["@id"]}
                                </CommandItem>
                              {/* ))} */}
          {/* </CommandGroup>
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
          /> */}

          {/*<FormField*/}
          {/*  control={form.control}*/}
          {/*  name="odrl"*/}
          {/*  disabled={!selectedPolicy}*/}
          {/*  render={({field}) => (*/}
          {/*    <FormItem>*/}
          {/*      <FormLabel>Odrl</FormLabel>*/}
          {/*      <FormControl>*/}
          {/*        <Textarea {...field} />*/}
          {/*      </FormControl>*/}
          {/*      <FormDescription>Provide the ODRL policy content</FormDescription>*/}
          {/*      <FormMessage/>*/}
          {/*    </FormItem>*/}
          {/*  )}*/}
          {/*/>*/}

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
  pendingComponent: () => <div className="p-4 text-center text-gray-600">Loading...</div>,
});
