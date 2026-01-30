import { createFileRoute } from "@tanstack/react-router";
import dayjs from "dayjs";
import Heading from "shared/src/components/ui/heading";
import { Badge } from "shared/src/components/ui/badge";
import { List, ListItem, ListItemDate, ListItemKey } from "shared/src/components/ui/list";
import { useGetConnectorInstancesByDistribution } from "shared/src/data/connector-queries.ts";
import { useGetDistributionById } from "shared/src/data/catalog-queries.ts";

function RouteComponent() {
  const { distributionId } = Route.useParams();
  const { data: distribution } = useGetDistributionById(distributionId);
  const { data: connector } = useGetConnectorInstancesByDistribution(distributionId);

  return (
    <div className="space-y-4 pb-4">
      <Heading level="h3" className="flex gap-2 items-center">
        Distribution info with id
        <Badge variant="info" size="lg">
          {" "}
          {distribution?.id.slice(9, 29) + "[...]"}
        </Badge>{" "}
      </Heading>
      <div>
        <List className="text-sm">
          <ListItem>
            <ListItemKey>Distribution title</ListItemKey>
            <p>{distribution?.dctTitle}</p>
          </ListItem>
          <ListItem>
            <ListItemKey>Distribution creation date</ListItemKey>
            <ListItemDate>
              {dayjs(distribution?.dctIssued).format("DD/MM/YYYY - HH:mm")}
            </ListItemDate>
          </ListItem>
        </List>
      </div>
      <div>
        <Heading level="h5">Connector Instance info</Heading>
        <List className="text-sm">
          <ListItem>
            <ListItemKey>Connector Instance infor</ListItemKey>
            <ListItemDate>
              <pre>{JSON.stringify(connector, null, 2)}</pre>
            </ListItemDate>
          </ListItem>
        </List>
      </div>
    </div>
  );
}

export const Route = createFileRoute("/catalog/$catalogId/distribution-connector/$distributionId")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
});
