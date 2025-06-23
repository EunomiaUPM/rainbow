import { createFileRoute, Link } from "@tanstack/react-router";
import {
  useGetDatasetById,
  useGetDistributionsByDatasetId,
} from "shared/src/data/catalog-queries.ts";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "shared/src/components/ui/table.tsx";
import dayjs from "dayjs";
import { ExternalLink } from "lucide-react";
import { useGetPoliciesByDatasetId } from "shared/src/data/policy-queries.ts";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "shared/src/components/ui/form";
import { SubmitHandler, useForm } from "react-hook-form";
import { Button } from "shared/src/components/ui/button.tsx";
import { Textarea } from "shared/src/components/ui/textarea.tsx";
import { usePostNewPolicyInDataset } from "shared/src/data/catalog-mutations.ts";
import Heading from "shared/src/components/ui/heading";
import {
  List,
  ListItem,
  ListItemKey,
  ListItemDate,
} from "shared/src/components/ui/list.tsx";
import { Badge } from "shared/src/components/ui/badge";
import PolicyComponent from "shared/src/components/ui/PolicyComponent.tsx";
import { Plus, Trash } from "lucide-react";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "shared/src/components/ui/accordion";
import { Separator } from "shared/src/components/ui/separator";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "shared/src/components/ui/select.tsx";
import { Input } from "shared/src/components/ui/input";

type Inputs = {
  odrl: string;
};

