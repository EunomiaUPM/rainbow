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
import {
  Policy,
  PolicyConstraint,
  PolicyItemContainer,
  PolicyItem,
  PolicyItemKey,
  PolicyItemValue,
  PolicyConstraintsContainer,
} from "shared/src/components/ui/policy";
import {useContext} from "react";
import {GlobalInfoContext, GlobalInfoContextType} from "shared/src/context/GlobalInfoContext.tsx";

type Inputs = {
  odrl: string;
};

function RouteComponent() {
    const {catalogId, datasetId} = Route.useParams()
    const {data: dataset} = useGetDatasetById(datasetId)
    const {data: distributions} = useGetDistributionsByDatasetId(datasetId)
    const {data: policies} = useGetPoliciesByDatasetId(datasetId)
    const {mutateAsync: createPolicyAsync, isPending} = usePostNewPolicyInDataset()
    const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!
    const form = useForm<Inputs>({
        defaultValues: {
            odrl: "{\"permission\":[{\"action\":\"use\",\"constraint\":[{\"rightOperand\":\"user\",\"leftOperand\":\"did:web:hola.es\",\"operator\":\"eq\"}]}],\"obligation\":[],\"prohibition\":[]}",
        },
    })
    const onSubmit: SubmitHandler<Inputs> = data => {
        // @ts-ignore
        createPolicyAsync({
            api_gateway,
            datasetId,
            content: {
                offer: data.odrl
            }
        })
        form.reset()
    }

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
        <div className="container-policies flex gap-4">
          {policies.map((policy) => (
            <List className=" w-1/2 border border-white/60 bg-white/10 px-4 py-2 rounded-md ">
              <div className="flex gap-2">
              <Heading level="h5" className="flex gap-3">
                <div>Policy with ID</div>
                <Badge variant="info" className="h-6">{policy["@id"].slice(9, 29) + "[...]"}</Badge>
              </Heading>

             </div>
              <ListItem>
                <ListItemKey>Policy Target</ListItemKey>
                <p>{policy.target?.slice(9)}</p>
              </ListItem>

              <ListItem>
                <ListItemKey> Profile</ListItemKey>
                <p> {JSON.stringify(policy.profile)}</p>
              </ListItem>
              <ListItem>
                <ListItemKey> Target</ListItemKey>
                <p> {JSON.stringify(policy.target.slice(9,29) + "[...]")}</p>
              </ListItem>
              <div className="h-5"></div>
              <Heading level="h6"> ODRL CONTENT</Heading>

              <div className="flex flex-col gap-2">
                {policy.permission.map((perm) => (
                  <Policy className="" variant="permission">
                    <Heading level="h6" className="uppercase text-success-200">
                      permission
                    </Heading>
                    <PolicyItemContainer>
                      <PolicyItem>
                        <PolicyItemKey>action:</PolicyItemKey>
                        <PolicyItemValue>{perm.action}</PolicyItemValue>
                      </PolicyItem>
                      <PolicyItem>
                        <PolicyItemKey>constraint:</PolicyItemKey>
                        <PolicyConstraintsContainer>
                          <PolicyConstraint type="rightOperand">
                            {" "}
                            {JSON.stringify(perm.constraint[0].rightOperand)}
                          </PolicyConstraint>
                          <PolicyConstraint type="operator">
                            {" "}
                            {JSON.stringify(perm.constraint[0].operator)}
                          </PolicyConstraint>
                          <PolicyConstraint type="leftOperand">
                            {JSON.stringify(perm.constraint[0].leftOperand)}
                          </PolicyConstraint>
                        </PolicyConstraintsContainer>
                      </PolicyItem>
                    </PolicyItemContainer>
                  </Policy>
                ))}

                <Policy className="" variant="obligation">
                  <Heading level="h6" className="uppercase text-warn-300">
                    obligation
                  </Heading>
                  {/* COMPROBACIÓN SI HAY ALGUNA OBLIGACIÓN (ARRAY VACIO O NO) */}
                  {policy.obligation.length === 0 ? (
                    <div> No obligations </div>
                  ) : (
                    <>
                      {policy.obligation.map((obl) => (
                        <PolicyItemContainer>
                          {/* // <div> {JSON.stringify(policy.obligation)}</div> */}
                          <PolicyItem>
                            <PolicyItemKey>action:</PolicyItemKey>
                            <PolicyItemValue>{obl.action}</PolicyItemValue>
                          </PolicyItem>
                          <PolicyItem>
                            <PolicyItemKey>constraint:</PolicyItemKey>
                            <PolicyConstraintsContainer>
                              <PolicyConstraint type="rightOperand">
                                {" "}
                                {JSON.stringify(obl.constraint[0].rightOperand)}
                              </PolicyConstraint>
                              <PolicyConstraint type="operator">
                                {" "}
                                {JSON.stringify(obl.constraint[0].operator)}
                              </PolicyConstraint>
                              <PolicyConstraint type="leftOperand">
                                {JSON.stringify(obl.constraint[0].leftOperand)}
                              </PolicyConstraint>
                            </PolicyConstraintsContainer>
                          </PolicyItem>
                        </PolicyItemContainer>
                      ))}
                    </>
                  )}
                </Policy>

                <Policy className="" variant="prohibition">
                  <Heading level="h6" className="uppercase text-danger-400">
                    prohibition
                  </Heading>
                  {/* COMPROBACIÓN SI HAY ALGUNA PROHIBICIÓN (ARRAY VACIO O NO) */}
                  {policy.prohibition.length === 0 ? (
                    <div> No prohibitions </div>
                  ) : (
                    <>
                      {policy.prohibition.map((prohib) => (
                        <PolicyItemContainer>
                          {/* // <div> {JSON.stringify(policy.prohibition)}</div> */}
                          <PolicyItem>
                            <PolicyItemKey>action:</PolicyItemKey>
                            <PolicyItemValue>{prohib.action}</PolicyItemValue>
                          </PolicyItem>
                          <PolicyItem>
                            <PolicyItemKey>constraint:</PolicyItemKey>
                            <PolicyConstraintsContainer>
                              <PolicyConstraint type="rightOperand">
                                {" "}
                                {JSON.stringify(
                                  prohib.constraint[0].rightOperand
                                )}
                              </PolicyConstraint>
                              <PolicyConstraint type="operator">
                                {" "}
                                {JSON.stringify(prohib.constraint[0].operator)}
                              </PolicyConstraint>
                              <PolicyConstraint type="leftOperand">
                                {JSON.stringify(
                                  prohib.constraint[0].leftOperand
                                )}
                              </PolicyConstraint>
                            </PolicyConstraintsContainer>
                          </PolicyItem>
                        </PolicyItemContainer>
                      ))}
                    </>
                  )}
                </Policy>
              </div>
            </List>
          ))}
        </div>
      </div>
      <div>
        <Heading level="h5">Create new odrl policy </Heading>
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