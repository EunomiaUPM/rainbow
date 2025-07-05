import React from "react";
// @ts-ignore
import { cn } from "@/lib/utils";
// @ts-ignore
import {
  MessageLog,
  RoleHeader,
  MessageBody,
  MessageTitle,
  MessageTimestamp,
  MessageMeta,
  MessageMetaContainer,
  MessageContent,
} from "./ui/message";

type TransferProcessMessageComponentProps = {
  message: TransferMessage;
};
let addSpacesFormat = (text: string) => {
  return text.replace(/(?!^)([A-Z])/g, " $1");
};

const TransferProcessMessageComponent: React.FC<TransferProcessMessageComponentProps> = ({ message }) => {
  return (
    <MessageLog key={message.id} variant={message.from}>
      <RoleHeader from={message.from} />
      <MessageBody variant={message.from}>
        <MessageTitle className="text-brand-sky mb-0 pb-0">
          {addSpacesFormat(message.message_type)}
        </MessageTitle>
        <MessageTimestamp created_at={message.created_at} />
          <MessageMetaContainer>
        <MessageMeta
          label=" Transfer Message Id"
          value={message.id.slice(9, 60)}
        />
        <MessageMeta
          label=" Transfer Process Id"
          value={message.transfer_process_id.slice(9, 60)}
        />
          </MessageMetaContainer>
        <MessageContent content={message.content} />
      
      </MessageBody>
    </MessageLog>
  );
};

export default TransferProcessMessageComponent;
