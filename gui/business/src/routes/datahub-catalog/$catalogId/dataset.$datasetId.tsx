import {createFileRoute} from '@tanstack/react-router'
import {Table, TableBody, TableCell, TableHead, TableHeader, TableRow} from "shared/src/components/ui/table.tsx";
import {FormProvider, useForm} from "react-hook-form"; // Import FormProvider
import {usePostNewPolicyInDataset} from "shared/src/data/catalog-mutations.ts";
import {useContext, useEffect} from "react";
import {GlobalInfoContext, GlobalInfoContextType} from "shared/src/context/GlobalInfoContext.tsx";
import {
    useBusinessGetPoliciesByDatasetId,
    useGetBusinessDatahubDataset,
    useGetBusinessPolicyTemplates
} from "shared/src/data/business-queries.ts";
import {AuthContext, AuthContextType} from "shared/src/context/AuthContext.tsx";
import {
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "shared/src/components/ui/form.tsx";
import {Textarea} from "shared/src/components/ui/textarea.tsx";
import {Button} from "shared/src/components/ui/button.tsx";
import {Input} from "shared/src/components/ui/input.tsx";

import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "shared/src/components/ui/select.tsx";

// Define the interface for PolicyTemplate, as it's crucial for type safety
interface PolicyTemplate {
    id: string;
    title: string;
    description: string;
    content: any; // ODRL content structure
    operand_options: {
        [key: string]: {
            dataType: string;
            defaultValue: string;
            formType: 'datetime' | 'select' | 'text'; // Added 'text' for generic input
            label: { "@language": string; "@value": string }[];
            options?: { label: { "@language": string; "@value": string }[]; value: string }[] | null;
        }
    };
}

// Define a type for the form values, which will be dynamic
type DynamicFormValues = {
    [key: string]: string; // Keys will be "$location", "$expiry_date", etc.
};

function RouteComponent() {
    const {catalogId, datasetId} = Route.useParams()
    const {participant} = useContext<AuthContextType | null>(AuthContext)!;
    // Use optional chaining for data properties as they might be undefined initially
    const {data: dataset} = useGetBusinessDatahubDataset(datasetId)
    const {data: policies} = useBusinessGetPoliciesByDatasetId(catalogId, datasetId)
    // Cast policy_templates data to the defined interface for better type checking
    const {data: policy_templates} = useGetBusinessPolicyTemplates() as { data: PolicyTemplate[] };
    const {mutateAsync: createPolicyAsync, isPending} = usePostNewPolicyInDataset()
    const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!

    // Initialize useForm with the DynamicFormValues type and an empty defaultValues object
    const form = useForm<DynamicFormValues>({
        defaultValues: {},
    })

    // This effect will populate the form with default values from policy_templates
    // when the policy_templates data becomes available.
    useEffect(() => {
        if (policy_templates) {
            const initialValues: DynamicFormValues = {};
            policy_templates.forEach(template => {
                // Iterate through each template's operand_options to set initial values
                Object.entries(template.operand_options).forEach(([key, value]) => {
                    if (key && value.defaultValue !== undefined) {
                        initialValues[key] = value.defaultValue;
                    }
                });
            });
            // Reset the form with the collected default values.
            // This is crucial to ensure react-hook-form recognizes the fields.
            form.reset(initialValues);
        }
    }, [policy_templates, form]); // Dependencies: re-run if policy_templates or form instance changes

    // onSubmit function to handle form submission
    const onSubmit = (formData: DynamicFormValues, currentPolicyTemplate: PolicyTemplate) => {
        let odrlContent = JSON.parse(JSON.stringify(currentPolicyTemplate.content));

        // Iterate over the submitted form data to replace placeholders in the ODRL content
        for (const key in formData) {
            if (formData.hasOwnProperty(key)) {
                const value = formData[key];
                // Convert the ODRL content to a string to perform string replacement
                let contentString = JSON.stringify(odrlContent);
                // Replace all occurrences of the placeholder (e.g., "$location") with the actual value
                // The RegExp is used to ensure all instances of the placeholder are replaced globally.
                contentString = contentString.replace(new RegExp(`"${key}"`, 'g'), `"${value}"`);
                // Parse the string back to a JSON object
                odrlContent = JSON.parse(contentString);
            }
        }

        console.log("ODRL conformed:", odrlContent);


        //
        // // Call the mutation to post the new policy
        // createPolicyAsync({
        //     api_gateway,
        //     datasetId,
        //     content: {
        //         offer: odrlContent // The modified ODRL content with placeholders replaced
        //     }
        // });
        form.reset(); // Reset the form fields after successful submission
    }


    return (
        <div className="space-y-4">
            <h2>Dataset info with id: {dataset?.name} </h2> {/* Use optional chaining */}
            <div>
                <Table className="text-sm">
                    <TableHeader>
                        <TableRow>
                            <TableHead>Key</TableHead>
                            <TableHead>Value</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {/* Use optional chaining for dataset and custom_properties */}
                        {dataset?.custom_properties?.map((property => (
                            <TableRow key={property[0]}>
                                <TableCell>{property[0]}</TableCell>
                                <TableCell>{property[1]}</TableCell>
                            </TableRow>
                        )))}
                    </TableBody>
                </Table>
            </div>

            <div>
                <h2>ODRL Policies</h2>
                <Table className="text-sm">
                    <TableHeader>
                        <TableRow>
                            <TableHead>Policy Id</TableHead>
                            <TableHead>Policy Target</TableHead>
                            <TableHead>ODRL Content</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {/* Use optional chaining for policies */}
                        {policies?.map((policy) => (
                            <TableRow key={policy["@id"].slice(0, 20)}>
                                <TableCell>
                                    {policy["@id"].slice(0, 20) + "..."}
                                </TableCell>
                                <TableCell>
                                    {policy.target?.slice(0, 20) + "..."}
                                </TableCell>
                                <TableCell>
                                    {JSON.stringify(policy)}
                                </TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            </div>

            {/* Render this section only if participant is a Provider and policy_templates are loaded */}
            {
                participant?.participant_type == "Provider" && policy_templates &&
                (<>
                    <div>
                        <h2>Create new ODRL policy from template</h2>
                        <Table className="text-sm">
                            <TableHeader>
                                <TableRow>
                                    <TableHead>Policy Template Id</TableHead>
                                    <TableHead>Policy Template Title</TableHead>
                                    <TableHead>Policy Template Description</TableHead>
                                    <TableHead>Policy Template Content</TableHead>
                                    <TableHead>Policy Template Options</TableHead>
                                </TableRow>
                            </TableHeader>
                            <TableBody>
                                {/* Map through each policy template to display its information and a form */}
                                {policy_templates.map(policy_template => (
                                    <TableRow key={policy_template.id}> {/* Add a unique key for each row */}
                                        <TableCell>
                                            {policy_template.id.slice(0, 20) + "..."}
                                        </TableCell>
                                        <TableCell>
                                            {policy_template.title}
                                        </TableCell>
                                        <TableCell>
                                            {policy_template.description}
                                        </TableCell>
                                        <TableCell>
                                            {JSON.stringify(policy_template.content)}
                                        </TableCell>
                                        <TableCell>
                                            {/* Use FormProvider explicitly to ensure context is available for FormField */}
                                            <FormProvider {...form}>
                                                {/* The actual HTML form element */}
                                                <form
                                                    onSubmit={form.handleSubmit((data) => onSubmit(data, policy_template))}
                                                    className="space-y-4" // Add some spacing to the form
                                                >
                                                    {/* Map through each operand option to render dynamic form fields */}
                                                    {Object.entries(policy_template.operand_options).map(([optionKey, optionValue]) => {
                                                        return (
                                                            <FormField
                                                                key={optionKey} // Unique key for each FormField
                                                                disabled={isPending}
                                                                control={form.control}
                                                                name={optionKey} // The name of the field in react-hook-form
                                                                render={({field}) => {
                                                                    let inputComponent; // Declare a variable to hold the input component inside render

                                                                    // Conditionally assign the correct input component
                                                                    if (optionValue.formType === "text") {
                                                                        inputComponent =
                                                                            <Input {...field} type="text"/>;
                                                                    } else if (optionValue.formType === "datetime") {
                                                                        inputComponent =
                                                                            <Input {...field} type="datetime-local"/>;
                                                                    } else if (optionValue.formType === "select") {
                                                                        inputComponent = (
                                                                            <Select onValueChange={field.onChange}
                                                                                    defaultValue={field.value}>
                                                                                <SelectTrigger>
                                                                                    <SelectValue
                                                                                        placeholder="Select an option"/>
                                                                                </SelectTrigger>
                                                                                <SelectContent>
                                                                                    {optionValue.options?.map(opt => (
                                                                                        <SelectItem key={opt.value}
                                                                                                    value={opt.value}>
                                                                                            {opt.label.find(l => l["@language"] == "en")?.["@value"]}
                                                                                        </SelectItem>
                                                                                    ))}
                                                                                </SelectContent>
                                                                            </Select>
                                                                        );
                                                                    } else {
                                                                        // Default to Textarea if no specific formType or for generic text
                                                                        inputComponent = <Textarea {...field} />;
                                                                    }

                                                                    return (
                                                                        <FormItem>
                                                                            <FormLabel>Option: {optionKey}</FormLabel>
                                                                            <FormControl>
                                                                                {/* Render the single input component */}
                                                                                {inputComponent}
                                                                            </FormControl>
                                                                            <FormDescription>
                                                                                {optionValue.label.find(l => l["@language"] == "en")?.["@value"]}
                                                                            </FormDescription>
                                                                            <FormMessage/>
                                                                        </FormItem>
                                                                    );
                                                                }}
                                                            />
                                                        );
                                                    })}
                                                    <Button type="submit" disabled={isPending}>
                                                        Enviar {isPending && <span>- loading...</span>}
                                                    </Button>
                                                </form>
                                            </FormProvider>
                                        </TableCell>
                                    </TableRow>
                                ))}
                            </TableBody>
                        </Table>
                    </div>
                </>)
            }
        </div>
    );
}

export const Route = createFileRoute('/datahub-catalog/$catalogId/dataset/$datasetId')({
    component: RouteComponent,
    pendingComponent: () => <div>Loading...</div>,
})
