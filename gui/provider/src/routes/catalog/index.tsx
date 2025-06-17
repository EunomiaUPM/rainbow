import { createFileRoute, Link } from "@tanstack/react-router";
import dayjs from "dayjs";
import { ExternalLink } from "lucide-react";
import {
  Table,
  TableBody,
  p,
  TableHead,
  TableHeader,
  TableRow,
  TableCell,
} from "shared/src/components/ui/table";
import { getCatalogsOptions, useGetCatalogs } from "@/data/catalog-queries.ts";
import Heading from "../../../../shared/src/components/ui/heading.tsx";
import { List, ListItem, ListItemKey } from "shared/src/components/ui/list.tsx";
import {
  Button,
  ButtonVariants,
} from "@/../../shared/src/components/ui/button.tsx";
import { Input } from "@/../../shared/src/components/ui/input.tsx";
import {
  Form,
  FormField,
  FormItem,
  FormMessage,
  FormDescription,
  FormControl,
  FormLabel,
} from "@/../../shared/src/components/ui/form.tsx";
import  {Textarea } from "@/../../shared/src/components/ui/textarea.tsx"

const RouteComponent = () => {
  const { data: catalogs } = useGetCatalogs();
  return (
    <div className="space-y-4">
      {/* <h1 className="text-xl font-bold">Catalogs</h1> */}
      <Heading level="h3" className="flex gap-2 items-center">
        Main Catalog with id : {catalogs["@id"]}
      </Heading>
      <div>
        <Heading level="h5">Main Catalog info: </Heading>
        <List className="text-sm">
          <ListItem>
            <ListItemKey>Catalog title</ListItemKey>
            <p>{catalogs.title}</p>
          </ListItem>
          <ListItem>
            <ListItemKey>Catalog participant id</ListItemKey>
            <p>{catalogs.participantId}</p>
          </ListItem>
          <ListItem>
            <ListItemKey>Catalog homepage</ListItemKey>
            <p>{catalogs.homepage}</p>
          </ListItem>
          <ListItem>
            <ListItemKey>Catalog creation date</ListItemKey>
            <p>{dayjs(catalogs.issued).format("DD/MM/YYYY - HH:mm")}</p>
          </ListItem>
        </List>
      </div>

      <div>
        <Heading level="h5">Datasets</Heading>
        <div className="pb-3 w-3/5">
          <Input type="search"></Input>
        </div>
        <Table className="text-sm">
          <TableHeader>
            <TableRow>
              <TableHead>Title</TableHead>
              <TableHead>Catalog Id</TableHead>
              <TableHead>Provider ID</TableHead>
              {/* <TableHead>CreatedAt</TableHead> */}
              <TableHead>Actions</TableHead>
              <TableHead>Link</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow key="urn:uuid:c4d4449d-a">
              <TableCell>
                <p className="text-18"> Dataset #1 </p>
                <p className="text-gray-400">
                  {" "}
                  <i>Created at: 23/6/25 16:34 </i>
                </p>
              </TableCell>
              <TableCell>urn:uuid:c4d4449d-a...</TableCell>
              <TableCell> urn:uuid:c4dsf49d-b...</TableCell>

              <TableCell>
                <Button>
                  <Link
                    to="/catalog/catalogId-mockup"
                    // params={{catalogId: catalog["@id"]}}
                  >
                    Create policy
                  </Link>
                </Button>
              </TableCell>
              <TableCell>
                <Button>
                  <Link
                    to="/catalog/catalogId-mockup"
                    // params={{catalogId: catalog["@id"]}}
                  >
                    See dataset
                  </Link>
                </Button>
              </TableCell>
            </TableRow>
            <TableRow key="urn:uuid:c4d4449d-a">
              <TableCell>
                <p className="text-18"> Dataset #1 </p>
                <p className="text-gray-400">
                  {" "}
                  <i>Created at: 23/6/25 16:34 </i>
                </p>
              </TableCell>
              <TableCell>urn:uuid:c4d4449d-a...</TableCell>
              <TableCell> urn:uuid:c4dsf49d-b...</TableCell>

              <TableCell>
                <Button>
                  <Link
                    to="/catalog/$catalogId"
                    // params={{catalogId: catalog["@id"]}}
                  >
                    Create policy
                  </Link>
                </Button>
              </TableCell>
              <TableCell>
                <Button>
                  <Link
                    to="/catalog/$catalogId"
                    // params={{catalogId: catalog["@id"]}}
                  >
                    See dataset
                  </Link>
                </Button>
              </TableCell>
            </TableRow>

            <TableRow key="urn:uuid:c4d4449d-a">
              <TableCell>
                <p className="text-18"> Dataset #1 </p>
                <p className="text-gray-400">
                  {" "}
                  <i>Created at: 23/6/25 16:34 </i>
                </p>
              </TableCell>
              <TableCell>urn:uuid:c4d4449d-a...</TableCell>
              <TableCell> urn:uuid:c4dsf49d-b...</TableCell>

              <TableCell>
                <Button>
                  <Link
                    to="/catalog/$catalogId"
                    // params={{catalogId: catalog["@id"]}}
                  >
                    Create policy
                  </Link>
                </Button>
              </TableCell>
              <TableCell>
                <Button>
                  <Link
                    to="/catalog/$catalogId"
                    // params={{catalogId: catalog["@id"]}}
                  >
                    See dataset
                  </Link>
                </Button>
              </TableCell>
            </TableRow>
          
            {/* ver creacion de ODRL */}
            {/* Nada de esto funciona asi que hago mockup */}
            {/* <div>
              <h2>Create new odrl policy</h2>
              <Form>
                <form>
                  <FormField
                    disabled=""
                    control=""
                    name="odrl"
                   ></FormField>
                      <FormItem>
                        <FormLabel>Odrl</FormLabel>
                        <FormControl>
                          <Textarea />
                        </FormControl>
                        <FormDescription>
                          Provide the ODRL policy content
                        </FormDescription>
                        <FormMessage />
                      </FormItem>
                  
                
                  <Button type="submit">
                    Enviar
                    {isPending && <span>- loading...</span>}
                  </Button>
                </form>
              </Form>
            </div> */}

            {/* {catalogs.catalog.map((catalog) => (
                            <TableRow key={catalog["@id"].slice(0, 20)}>
                                <p>
                                    {catalog["@id"].slice(0, 20) + "..."}
                                </p>
                                <p>
                                    {catalog.title?.slice(0, 20) + "..."}
                                </p>
                                <p>{catalog.participantId}</p>
                                <p>
                                    {dayjs(catalog.issued).format("DD/MM/YYYY - HH:mm")}
                                </p>
                                <p>
                                    <Link
                                        to="/catalog/$catalogId"
                                        params={{catalogId: catalog["@id"]}}
                                    >
                                        <ExternalLink size={12} className="text-pink-600"/>
                                    </Link>
                                </p>
                            </TableRow>
                        ))} */}
          </TableBody>
        </Table>
          <Form>
            <form>
{/* <FormField> </FormField> Cannot read properties of undefined (reading "_names") */}

            </form>
            </Form>

      </div>
    </div>
  );
};

export const Route = createFileRoute("/catalog/")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
  loader: async ({ context: { queryClient } }) => {
    return await queryClient.ensureQueryData(getCatalogsOptions());
  },
});
