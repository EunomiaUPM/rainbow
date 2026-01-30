import * as React from "react";
import { cn } from "shared/src/lib/utils";
import { Badge } from "../badge";
import { ListItem, ListItemKey } from "../list";

export const MessageMetaContainer = React.forwardRef<
  HTMLUListElement,
  React.HTMLAttributes<HTMLUListElement>
>(({ className, children, ...props }, ref) => (
  <ul ref={ref} className={cn("gap-0", className)} {...props}>
    {children}
  </ul>
));
MessageMetaContainer.displayName = "MessageMetaContainer";

export interface MessageMetaProps extends React.HTMLAttributes<HTMLDivElement> {
  label: string;
  value: string;
}

export const MessageMeta = React.forwardRef<HTMLDivElement, MessageMetaProps>(
  ({ label, value, className, ...props }, ref) => (
    <ListItem
      className={cn(
        "min-h-8 flex flex-row flex-wrap gap-1 mb-1 text-white/70 border-none p-0 m-0",
        className,
      )}
      {...props}
    >
      <ListItemKey className="font-bold max-w-40">{label}</ListItemKey>
      <Badge className="max-w-full overflow-hidden" variant="info">
        {value}
      </Badge>
    </ListItem>
  ),
);
MessageMeta.displayName = "MessageMeta";
