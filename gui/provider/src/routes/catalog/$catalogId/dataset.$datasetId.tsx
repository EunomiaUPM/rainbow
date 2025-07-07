import { createFileRoute, Link, useRouterState } from "@tanstack/react-router";
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
import { ArrowRight, Plus, Trash } from "lucide-react";
import { useGetPoliciesByDatasetId } from "shared/src/data/policy-queries.ts";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
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
  ListItemDate,
  ListItemKey,
} from "shared/src/components/ui/list";
import { Badge } from "shared/src/components/ui/badge";
import PolicyComponent from "shared/src/components/PolicyComponent.tsx";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "shared/src/components/ui/accordion";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "shared/src/components/ui/select.tsx";
import { Input } from "shared/src/components/ui/input";
import {
  Drawer,
  DrawerClose,
  DrawerContent,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger,
} from "shared/src/components/ui/drawer";

type Inputs = {
  odrl: string;
};

function RouteComponent() {
  const routerState = useRouterState();
  let paths = routerState.location.pathname.split("/");

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
    <div className="space-y-8 pb-4">
      <Heading level="h3" className="flex gap-2 items-center">
        Dataset with id
        <Badge variant="info" size="lg">
          {" "}
          {dataset["@id"].slice(9, 29) + "[...]"}
        </Badge>
      </Heading>
      <div className="gridColsLayout">
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
                  <Link
                    to="/catalog/$catalogId/data-service/$dataserviceId"
                    params={{
                      catalogId: catalogId,
                      dataserviceId: distribution.accessService["@id"],
                    }}
                  >
                    <Button variant="link">
                      See dataservice
                      <ArrowRight />
                    </Button>
                  </Link>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>

      <div>
        <div className=" flex flex-row mb-2 items-center">
          <Heading level="h5" className="mb-0">
            {" "}
            ODRL Policies{" "}
          </Heading>
          <Drawer direction={"right"}>
            <DrawerTrigger>
              <Button variant="default" size="sm" className="mb-1 ml-3">
                Add ODRL policy
                <Plus className="" />
              </Button>
            </DrawerTrigger>
            <DrawerContent>
              <DrawerHeader className="px-8">
                <DrawerTitle>
                  <Heading level="h4" className="text-curren mb-0 ">
                    New ODRL Policy
                  </Heading>
                  <p className="font-normal text-brand-sky">
                    {" "}
                    for Dataset
                    <Badge variant="info" size="sm" className="ml-2">
                      {" "}
                      {dataset["@id"].slice(9, 29) + "[...]"}
                    </Badge>
                  </p>
                </DrawerTitle>
              </DrawerHeader>
              <div className="px-8 overflow-y-auto ">
                <p className="mb-5">
                  Add permissions, obligations and prohibitions to apply to the
                  dataset. Select the action and constraints for each
                  permission, obligation or prohibition.
                </p>
                <div className="flex flex-col gap-4">
                  <Accordion type="single" collapsible className="w-full">
                    <AccordionItem
                      value="item-1"
                      className="bg-success-500/10 border border-success-600/20"
                    >
                      <AccordionTrigger className="text-white/70 flex bg-success-400/25 uppercase overflow-hidden rounded-md data-[state=open]:rounded-b-none">
                        <div className="flex items-center w-full">
                          <p className="text-current">permission</p>
                        </div>
                      </AccordionTrigger>
                      <AccordionContent className="relative">
                        <Button
                          className="border-b border-white/15"
                          policy="permission"
                          variant="outline"
                          size="xs"
                        >
                          <Plus />
                          Add permission
                        </Button>
                        <div>
                          <div className="policy-item-create">
                            <div className="flex justify-between">
                              <p className="mb-2"> Action: </p>
                              <Button
                                variant="icon_destructive"
                                size="xs"
                                className="ml-4 border"
                              >
                                <Trash className="mb-0.5" />
                                Remove permission
                              </Button>
                            </div>
                            <Select>
                              <SelectTrigger className="w-[240px]">
                                <SelectValue placeholder="Select action" />
                              </SelectTrigger>
                              <SelectContent>
                                <SelectItem value="light">Read</SelectItem>
                                <SelectItem value="dark">Analyze</SelectItem>
                                <SelectItem value="system">Share</SelectItem>
                              </SelectContent>
                            </Select>
                            <div className="h-6"></div>
                            <p className="mb-2"> Constraints: </p>
                            <div className="flex flex-col gap-2">
                              <div className="constraint-create flex gap-3">
                                <Select>
                                  <div className="flex flex-col">
                                    <p className="text-xs text-gray-400 mb-1">
                                      {" "}
                                      Left Operand:{" "}
                                    </p>
                                    <SelectTrigger className="w-[180px]">
                                      <SelectValue placeholder="Select item" />
                                    </SelectTrigger>
                                    <SelectContent>
                                      <SelectItem value="light">
                                        Date
                                      </SelectItem>
                                      <SelectItem value="dark">User</SelectItem>
                                      <SelectItem value="system">
                                        Location
                                      </SelectItem>
                                    </SelectContent>
                                  </div>
                                </Select>
                                <Select>
                                  <div className="flex flex-col">
                                    <p className="text-xs text-gray-400 mb-1">
                                      {" "}
                                      Operator:{" "}
                                    </p>
                                    <SelectTrigger className="w-[140px]">
                                      <SelectValue placeholder="Select operator" />
                                    </SelectTrigger>
                                    <SelectContent>
                                      <SelectItem value="light">eq</SelectItem>
                                      <SelectItem value="dark">neq</SelectItem>
                                      <SelectItem value="system">
                                        gteq
                                      </SelectItem>
                                    </SelectContent>
                                  </div>
                                </Select>
                                <Select>
                                  <div className="flex flex-col">
                                    <p className="text-xs text-gray-400 mb-1">
                                      {" "}
                                      Right Operand:{" "}
                                    </p>
                                    <Input placeholder="Type value" />
                                  </div>
                                </Select>
                                <div className="flex flex-col">
                                  <p className="text-xs text-gray-400 mb-1">
                                    Unity
                                  </p>
                                  <p className="mt-2">Unity</p>
                                </div>
                                <Button
                                  variant="icon_destructive"
                                  size="icon_sm"
                                  className="ml-2 self-end mb-1"
                                >
                                  <Trash className="mb-0.5" />
                                </Button>
                              </div>
                            </div>
                            <Button
                              size="xs"
                              variant="outline"
                              className="mt-3"
                            >
                              <Plus />
                              Add constraint
                            </Button>
                          </div>
                          <div className="policy-item-create">
                            <div className="flex justify-between">
                              <p className="mb-2"> Action: </p>
                              <Button
                                variant="icon_destructive"
                                size="xs"
                                className="ml-4"
                              >
                                <Trash className="mb-0.5" />
                                Remove permission
                              </Button>
                            </div>
                            <Select>
                              <SelectTrigger className="w-[180px]">
                                <SelectValue placeholder="Select action" />
                              </SelectTrigger>
                              <SelectContent>
                                <SelectItem value="light">Read</SelectItem>
                                <SelectItem value="dark">Analyze</SelectItem>
                                <SelectItem value="system">Share</SelectItem>
                              </SelectContent>
                            </Select>
                            <div className="h-6"></div>
                            <p className="mb-2"> Constraints: </p>
                            <div className="flex flex-col gap-2">
                              <div className="constraint-create flex gap-3">
                                <Select>
                                  <div className="flex flex-col">
                                    <p className="text-xs text-gray-400 mb-1">
                                      {" "}
                                      Left Operand:{" "}
                                    </p>
                                    <SelectTrigger className="w-[180px]">
                                      <SelectValue placeholder="Select an item" />
                                    </SelectTrigger>
                                    <SelectContent>
                                      <SelectItem value="light">
                                        Date
                                      </SelectItem>
                                      <SelectItem value="dark">User</SelectItem>
                                      <SelectItem value="system">
                                        Location
                                      </SelectItem>
                                    </SelectContent>
                                  </div>
                                </Select>
                                <Select>
                                  <div className="flex flex-col">
                                    <p className="text-xs text-gray-400 mb-1">
                                      {" "}
                                      Operator:{" "}
                                    </p>
                                    <SelectTrigger className="w-[140px]">
                                      <SelectValue placeholder="Select an operator" />
                                    </SelectTrigger>
                                    <SelectContent>
                                      <SelectItem value="light">
                                        Date
                                      </SelectItem>
                                      <SelectItem value="dark">User</SelectItem>
                                      <SelectItem value="system">
                                        Location
                                      </SelectItem>
                                    </SelectContent>
                                  </div>
                                </Select>
                                <Select>
                                  <div className="flex flex-col">
                                    <p className="text-xs text-gray-400 mb-1">
                                      {" "}
                                      Right Operand:{" "}
                                    </p>
                                    <Input placeholder="Type value" />
                                  </div>
                                </Select>
                                <div className="flex flex-col">
                                  <p className="text-xs text-gray-400 mb-1">
                                    Unity
                                  </p>
                                  <p className="mt-2">Unity</p>
                                </div>
                                <Button
                                  variant="icon_destructive"
                                  size="icon_sm"
                                  className="ml-2 self-end mb-1"
                                >
                                  <Trash className="mb-0.5" />
                                </Button>
                              </div>
                              <div className="constraint-create flex gap-3">
                                <Select>
                                  <div className="flex flex-col">
                                    <p className="text-xs text-gray-400 mb-1">
                                      {" "}
                                      Left Operand:{" "}
                                    </p>
                                    <SelectTrigger className="w-[180px]">
                                      <SelectValue placeholder="Select an item" />
                                    </SelectTrigger>
                                    <SelectContent>
                                      <SelectItem value="light">
                                        Date
                                      </SelectItem>
                                      <SelectItem value="dark">User</SelectItem>
                                      <SelectItem value="system">
                                        Location
                                      </SelectItem>
                                    </SelectContent>
                                  </div>
                                </Select>
                                <Select>
                                  <div className="flex flex-col">
                                    <p className="text-xs text-gray-400 mb-1">
                                      {" "}
                                      Operator:{" "}
                                    </p>
                                    <SelectTrigger className="w-[140px]">
                                      <SelectValue placeholder="Select an operator" />
                                    </SelectTrigger>
                                    <SelectContent>
                                      <SelectItem value="light">
                                        Date
                                      </SelectItem>
                                      <SelectItem value="dark">User</SelectItem>
                                      <SelectItem value="system">
                                        Location
                                      </SelectItem>
                                    </SelectContent>
                                  </div>
                                </Select>
                                <Select>
                                  <div className="flex flex-col">
                                    <p className="text-xs text-gray-400 mb-1">
                                      {" "}
                                      Right Operand:{" "}
                                    </p>
                                    <Input placeholder="Type value" />
                                  </div>
                                </Select>
                                <div className="flex flex-col">
                                  <p className="text-xs text-gray-400 mb-1">
                                    Unity
                                  </p>
                                  <p className="mt-2">Unity</p>
                                </div>
                                <Button
                                  variant="icon_destructive"
                                  size="icon_sm"
                                  className="ml-2 self-end mb-1"
                                >
                                  <Trash className="mb-0.5" />
                                </Button>
                              </div>
                            </div>
                            <Button
                              size="xs"
                              variant="outline"
                              className="mt-3"
                            >
                              <Plus />
                              Add constraint
                            </Button>
                          </div>
                        </div>
                      </AccordionContent>
                    </AccordionItem>
                  </Accordion>
                  <Accordion type="single" collapsible className="w-full">
                    <AccordionItem
                      value="item-1"
                      className="bg-warn-500/10 border border-warn-600/20"
                    >
                      <AccordionTrigger className="text-white/70 flex bg-warn-400/25 uppercase overflow-hidden rounded-md data-[state=open]:rounded-b-none">
                        <div className="flex items-center w-full">
                          <p className="text-current">obligation</p>
                        </div>
                      </AccordionTrigger>
                      <AccordionContent className="relative">
                        content obligation
                      </AccordionContent>
                    </AccordionItem>
                  </Accordion>
                  <Accordion type="single" collapsible className="w-full">
                    <AccordionItem
                      value="item-1"
                      className="bg-danger-500/10 border border-danger-600/20"
                    >
                      <AccordionTrigger className="text-white/70 flex bg-danger-500/25 uppercase overflow-hidden rounded-md data-[state=open]:rounded-b-none">
                        <div className="flex items-center w-full">
                          <p className="text-current">prohibition</p>
                        </div>
                      </AccordionTrigger>
                      <AccordionContent className="relative">
                        content prohibition
                      </AccordionContent>
                    </AccordionItem>
                  </Accordion>
                </div>
                <div className="h-6"></div>
                <p className="mb-5">
                  ...or paste the ODRL policy content directly in the textarea
                  below.
                </p>
                <Form {...form}>
                  <form onSubmit={form.handleSubmit(onSubmit)}>
                    <FormField
                      disabled={isPending}
                      control={form.control}
                      name="odrl"
                      render={({ field }) => (
                        <FormItem>
                          {/* <FormLabel>Odrl</FormLabel> */}
                          <FormControl>
                            <Textarea className="h-24" {...field} />
                          </FormControl>
                          <FormDescription>
                            <i>Type or paste the ODRL policy content here.</i>
                          </FormDescription>
                          <FormMessage />
                        </FormItem>
                      )}
                    />
                    {/* <Button type="submit">
                      Enviar {isPending && <span>- loading...</span>}
                    </Button> */}
                  </form>
                </Form>
                <div className="h-6"></div>
              </div>
              <DrawerFooter>
                <DrawerClose className="flex justify-start gap-4">
                  <Button variant="ghost" className="w-40">
                    Cancel
                  </Button>
                  <Button variant="default" className="w-40">
                    Save policy
                  </Button>
                  {/* <Button className="w-40">Add Participant</Button> */}
                </DrawerClose>
              </DrawerFooter>
            </DrawerContent>
          </Drawer>
        </div>
        <div className="gridColsLayout">
          {policies && policies.map((policy) => (
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
        <div className="h-6"></div>
      </div>
    </div>
  );
}

export const Route = createFileRoute("/catalog/$catalogId/dataset/$datasetId")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
