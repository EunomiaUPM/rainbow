import {createFileRoute} from '@tanstack/react-router'
import dayjs from "dayjs";
import {useGetBypassDataServiceById} from "../../../../../../shared/src/data/catalog-bypass-queries.ts";
import Heading from 'shared/src/components/ui/heading';
import { Badge } from "shared/src/components/ui/badge";
import {
  List,
  ListItem,
  ListItemKey,
  ListItemDate,
} from "shared/src/components/ui/list";

function RouteComponent() {
    const {provider, dataServiceId} = Route.useParams()
    const {data: dataService} = useGetBypassDataServiceById(provider, dataServiceId)
    return <div className="space-y-4 pb-4">
        <Heading level="h3" className="flex gap-2 items-center">
        Data service info with id
        <Badge variant="info" size="lg">
          {" "}
          {dataService["@id"].slice(9, 29) + "[...]"}
        </Badge>{" "}
      </Heading>
      <div>
        <List className="text-sm">
          <ListItem>
            <ListItemKey>Data service title</ListItemKey>
            <p>{dataService.title}</p>
          </ListItem>
          <ListItem>
            <ListItemKey>Data service creation date</ListItemKey>
            <ListItemDate>
              {dayjs(dataService.issued).format("DD/MM/YYYY - HH:mm")}
            </ListItemDate>
          </ListItem>
          <ListItem>
            <ListItemKey>Data service endpoint URL</ListItemKey>
            <p>{dataService.endpointURL}</p>
          </ListItem>
          <ListItem>
            <ListItemKey>Data service description</ListItemKey>
            <p>{dataService.endpointDescription}</p>
          </ListItem>
        </List>
        </div>
    </div>
}

export const Route = createFileRoute('/provider-catalog/$provider/catalog/$catalogId/data-service/$dataServiceId')({
    component: RouteComponent,
    pendingComponent: () => <div>Loading...</div>,

})