function RouteComponent() {
  const { catalogId, datasetId } = Route.useParams();
  const { data: dataset } = useGetDatasetById(datasetId);
  const { data: distributions } = useGetDistributionsByDatasetId(datasetId);
  const { data: policies } = useGetPoliciesByDatasetId(datasetId);
  const { mutate: postNewPolicy, isPending } = usePostNewPolicyInDataset();
  const form = useForm<Inputs>({
    defaultValues: {
      odrl: '{"permission":[{"action":"use","constraint":[{"rightOperand":"user","leftOperand":"did:web:hola.es","operator":"eq"}]}],"obligation":[],"prohibition":[]}',
    },
  });
  const onSubmit: SubmitHandler<Inputs> = (data) => {
    // @ts-ignore
    postNewPolicy({ datasetId, body: data.odrl });
    form.reset();
  };

  return (
    <div className="space-y-4">
      <Heading level="h3" className="flex gap-2 items-center">
        Dataset info with id
        <Badge variant="info" size="lg">
          {" "}
          {dataset["@id"].slice(9, 29) + "[...]"}
        </Badge>
      </Heading>
      <div>
        <List className="text-sm">
          <ListItem>
            <ListItemKey>Dataset title</ListItemKey>
            <p>{dataset.title}</p>
          </ListItem>
          <ListItem>
            <ListItemKey>Catalog creation date</ListItemKey>
            <ListItemDate>
              {dayjs(dataset.issued).format("DD/MM/YYYY - HH:mm")}
            </ListItemDate>
          </ListItem>
        </List>
      </div>

      <div>
        <Heading level="h5">Distributions</Heading>
        <Table className="text-sm">
          <TableHeader>
            <TableRow>
              <TableHead>Distribution Id</TableHead>
              <TableHead>Distribution Title</TableHead>
              <TableHead>Created at</TableHead>
              <TableHead>Associated Data service</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {distributions.map((distribution) => (
              <TableRow key={distribution["@id"].slice(0, 20)}>
                <TableCell>
                  <Badge variant="info">
                    {distribution["@id"].slice(9, 29) + "[...]"}
                  </Badge>
                </TableCell>
                <TableCell>
                  {distribution.title ? distribution.title : "undefined"}
                </TableCell>
                <TableCell>
                  <ListItemDate>
                    {dayjs(distribution.issued).format("DD/MM/YYYY - HH:mm")}
                  </ListItemDate>
                </TableCell>
                <TableCell>
                  <Button variant="default">
                    <Link
                      to="/catalog/$catalogId/data-service/$dataserviceId"
                      params={{
                        catalogId: catalogId,
                        dataserviceId: distribution.accessService["@id"],
                      }}
                    >
                      See dataservice
                    </Link>
                  </Button>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>

      <div>
        <Heading level="h5"> ODRL Policies </Heading>
        <div className="container-policies flex flex-wrap gap-4">
          {policies.map((policy) => (
            <List className=" border border-white/30 bg-white/10 px-4 py-2 rounded-md justify-start">
              <div className="flex">
                <Heading level="h5" className="flex gap-3">
                  <div>Policy with ID</div>
                  <Badge variant="info" className="h-6">
                    {policy["@id"].slice(9, 29) + "[...]"}
                  </Badge>
                </Heading>
              </div>
              <ListItem>
                <ListItemKey>Policy Target</ListItemKey>
                <p>{policy["@type"]}</p>
              </ListItem>

              <ListItem>
                <ListItemKey> Profile</ListItemKey>
                <p className="whitespace-normal">
                  {" "}
                  {JSON.stringify(policy.profile)}
                </p>
              </ListItem>
              <ListItem>
                <ListItemKey> Target</ListItemKey>
                <p> {policy.target.slice(9)}</p>
              </ListItem>
              <div className="h-5"></div>
              <Heading level="h6"> ODRL CONTENT</Heading>

              <div className="flex flex-col gap-2 mb-2">
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
            </List>
          ))}
        </div>
      </div>
      <div>
        <Heading level="h5">Create new odrl policy </Heading>
        <div className="div flex gap-2">
          <Button policy="permission">
            {" "}
            <Plus />
            Add permission{" "}
          </Button>
          <Button policy="obligation">
            {" "}
            <Plus />
            Add obligation{" "}
          </Button>
          <Button policy="prohibition">
            <Plus /> Add prohibition{" "}
          </Button>
        </div>
        <div className="h-6"> </div>
        <div>
          <Accordion type="single" collapsible className="w-fit min-w-[800px]">
            <AccordionItem
              value="item-1"
              className="bg-success-500/10 border border-success-600/20"
            >
              <AccordionTrigger className="text-white/60 flex bg-success-400/10 uppercase">
                <div className="flex items-center w-full">
                  <p>permission</p>
                  <Button
                    variant="ghost_destructive"
                    size="icon"
                    className="ml-2"
                  >
                    <Trash className="mb-0.5" />
                  </Button>
                </div>
              </AccordionTrigger>
              <AccordionContent>
                {/* <FormLabel className="">Actions</FormLabel> */}
                <p className="mb-2"> Action: </p>
                <Select>
                  <SelectTrigger className="w-[240px]">
                    <SelectValue placeholder="Select an action" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="light">Read</SelectItem>
                    <SelectItem value="dark">Analyze</SelectItem>
                    <SelectItem value="system">Share</SelectItem>
                  </SelectContent>
                </Select>
                <div className="h-6"> </div>
                <p className="mb-2"> Constraints: </p>
                <div className="flex gap-3">
                  <Select>
                    <div className="flex flex-col">
                      <p className="text-xs text-gray-400 mb-2">
                        {" "}
                        Left Operand:{" "}
                      </p>
                      <SelectTrigger className="w-[240px]">
                        <SelectValue placeholder="Select an item" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="light">Date</SelectItem>
                        <SelectItem value="dark">User</SelectItem>
                        <SelectItem value="system">Location</SelectItem>
                      </SelectContent>
                    </div>
                  </Select>
                  <Select>
                    <div className="flex flex-col">
                      <p className="text-xs text-gray-400 mb-2"> Operator: </p>
                      <SelectTrigger className="w-[140px]">
                        <SelectValue placeholder="Select an operator" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="light">Date</SelectItem>
                        <SelectItem value="dark">User</SelectItem>
                        <SelectItem value="system">Location</SelectItem>
                      </SelectContent>
                    </div>
                  </Select>

                  <Select>
                    <div className="flex flex-col">
                      <p className="text-xs text-gray-400 mb-2">
                        {" "}
                        Right Operand:{" "}
                      </p>
                     <Input/>
                    </div>
                  </Select>
                </div>
              </AccordionContent>
            </AccordionItem>
          </Accordion>
        </div>
        <div className="h-6"> </div>
        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)}>
            <FormField
              disabled={isPending}
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
            <Button type="submit">
              Enviar {isPending && <span>- loading...</span>}
            </Button>
          </form>
        </Form>
      </div>
    </div>
  );
}

export const Route = createFileRoute("/catalog/$catalogId/dataset/$datasetId")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
