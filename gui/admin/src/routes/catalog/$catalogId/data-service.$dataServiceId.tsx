import { createFileRoute } from "@tanstack/react-router";
import { useGetDataServiceById, getDataServiceByIdOptions } from "shared/src/data/catalog-queries.ts";
import dayjs from "dayjs";
import Heading from "shared/src/components/ui/heading";
import { Badge } from "shared/src/components/ui/badge";
import { List, ListItem, ListItemDate, ListItemKey } from "shared/src/components/ui/list";
import { formatUrn } from "shared/src/lib/utils.ts";
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageHeader } from "shared/src/components/layout/PageHeader";
import { InfoGrid } from "shared/src/components/layout/InfoGrid";
import { PageSection } from "shared/src/components/layout/PageSection";

function RouteComponent() {
  const { dataServiceId } = Route.useParams();
  const { data: dataService } = useGetDataServiceById(dataServiceId);
  return (
    <PageLayout>
      <PageHeader
        title="Data service info with id"
        badge={<Badge variant="info" size="lg">{formatUrn(dataService.id)}</Badge>}
      />
      <InfoGrid>
        <PageSection>
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
        </PageSection>
      </InfoGrid>
    </PageLayout>
  );
}

/**
 * Route for displaying data service details.
 */
export const Route = createFileRoute("/catalog/$catalogId/data-service/$dataServiceId")({
  component: RouteComponent,
  pendingComponent: () => <div>Loading...</div>,
  loader: ({ context: { queryClient, api_gateway }, params: { dataServiceId } }) => {
    if (!api_gateway) return;
    return queryClient.ensureQueryData(getDataServiceByIdOptions(api_gateway, dataServiceId));
  },
});
