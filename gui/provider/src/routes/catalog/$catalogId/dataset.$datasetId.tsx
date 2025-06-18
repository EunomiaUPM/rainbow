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
      <Heading level="h3">Dataset info with id: {dataset["@id"]}</Heading>

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
        <h2>Distributions</h2>
        <Table className="text-sm">
          <TableHeader>
            <TableRow>
              <TableHead>Distribution Id</TableHead>
              <TableHead>Distribution Title</TableHead>
              <TableHead>CreatedAt</TableHead>
              <TableHead>Associated Data service</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {distributions.map((distribution) => (
              <TableRow key={distribution["@id"].slice(0, 20)}>
                <TableCell>
                  {distribution["@id"].slice(0, 20) + "..."}
                </TableCell>
                <TableCell>
                  {distribution.title?.slice(0, 20) + "..."}
                </TableCell>
                <TableCell>
                  {dayjs(distribution.issued).format("DD/MM/YYYY - HH:mm")}
                </TableCell>
                <TableCell>
                  <Link
                    to="/catalog/$catalogId/data-service/$dataserviceId"
                    params={{
                      catalogId: catalogId,
                      dataserviceId: distribution.accessService["@id"],
                    }}
                  >
                    <ExternalLink size={12} className="text-pink-600" />
                  </Link>
                </TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      </div>

      <div>
     
             <Heading level="h5"> ODRL Policies </Heading>
        <div className="container-policies flex gap-4">
            {policies.map((policy) => (
              <div className="border border-white/60 bg-white/10 p-4 rounded-md ">
                  <Heading level="h6" >Policy Id</Heading>
                <p>{policy["@id"].slice(0, 20) + "..."}</p>
                  <Heading level="h6">Policy Target</Heading>
                <p>{policy.target?.slice(0, 20) + "..."}</p>
                <Heading level="h6"> ODRL CONTENT</Heading>

                  <div>
            
                    {policy.permission.map((perm) => (
                      <div className="flex gap-3">
                        <b>permission</b>
                        <div>
                          <p > {perm.action}</p>
                          <div className="flex gap-3">
                            <p >
                              {" "}
                              {JSON.stringify(perm.constraint[0].leftOperand)}
                            </p>
                            <p >
                              {" "}
                              {JSON.stringify(perm.constraint[0].operator)}
                            </p>
                            <p>
                              {" "}
                              {JSON.stringify(perm.constraint[0].rightOperand)}
                            </p>
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>
                  <div className="flex gap-3">
                    {/* TO-DO: HACER LO MISMO CON OBLIGATION Y PROHIBITION QUE CON 
                    PERMISSION */}
                    <b>obligation</b>
                    <div> {JSON.stringify(policy.obligation)}</div>
                  </div>
                  <div className="flex gap-3">
                    <b>prohibition</b>
                    <div> {JSON.stringify(policy.prohibition)}</div>
                  </div>
                  <div className="flex gap-3">
                    <b>profile</b>
                    <p> {JSON.stringify(policy.profile)}</p>
                  </div>
                  <div className="flex gap-3">
                    <b>target</b> <p> {JSON.stringify(policy.target)}</p>
                  </div>
             
              </div>
            ))}
        </div>
      </div>
      <div>
        <h2>Create new odrl policy</h2>
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
