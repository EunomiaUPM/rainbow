import { createFileRoute } from "@tanstack/react-router";
import { useGetDataServiceById, getDataServiceByIdOptions } from "shared/src/data/catalog-queries.ts";
import dayjs from "dayjs";
import Heading from "shared/src/components/ui/heading";
import { Badge } from "shared/src/components/ui/badge";
import { List, ListItem, ListItemDate, ListItemKey } from "shared/src/components/ui/list";
import { formatUrn } from "shared/src/lib/utils.ts";

function RouteComponent() {
  const { dataServiceId } = Route.useParams();
  const { data: dataService } = useGetDataServiceById(dataServiceId);
  return (
    <div className="space-y-4 pb-4">
      <Heading level="h3" className="flex gap-2 items-center">
        Data service info with id
        <Badge variant="info" size="lg">
          {" "}
          {formatUrn(dataService.id)}
        </Badge>{" "}
      </Heading>
      <div className="gridColsLayout">
        <List className="text-sm">
          <ListItem>
            <ListItemKey>Data service title</ListItemKey>
            <p>{dataService.dctTitle}</p>
          </ListItem>
          <ListItem>
            <ListItemKey>Data service creation date</ListItemKey>
            <ListItemDate>{dayjs(dataService.dctIssued).format("DD/MM/YYYY - HH:mm")}</ListItemDate>
          </ListItem>
          <ListItem>
            <ListItemKey>Data service endpoint URL</ListItemKey>
            <p>{dataService.dcatEndpointUrl}</p>
          </ListItem>
          <ListItem>
            <ListItemKey>Data service description</ListItemKey>
            <p>{dataService.dcatEndpointDescription}</p>
          </ListItem>
        </List>
      </div>
    </div>
  );
}

export const Route = createFileRoute("/catalog/$catalogId/data-service/$dataServiceId")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
  loader: ({ context: { queryClient, api_gateway }, params: {dataServiceId} }) => {
      if (!api_gateway) return;
      return queryClient.ensureQueryData(getDataServiceByIdOptions(api_gateway, dataServiceId));
  },
});
