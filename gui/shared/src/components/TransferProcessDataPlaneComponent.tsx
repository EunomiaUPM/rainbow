import React from "react";
import dayjs from "dayjs";
import {List, ListItem, ListItemKey} from "./ui/list";
import {Badge, BadgeState} from "shared/src/components/ui/badge";

type TransferProcessDataPlaneComponentProps = {
  dataPlane: DataplaneSession;
};

const TransferProcessDataPlaneComponent: React.FC<TransferProcessDataPlaneComponentProps> = ({
                                                                                               dataPlane,
                                                                                             }) => {
  const scopedListItemKeyClasses = "basis-[33%]";

  return (
    <List className="h-fit">
      <ListItem>
        <ListItemKey className={scopedListItemKeyClasses}>Data plane Id</ListItemKey>
        <Badge variant={"info"}>{dataPlane.id.slice(9, -1)}</Badge>
      </ListItem>
      <ListItem>
        <ListItemKey className={scopedListItemKeyClasses}>Process Direction</ListItemKey>
        <Badge className="uppercase">{dataPlane.process_direction}</Badge>
      </ListItem>
      {dataPlane.upstream_hop.url && (
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>Upstream protocol</ListItemKey>
          <Badge className="uppercase">{dataPlane.upstream_hop.protocol}</Badge>
        </ListItem>
      )}
      {dataPlane.downstream_hop.url && (
        <ListItem>
          <ListItemKey className={scopedListItemKeyClasses}>Downstream protocol</ListItemKey>
          <Badge className="uppercase">{dataPlane.downstream_hop.protocol}</Badge>
        </ListItem>
      )}
      <ListItem>
        <ListItemKey className={scopedListItemKeyClasses}>Process address</ListItemKey>
        <Badge variant={"info"}>{dataPlane.process_address.url.slice(0, -45) + "[...]"}</Badge>
      </ListItem>
      <ListItem>
        <ListItemKey className={scopedListItemKeyClasses}>Created At</ListItemKey>
        {dayjs(dataPlane.created_at).format("DD/MM/YY HH:mm")}
      </ListItem>
      <ListItem>
        <ListItemKey className={scopedListItemKeyClasses}>Updated At</ListItemKey>
        {dayjs(dataPlane.updated_at).format("DD/MM/YY HH:mm")}
      </ListItem>
      <ListItem>
        <ListItemKey className={scopedListItemKeyClasses}>State</ListItemKey>
        <Badge variant={"status"} state={dataPlane.state as BadgeState}>
          {dataPlane.state}
        </Badge>
      </ListItem>
    </List>
  );
};

export default TransferProcessDataPlaneComponent;
