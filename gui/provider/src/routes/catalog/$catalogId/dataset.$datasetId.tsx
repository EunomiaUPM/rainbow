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
import {
  Policy,
  policyVariants,
  PolicyConstraint,
  PolicyItemContainer,
  PolicyItem,
  PolicyItemKey,
  PolicyItemValue,
  PolicyConstraintsContainer,
  PolicyConstraintsWrapper,
} from "shared/src/components/ui/policy";

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
            <List className=" border border-white/30 bg-white/10 px-4 py-2 rounded-md ">
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
                <p>{policy.target?.slice(9)}</p>
              </ListItem>

              <ListItem>
                <ListItemKey> Profile</ListItemKey>
                <p> {JSON.stringify(policy.profile)}</p>
              </ListItem>
              <ListItem>
                <ListItemKey> Target</ListItemKey>
                <p> {JSON.stringify(policy.target.slice(9, 29) + "[...]")}</p>
              </ListItem>
              <div className="h-5"></div>
              <Heading level="h6"> ODRL CONTENT</Heading>

              <div className="flex flex-col gap-2">
                {/* {policy.permission.map((perm) => ( */}
                  <Policy className="" variant="permission">
                    <Heading level="h6" className="uppercase text-success-200">
                      permission
                    </Heading>
                    {policy.permission.length === 0 ? (
                      <div> No permissions </div>
                    ) : (
                      <div className="flex flex-col">
                        {policy.permission.map((perm) => (
                          <PolicyItemContainer>
                            <PolicyItem>
                              <PolicyItemKey>action:</PolicyItemKey>
                              <PolicyItemValue>{perm.action}</PolicyItemValue>
                            </PolicyItem>
                            <PolicyItem>
                              <PolicyItemKey>constraints:</PolicyItemKey>
                              <PolicyConstraintsContainer>
                                <PolicyConstraint type="rightOperand">
                                  {" "}
                                  {JSON.stringify(
                                    perm.constraint[0].rightOperand
                                  )}
                                </PolicyConstraint>
                                <PolicyConstraint type="operator">
                                  {" "}
                                  {JSON.stringify(perm.constraint[0].operator)}
                                </PolicyConstraint>
                                <PolicyConstraint type="leftOperand">
                                  {JSON.stringify(
                                    perm.constraint[0].leftOperand
                                  )}
                                </PolicyConstraint>
                              </PolicyConstraintsContainer>
                            </PolicyItem>
                          </PolicyItemContainer>
                        ))}
                      </div>
                    )}
                  </Policy>
                
                <Policy className="" variant="obligation">
                  <Heading level="h6" className="uppercase text-warn-300">
                    obligation
                  </Heading>

                  {/* COMPROBACIÓN SI HAY ALGUNA OBLIGACIÓN (ARRAY VACIO O NO) */}
                  {policy.obligation.length === 0 ? (
                    <div> No obligations </div>
                  ) : (
                         <div className="flex flex-col">
                      {policy.obligation.map((obl) => (
                        <PolicyItemContainer>
                          <PolicyItem>
                            <PolicyItemKey>action:</PolicyItemKey>
                            <PolicyItemValue>{obl.action}</PolicyItemValue>
                          </PolicyItem>
                          <PolicyItem>
                            <PolicyItemKey>constraints:</PolicyItemKey>
                            <PolicyConstraintsWrapper>
                              {/* comprobar que el constraint no sea null o un array vacio. 
                                Si no lo es, pintar los rightoperand, leftoperand, operator */}
                              {obl.constraint == null ||
                              obl.constraint.length === 0 ? (
                                "No constraints"
                              ) : (
                                <>
                                {obl.constraint.map((constr) => (
                                <PolicyConstraintsContainer>
                                    {console.log(constr, "brrrr brrrr")}
                                  <PolicyConstraint type="rightOperand">
                                    {JSON.stringify(
                                      constr.rightOperand
                                    )}
                                  </PolicyConstraint>
                                  <PolicyConstraint type="operator">
                                    {JSON.stringify(constr.operator)}
                                  </PolicyConstraint>
                                  <PolicyConstraint type="leftOperand">
                                    {JSON.stringify(
                                      constr.leftOperand
                                    )}
                                  </PolicyConstraint>
                                </PolicyConstraintsContainer>
                                ))}
                                </>
                              )}
                            </PolicyConstraintsWrapper>
                          </PolicyItem>
                        </PolicyItemContainer>
                      ))}
                    </div>
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
                         <div className="flex flex-col">
                      {policy.prohibition.map((prohib) => (
                        <PolicyItemContainer>
                          {/* // <div> {JSON.stringify(policy.prohibition)}</div> */}
                          <PolicyItem>
                            <PolicyItemKey>action:</PolicyItemKey>
                            <PolicyItemValue>{prohib.action}</PolicyItemValue>
                          </PolicyItem>
                          <PolicyItem>
                            <PolicyItemKey>constraints:</PolicyItemKey>
                            <PolicyConstraintsWrapper>
                              {/* comprobar que el constraint no sea null o un array vacio. 
                                Si no lo es, pintar los rightoperand, leftoperand, operator */}
                              {prohib.constraint == null ||
                              prohib.constraint.length === 0 ? (
                                "No constraints"
                              ) : (
                                <>
                                  {/* {console.log(prohib.constraint, " prohib constr")} */}
                                  {prohib.constraint.map((constr) => (
                                    <PolicyConstraintsContainer>
                                      <PolicyConstraint type="rightOperand">
                                        {/* {console.log(constr, "constrrrr")} */}
                                        {JSON.stringify(constr.rightOperand)}
                                      </PolicyConstraint>
                                      <PolicyConstraint type="operator">
                                        {" "}
                                        {JSON.stringify(constr.operator)}
                                      </PolicyConstraint>
                                      <PolicyConstraint type="leftOperand">
                                        {JSON.stringify(constr.leftOperand)}
                                      </PolicyConstraint>
                                    </PolicyConstraintsContainer>
                                  ))}
                                </>
                              )}
                            </PolicyConstraintsWrapper>
                          </PolicyItem>
                        </PolicyItemContainer>
                      ))}
                    </div>
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
