// components/message/message-container.tsx
import * as React from "react";
import dayjs from "dayjs";
import {cn} from "shared/src/lib/utils";

import {cva} from "class-variance-authority";

import {Badge} from "./badge";
import Heading from "./heading";
import {ListItem, ListItemKey} from "./list";
import SyntaxHighlighter from 'react-syntax-highlighter';
import {vs2015} from 'react-syntax-highlighter/dist/esm/styles/hljs';


export type RoleType = "Provider" | "Consumer" | "Business" | "Customer";

// Helper para saber qué roles usan el estilo "Provider" y cuáles "Consumer"
const mapRoleToVariant = (role: RoleType) => {
  if (role === "Provider" || role === "Business") return "Provider";
  if (role === "Consumer" || role === "Customer") return "Consumer";
  return "default";
};

// Color variants para mensajes
const roleVariants = cva("", {
  variants: {
    variant: {
      default: "bg-brand-snow/10 border border-brand-snow",
      Provider: "bg-roles-provider/10 border border-roles-provider",
      Consumer: "bg-roles-consumer/10 border border-roles-consumer",
    },
  },
  defaultVariants: {
    variant: "default",
  },
});

const layoutVariants = cva("my-4 text-sm overflow-hidden", {
  variants: {
    variant: {
      Provider: "pr-[7%]",
      Consumer: "pl-[7%]",
      default: "pr-[7%]",
    },
  },
});

// === COMPONENTES ===

const MessageLog = React.forwardRef<
  HTMLDivElement,
  React.HTMLAttributes<HTMLDivElement> & { variant: RoleType }
>(({className, variant, children, ...props}, ref) => {
  // Mapeamos a "Provider" o "Consumer"
  const styleVariant = mapRoleToVariant(variant);
  const isLeft = styleVariant === "Provider";

  return (
    <div
      ref={ref}
      className={cn(
        layoutVariants({variant: styleVariant}),
        "w-full min-h-fit flex flex-col",
        isLeft ? "justify-start items-start" : "justify-end items-end",
        className,
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
  React.HTMLAttributes<HTMLDivElement> & { from: RoleType }
>(({from, className, ...props}, ref) => {
  const styleVariant = mapRoleToVariant(from);
  const isLeft = styleVariant === "Provider";

  return (
    <div
      ref={ref}
      className={cn("flex w-full", isLeft ? "justify-start" : "justify-end", className)}
      {...props}
    >
      <div
        className={cn(
          roleVariants({variant: styleVariant}),
          "w-fit max-w-[640px] uppercase text-18 px-4 py-1 font-medium rounded-t-sm border-none",
          isLeft ? "ml-2 text-roles-provider" : "mr-2 text-roles-consumer",
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
  React.HTMLAttributes<HTMLDivElement> & { variant: RoleType }
>(({className, children, variant, ...props}, ref) => {
  const styleVariant = mapRoleToVariant(variant!);

  return (
    <div
      ref={ref}
      className={cn(
        roleVariants({variant: styleVariant}),
        "w-full max-w-[640px] px-4 py-3 rounded-md rounded-b-xl border flex flex-col gap-2",
        className,
      )}
      {...props}
    >
      {children}
    </div>
  );
});
MessageBody.displayName = "MessageBody";

const MessageTitle = React.forwardRef<HTMLHeadingElement, React.HTMLAttributes<HTMLHeadingElement>>(
  ({className, children, ...props}, ref) => (
    <Heading
      level="h4"
      ref={ref}
      className={cn("mb-0 !text-[20px] !text-foreground-400", className)}
      {...props}
    >
      {children}
    </Heading>
  ),
);
MessageTitle.displayName = "MessageTitle";

const MessageTimestamp = React.forwardRef<
  HTMLParagraphElement,
  { created_at: string } & React.HTMLAttributes<HTMLParagraphElement>
>(({created_at, className, ...props}, ref) => (
  <p ref={ref} className={cn("text-foreground mb-3", className)} {...props}>
    <i>{dayjs(created_at).format("DD/MM/YYYY - HH:mm")}</i>
  </p>
));
MessageTimestamp.displayName = "MessageTimestamp";

const MessageMetaContainer = React.forwardRef<
  HTMLUListElement,
  React.HTMLAttributes<HTMLUListElement>
>(({className, children, ...props}, ref) => (
  <ul ref={ref} className={cn("gap-0", className)} {...props}>
    {children}
  </ul>
));
MessageMetaContainer.displayName = "MessageMetaContainer";

interface MessageMetaProps extends React.HTMLAttributes<HTMLDivElement> {
  label: string;
  value: string;
}

const MessageMeta = React.forwardRef<HTMLDivElement, MessageMetaProps>(
  ({label, value, className, ...props}, ref) => (
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

const MessageContent = React.forwardRef<HTMLDivElement, React.HTMLAttributes<HTMLDivElement>>(
  ({className, ...props}, ref) => {
    const content = (props as any).content;
    return (
      <div ref={ref} className={cn("flex flex-col gap-3", className)} {...props}>
        <p className="font-bold min-w-40 text-white/60">Content:</p>
        <div className="w-full break-all">
          <pre
            className="p-4 rounded-lg break-all text-[13px] !font-mono overflow-hidden bg-black/70 text-secondary-300">
            <code className="whitespace-pre break-all">
               <SyntaxHighlighter style={vs2015} language="json" wrapLongLines={false}
                                  showLineNumbers={true}>{content}</SyntaxHighlighter>
            </code>
          </pre>
        </div>
      </div>
    );
  },
);
MessageContent.displayName = "MessageContent";

// EXPORTS
export {
  MessageLog,
  RoleHeader,
  MessageBody,
  MessageTitle,
  MessageMetaContainer,
  MessageTimestamp,
  MessageMeta,
  MessageContent,
};
