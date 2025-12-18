import {createContext, ReactNode, useContext, useEffect, useState} from "react";
import {useGetSubscriptionByCallbackAddress} from "../data/pubsub-queries";
import {useQueryClient} from "@tanstack/react-query";
import {GlobalInfoContext, GlobalInfoContextType} from "./../context/GlobalInfoContext";

interface PubSubContextType {
  websocket: WebSocket | null;
  connected: boolean;
  connectionError: boolean;
  subscriptionId: UUID | null;
  lastHighLightedNotification: UUID | null;
}

export const PubSubContext = createContext<PubSubContextType | null>(null);
export const PubSubContextProvider = ({children}: { children: ReactNode }) => {
  const queryClient = useQueryClient();
  const [websocket, setWebsocket] = useState<WebSocket | null>(null);
  const [wsConnected, setWsConnected] = useState(false);
  const [wsConnectionError, setWsConnectionError] = useState(false);
  const [timer, setTimer] = useState<NodeJS.Timeout | null>(null);
  const [lastHighLightedNotification, setLastHighLightedNotification] = useState<UUID | null>(null);
  const globalInfo =
    useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const api_gateway = globalInfo?.api_gateway;
  const api_gateway_callback_address = globalInfo?.api_gateway_callback_address;
  const {data: dataSubscriptions, isError: isDataSubscriptionError} =
    useGetSubscriptionByCallbackAddress(api_gateway_callback_address);


  useEffect(() => {
    if (!api_gateway) return;
    console.log("Init WebSocket in:", api_gateway + "/ws");
    const connectWs = () => {
      const ws = new WebSocket(api_gateway + "/ws");
      setWebsocket(ws);

      ws.onopen = () => {
        console.log("WebSocket connected");
        setWsConnected(true);
      };
    };

    connectWs();
    return () => {
      if(websocket) websocket.close();
    }

  }, [api_gateway]);

  const reconnectOnClose = () => {
    const _timer = setInterval(() => {
      if (!websocket || websocket.readyState === WebSocket.CLOSED) {
        console.log("Attempting to reconnect WebSocket");
        webSocketConfig();
      }
    }, 1000);
    setTimer(_timer);
  };

  // connect to the WebSocket server
  const webSocketConfig = () => {
    const ws = new WebSocket(api_gateway + "/ws");
    setWebsocket(ws);
    ws.onopen = () => {
      console.log("WebSocket connected");
      setWsConnected(true);
    };
    ws.onclose = () => {
      console.log("WebSocket disconnected");
      setWsConnected(false);
    };
    ws.onerror = (error) => {
      console.error("WebSocket error", error);
      setWsConnectionError(true);
    };
    ws.onmessage = (event) => {
      const notification: NotificationSub = JSON.parse(event.data);
      console.log(notification);
      const category = notification.category;
      const subcategory = notification.subcategory;
      const notificationOperation = notification.messageOperation;

      switch (category) {
        case "ContractNegotiation":
          switch (subcategory) {
            case "Participant":
              switch (notificationOperation) {
                case "Creation":
                  queryClient.setQueryData(["PARTICIPANTS"], (oldData: Participant[]) => {
                    if (!oldData) return;

                    const data = [...oldData];
                    data.push(notification.messageContent as Participant);
                    return data;
                  });
                  setLastHighLightedNotification(notification.messageContent.participant_id);
                  break;
                default:
                  console.warn("Unknown ContractNegotiation subcategory:", subcategory);
              }
              console.log("Participant Notification:", notification);
              break;
            case "ContractRequestMessage":
              // for list view
              queryClient.setQueryData(
                ["CONTRACT_NEGOTIATION_PROCESSES"],
                (oldData: CNProcess[]) => {
                  if (!oldData) return;

                  const data = [...oldData];
                  const isRequestAvailable = data.find(
                    (d) => d.provider_id === notification.messageContent.process.provider_id,
                  );
                  if (!isRequestAvailable) {
                    data.push(notification.messageContent.process as CNProcess);
                  } else {
                    const index = data.findIndex(
                      (d) => d.provider_id === notification.messageContent.process.provider_id,
                    );
                    if (index !== -1) {
                      data[index] = notification.messageContent.process as CNProcess;
                    }
                  }
                  return data;
                },
              );
              // for single view
              queryClient.setQueryData(
                [
                  "CONTRACT_NEGOTIATION_PROCESSES_BY_ID",
                  notification.messageContent.process.cn_process_id,
                ],
                notification.messageContent.process as CNProcess,
              );
              // for messages list
              // @ts-ignore
              queryClient.refetchQueries([
                "CONTRACT_NEGOTIATION_MESSAGES_BY_CNID",
                notification.messageContent.process.cn_process_id,
              ]);
              setLastHighLightedNotification(notification.messageContent.process.cn_process_id);
              console.log("ContractRequestMessage Notification:", notification);
              break;
            case "ContractOfferMessage":
              queryClient.setQueryData(
                ["CONTRACT_NEGOTIATION_PROCESSES"],
                (oldData: CNProcess[]) => {
                  if (!oldData) return;

                  const data = [...oldData];
                  const index = data.findIndex(
                    (d) => d.provider_id === notification.messageContent.process.provider_id,
                  );
                  if (index !== -1) {
                    data[index] = notification.messageContent.process as CNProcess;
                  }
                  return data;
                },
              );
              // for single view
              queryClient.setQueryData(
                [
                  "CONTRACT_NEGOTIATION_PROCESSES_BY_ID",
                  notification.messageContent.process.cn_process_id,
                ],
                notification.messageContent.process as CNProcess,
              );
              // for messages list
              // @ts-ignore
              queryClient.refetchQueries([
                "CONTRACT_NEGOTIATION_MESSAGES_BY_CNID",
                notification.messageContent.process.cn_process_id,
              ]);
              setLastHighLightedNotification(notification.messageContent.process.cn_process_id);
              console.log("ContractOfferMessage Notification:", notification);
              break;
            case "ContractNegotiationEventMessage:accepted":
              queryClient.setQueryData(
                ["CONTRACT_NEGOTIATION_PROCESSES"],
                (oldData: CNProcess[]) => {
                  if (!oldData) return;

                  const data = [...oldData];
                  const index = data.findIndex(
                    (d) => d.provider_id === notification.messageContent.process.provider_id,
                  );
                  if (index !== -1) {
                    data[index] = notification.messageContent.process as CNProcess;
                  }
                  return data;
                },
              );
              // for single view
              queryClient.setQueryData(
                [
                  "CONTRACT_NEGOTIATION_PROCESSES_BY_ID",
                  notification.messageContent.process.cn_process_id,
                ],
                notification.messageContent.process as CNProcess,
              );
              // for messages list
              // @ts-ignore
              queryClient.refetchQueries([
                "CONTRACT_NEGOTIATION_MESSAGES_BY_CNID",
                notification.messageContent.process.cn_process_id,
              ]);
              setLastHighLightedNotification(notification.messageContent.process.cn_process_id);
              console.log("ContractNegotiationEventMessage Notification:", notification);
              break;
            case "ContractAgreementMessage":
              queryClient.setQueryData(
                ["CONTRACT_NEGOTIATION_PROCESSES"],
                (oldData: CNProcess[]) => {
                  if (!oldData) return;

                  const data = [...oldData];
                  const index = data.findIndex(
                    (d) => d.provider_id === notification.messageContent.process.provider_id,
                  );
                  if (index !== -1) {
                    data[index] = notification.messageContent.process as CNProcess;
                  }
                  return data;
                },
              );
              // for single view
              queryClient.setQueryData(
                [
                  "CONTRACT_NEGOTIATION_PROCESSES_BY_ID",
                  notification.messageContent.process.cn_process_id,
                ],
                notification.messageContent.process as CNProcess,
              );
              // for messages list
              // @ts-ignore
              queryClient.refetchQueries([
                "CONTRACT_NEGOTIATION_MESSAGES_BY_CNID",
                notification.messageContent.process.cn_process_id,
              ]);
              setLastHighLightedNotification(notification.messageContent.process.cn_process_id);
              console.log("ContractAgreementMessage Notification:", notification);
              break;
            case "ContractVerificationMessage":
            case "ContractAgreementVerificationMessage":
              queryClient.setQueryData(
                ["CONTRACT_NEGOTIATION_PROCESSES"],
                (oldData: CNProcess[]) => {
                  if (!oldData) return;

                  const data = [...oldData];
                  const index = data.findIndex(
                    (d) => d.provider_id === notification.messageContent.process.provider_id,
                  );
                  if (index !== -1) {
                    data[index] = notification.messageContent.process as CNProcess;
                  }
                  return data;
                },
              );
              // for single view
              queryClient.setQueryData(
                [
                  "CONTRACT_NEGOTIATION_PROCESSES_BY_ID",
                  notification.messageContent.process.cn_process_id,
                ],
                notification.messageContent.process as CNProcess,
              );
              // for messages list
              // @ts-ignore
              queryClient.refetchQueries([
                "CONTRACT_NEGOTIATION_MESSAGES_BY_CNID",
                notification.messageContent.process.cn_process_id,
              ]);
              setLastHighLightedNotification(notification.messageContent.process.cn_process_id);
              console.log("ContractAgreementVerificationMessage Notification:", notification);
              break;
            case "ContractEventMessage:finalized":
            case "ContractNegotiationEventMessage:finalized":
              queryClient.setQueryData(
                ["CONTRACT_NEGOTIATION_PROCESSES"],
                (oldData: CNProcess[]) => {
                  if (!oldData) return;

                  const data = [...oldData];
                  const index = data.findIndex(
                    (d) => d.provider_id === notification.messageContent.process.provider_id,
                  );
                  if (index !== -1) {
                    data[index] = notification.messageContent.process as CNProcess;
                  }
                  return data;
                },
              );
              // for single view
              queryClient.setQueryData(
                [
                  "CONTRACT_NEGOTIATION_PROCESSES_BY_ID",
                  notification.messageContent.process.cn_process_id,
                ],
                notification.messageContent.process as CNProcess,
              );
              // for messages list
              // @ts-ignore
              queryClient.refetchQueries([
                "CONTRACT_NEGOTIATION_MESSAGES_BY_CNID",
                notification.messageContent.process.cn_process_id,
              ]);
              setLastHighLightedNotification(notification.messageContent.process.cn_process_id);
              console.log("ContractNegotiationEventMessage:finalized Notification:", notification);
              break;
            case "ContractAcceptanceMessage":
              queryClient.setQueryData(
                ["CONTRACT_NEGOTIATION_PROCESSES"],
                (oldData: CNProcess[]) => {
                  if (!oldData) return;

                  const data = [...oldData];
                  const index = data.findIndex(
                    (d) => d.provider_id === notification.messageContent.process.provider_id,
                  );
                  if (index !== -1) {
                    data[index] = notification.messageContent.process as CNProcess;
                  }
                  return data;
                },
              );
              // for single view
              queryClient.setQueryData(
                [
                  "CONTRACT_NEGOTIATION_PROCESSES_BY_ID",
                  notification.messageContent.process.cn_process_id,
                ],
                notification.messageContent.process as CNProcess,
              );
              // for messages list
              // @ts-ignore
              queryClient.refetchQueries([
                "CONTRACT_NEGOTIATION_MESSAGES_BY_CNID",
                notification.messageContent.process.cn_process_id,
              ]);
              setLastHighLightedNotification(notification.messageContent.process.cn_process_id);
              console.log("ContractAcceptanceMessage Notification:", notification);
              break;
            case "ContractNegotiationTerminationMessage":
            case "ContractTerminationMessage":
              queryClient.setQueryData(
                ["CONTRACT_NEGOTIATION_PROCESSES"],
                (oldData: CNProcess[]) => {
                  if (!oldData) return;

                  const data = [...oldData];
                  const index = data.findIndex(
                    (d) => d.provider_id === notification.messageContent.process.provider_id,
                  );
                  if (index !== -1) {
                    data[index] = notification.messageContent.process as CNProcess;
                  }
                  return data;
                },
              );
              // for single view
              queryClient.setQueryData(
                [
                  "CONTRACT_NEGOTIATION_PROCESSES_BY_ID",
                  notification.messageContent.process.cn_process_id,
                ],
                notification.messageContent.process as CNProcess,
              );
              // for messages list
              // @ts-ignore
              queryClient.refetchQueries([
                "CONTRACT_NEGOTIATION_MESSAGES_BY_CNID",
                notification.messageContent.process.cn_process_id,
              ]);
              setLastHighLightedNotification(notification.messageContent.process.cn_process_id);
              console.log("ContractNegotiationEventMessage:finalized Notification:", notification);
              break;
            default:
              console.warn("Unknown ContractNegotiation subcategory:", subcategory);
          }
          break;
        case "Catalog":
          switch (subcategory) {
            case "Catalog":
              console.log("Catalog Notification:", notification);
              break;
            case "Dataset":
              console.log("Dataset Notification:", notification);
              break;
            case "DataService":
              console.log("DataService Notification:", notification);
              break;
            case "Distribution":
              console.log("Distribution Notification:", notification);
              break;
            case "DatasetPolicies":
              console.log("DatasetPolicies Notification:", notification);
              break;
            default:
              console.warn("Unknown Catalog subcategory:", subcategory);
          }
          break;
        case "TransferProcess":
          switch (subcategory) {
            case "TransferRequestMessage":
              queryClient.setQueryData(["TRANSFER_PROCESSES"], (oldData: TransferProcess[]) => {
                if (!oldData) return;

                const data = [...oldData];
                const index = data.findIndex(
                  (d) => d.provider_pid === notification.messageContent.process.provider_pid,
                );
                if (index !== -1) {
                  data[index] = notification.messageContent.process as TransferProcess;
                }
                return data;
              });
              // for single view
              queryClient.setQueryData(
                ["TRANSFER_PROCESS_BY_ID", notification.messageContent.process.provider_pid],
                notification.messageContent.process as TransferProcess,
              );
              // for messages list
              // @ts-ignore
              queryClient.refetchQueries([
                "TRANSFER_MESSAGES_BY_PROVIDER_ID",
                notification.messageContent.process.provider_pid,
              ]);
              setLastHighLightedNotification(notification.messageContent.process.provider_pid);
              console.log("TransferRequestMessage Notification:", notification);
              break;
            case "TransferStartMessage":
              queryClient.setQueryData(["TRANSFER_PROCESSES"], (oldData: TransferProcess[]) => {
                if (!oldData) return;
                const data = [...oldData];
                const index = data.findIndex(
                  (d) => d.provider_pid === notification.messageContent.process.provider_pid,
                );
                if (index !== -1) {
                  data[index] = notification.messageContent.process as TransferProcess;
                }
                return data;
              });
              // for single view
              queryClient.setQueryData(
                ["TRANSFER_PROCESS_BY_ID", notification.messageContent.process.provider_pid],
                notification.messageContent.process as TransferProcess,
              );
              // for messages list
              // @ts-ignore
              queryClient.refetchQueries([
                "TRANSFER_MESSAGES_BY_PROVIDER_ID",
                notification.messageContent.process.provider_pid,
              ]);
              setLastHighLightedNotification(notification.messageContent.process.provider_pid);
              console.log("TransferStartMessage Notification:", notification);
              break;
            case "TransferSuspensionMessage":
              queryClient.setQueryData(["TRANSFER_PROCESSES"], (oldData: TransferProcess[]) => {
                if (!oldData) return;

                const data = [...oldData];
                const index = data.findIndex(
                  (d) => d.provider_pid === notification.messageContent.process.provider_pid,
                );
                if (index !== -1) {
                  data[index] = notification.messageContent.process as TransferProcess;
                }
                return data;
              });
              // for single view
              queryClient.setQueryData(
                ["TRANSFER_PROCESS_BY_ID", notification.messageContent.process.provider_pid],
                notification.messageContent.process as TransferProcess,
              );
              // for messages list
              // @ts-ignore
              queryClient.refetchQueries([
                "TRANSFER_MESSAGES_BY_PROVIDER_ID",
                notification.messageContent.process.provider_pid,
              ]);
              setLastHighLightedNotification(notification.messageContent.process.provider_pid);
              console.log("TransferSuspensionMessage Notification:", notification);
              break;
            case "TransferCompletionMessage":
              queryClient.setQueryData(["TRANSFER_PROCESSES"], (oldData: TransferProcess[]) => {
                if (!oldData) return;

                const data = [...oldData];
                const index = data.findIndex(
                  (d) => d.provider_pid === notification.messageContent.process.provider_pid,
                );
                if (index !== -1) {
                  data[index] = notification.messageContent.process as TransferProcess;
                }
                return data;
              });
              // for single view
              queryClient.setQueryData(
                ["TRANSFER_PROCESS_BY_ID", notification.messageContent.process.provider_pid],
                notification.messageContent.process as TransferProcess,
              );
              // for messages list
              // @ts-ignore
              queryClient.refetchQueries([
                "TRANSFER_MESSAGES_BY_PROVIDER_ID",
                notification.messageContent.process.provider_pid,
              ]);
              setLastHighLightedNotification(notification.messageContent.process.provider_pid);
              console.log("TransferCompletionMessage Notification:", notification);
              break;
            case "TransferTerminationMessage":
              queryClient.setQueryData(["TRANSFER_PROCESSES"], (oldData: TransferProcess[]) => {
                if (!oldData) return;
                const data = [...oldData];
                const index = data.findIndex(
                  (d) => d.provider_pid === notification.messageContent.process.provider_pid,
                );
                if (index !== -1) {
                  data[index] = notification.messageContent.process as TransferProcess;
                }
                return data;
              });
              // for single view
              queryClient.setQueryData(
                ["TRANSFER_PROCESS_BY_ID", notification.messageContent.process.provider_pid],
                notification.messageContent.process as TransferProcess,
              );
              // for messages list
              // @ts-ignore
              queryClient.refetchQueries([
                "TRANSFER_MESSAGES_BY_PROVIDER_ID",
                notification.messageContent.process.provider_pid,
              ]);
              setLastHighLightedNotification(notification.messageContent.process.provider_pid);
              console.log("TransferTerminationMessage Notification:", notification);
              break;
            default:
              console.warn("Unknown TransferProcess subcategory:", subcategory);
          }
          break;
        default:
          console.warn("Unknown notification category:", category);
      }
    };
    return () => {
      ws.close();
    };
  };

  useEffect(() => {
    webSocketConfig();
  }, []);

  useEffect(() => {
    if (websocket) {
      if (websocket.readyState === WebSocket.CLOSED) {
        reconnectOnClose();
      } else {
        if (timer) {
          clearInterval(timer);
          setTimer(null);
        }
      }
      return () => {
        if (timer) {
          clearInterval(timer);
          setTimer(null);
        }
      };
    }
  }, [wsConnected]);

  const value = {
    websocket,
    connected: wsConnected,
    connectionError: wsConnectionError,
    subscriptionId: isDataSubscriptionError ? null : dataSubscriptions.subscriptionId,
    lastHighLightedNotification,
  };

  // @ts-ignore
  return <PubSubContext.Provider value={value}>{children}</PubSubContext.Provider>;
};
