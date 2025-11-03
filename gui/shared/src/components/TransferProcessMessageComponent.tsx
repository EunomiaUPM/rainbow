import React from "react";
// @ts-ignore
import {cn} from "@/lib/utils";
// @ts-ignore
import {
  MessageBody,
  MessageContent,
  MessageLog,
  MessageMeta,
  MessageMetaContainer,
  MessageTimestamp,
  MessageTitle,
  RoleHeader,
  RoleType,
} from "./ui/message";

type TransferProcessMessageComponentProps = {
  message: TransferMessage;
};
let addSpacesFormat = (text: string) => {
  return text.replace(/(?!^)([A-Z])/g, " $1");
};

const TransferProcessMessageComponent: React.FC<TransferProcessMessageComponentProps> = ({message,}) => {
  return (
    <MessageLog key={message.id} variant={message.from as RoleType}>
      <RoleHeader from={message.from as RoleType}/>
      <MessageBody variant={message.from as RoleType}>
        <MessageTitle className="text-brand-sky mb-0 pb-0">
          {addSpacesFormat(message.message_type)}
        </MessageTitle>
        <MessageTimestamp created_at={message.created_at.toString()}/>
        <MessageMetaContainer>
          <MessageMeta label=" Transfer Message Id" value={message.id.slice(9, 60)}/>
          <MessageMeta
            label=" Transfer Process Id"
            value={message.transfer_process_id.slice(9, 60)}
          />
        </MessageMetaContainer>
        <MessageContent content={JSON.stringify(message.content, null, 4)}/>
      </MessageBody>
    </MessageLog>
  );
};

export default TransferProcessMessageComponent;
