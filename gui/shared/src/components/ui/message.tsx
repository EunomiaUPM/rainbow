// components/message/message-container.tsx
import * as React from "react";
import dayjs from "dayjs";
// @ts-ignore
import { cn } from "@/lib/utils";
// @ts-ignore
import { cva, type VariantProps } from "class-variance-authority";

import { Badge } from "./badge";
import Heading from "./heading";
import { ListItem, ListItemKey } from "./list";

type MessageType = {
  from: "Provider" | "Consumer";
  _type: string;
  created_at: string;
  cn_message_id: string;
  cn_process_id: string;
  content: unknown;
};


// Color variants para mensajes
const roleVariants = cva("", {
  variants: {
    variant: {
      default: "bg-brand-snow/10 border border-brand-snow", // container
      Provider: "bg-roles-provider/10 border border-roles-provider", // container
      Consumer: "bg-roles-consumer/10 border border-roles-consumer", // container
    },

    defaultVariants: {
      variant: "default",
    },
  },
});

const layoutVariants = cva("my-4 text-sm overflow-hidden", {
  variants: {
    variant: {
      Provider: "pr-[7%]",
      Consumer: "pl-[7%]",
    },
  },
});

// === COMPONENTES ===

const MessageLog = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement> & VariantProps<typeof layoutVariants>
>(({ className, variant, children, ...props }, ref) => {
  const isProvider = variant === "Provider";
  const role = isProvider ? "Provider" : "Consumer";

  return (
    <div
      ref={ref}
      className={cn(
        layoutVariants({ variant }),
        "w-full min-h-fit flex flex-col",
        isProvider ? "justify-start items-start" : "justify-end items-end",
        className
      )}
      {...props}
    >
      {children}
    </div>
  );
});
MessageLog.displayName = "MessageLog";

const RoleHeader = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement> & { from: "Provider" | "Consumer" }
>(({ from, className, ...props }, ref) => {
  const isProvider = from === "Provider";
  const variant = isProvider ? "Provider" : "Consumer";

  return (
    <div
      ref={ref}
      className={cn(
        "flex w-full",
        isProvider ? "justify-start" : "justify-end",
        className
      )}
      {...props}
    >
      <div
        className={cn(
          roleVariants({ variant }),
          "w-fit max-w-[640px] uppercase text-18 px-4 py-1 font-medium rounded-t-sm border-none",
          isProvider ? "ml-2 text-roles-provider" : "mr-2 text-roles-consumer"
        )}
      >
        {from}
      </div>
    </div>
  );
});
RoleHeader.displayName = "RoleHeader";

const MessageBody = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement> & VariantProps<typeof roleVariants>
>(({ className, children, variant = "", ...props }, ref) => {
  return (
    <div
      ref={ref}
      className={cn(
        roleVariants({ variant }),
        "w-full max-w-[640px] px-4 py-3 rounded-md rounded-b-xl border flex flex-col gap-2",
        className
      )}
      {...props}
    >
      {children}
    </div>
  );
});
MessageBody.displayName = "MessageBody";

const MessageTitle = React.forwardRef<
  HTMLHeadingElement,
  React.HTMLAttributes<HTMLHeadingElement>
>(({ className, children, ...props }, ref) => (
  <Heading
    level="h5"
    ref={ref}
    className={cn("mb-0 text-20", className)}
    {...props}
  >
    {children}
  </Heading>
));
MessageTitle.displayName = "MessageTitle";

const MessageTimestamp = React.forwardRef<
  HTMLParagraphElement,
  { created_at: string } & React.HTMLAttributes<HTMLParagraphElement>
>(({ created_at, className, ...props }, ref) => (
  <p ref={ref} className={cn("text-foreground mb-3", className)} {...props}>
    <i>{dayjs(created_at).format("DD/MM/YYYY - HH:mm")}</i>
  </p>
));
MessageTimestamp.displayName = "MessageTimestamp";

const MessageMeta = React.forwardRef<
  HTMLDivElement,
  { label: string; value: string } & React.HTMLAttributes<HTMLDivElement>
>(({ label, value, className, ...props }, ref) => (
  <ListItem
    ref={ref}
    className={cn(
      "flex flex-row flex-wrap gap-1 mb-1 text-white/70 border-none p-0 m-0",
      className
    )}
    {...props}
  >
    <ListItemKey className="font-bold max-w-40">{label}</ListItemKey>
    <Badge variant="info">{value}</Badge>
  </ListItem>
));
MessageMeta.displayName = "MessageMeta";

const MessageContent = React.forwardRef<
  HTMLDivElement,
  { content: unknown } & React.HTMLAttributes<HTMLDivElement>
>(({ content, className, ...props }, ref) => (
  <div ref={ref} className={cn("flex flex-col gap-3", className)} {...props}>
    <p className="font-bold min-w-40 text-white/60">Content:</p>
    <div className="w-full break-all">
      <pre className="p-4 rounded-lg break-all text-[11px] bg-black/70 text-secondary-300">
        <code className="whitespace-pre-wrap break-all">
          {JSON.stringify(content, null, 2)}
        </code>
      </pre>
    </div>
  </div>
));
MessageContent.displayName = "MessageContent";

// EXPORTS
export {
  MessageLog,
  RoleHeader,
  MessageBody,
  MessageTitle,
  MessageTimestamp,
  MessageMeta,
  MessageContent,
  type MessageType,
};
