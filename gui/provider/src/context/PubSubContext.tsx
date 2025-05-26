import {createContext, ReactNode, useEffect, useState} from "react";
import {GATEWAY_API, GATEWAY_CALLBACK_ADDRESS} from "@/data";
import {useGetSubscriptionByCallbackAddress} from "@/data/pubsub-queries.ts";
import {queryClient} from "@/main.tsx";

interface PubSubContextType {
    websocket: WebSocket | null,
    connected: boolean,
    connectionError: boolean,
    subscriptionId: UUID | null
    lastHighLightedNotification: UUID | null
}

export const PubSubContext = createContext<PubSubContextType | null>(null)
export const PubSubContextProvider = ({children}: { children: ReactNode }) => {
    const [websocket, setWebsocket] = useState<WebSocket | null>(null)
    const [wsConnected, setWsConnected] = useState(false)
    const [wsConnectionError, setWsConnectionError] = useState(false)
    const [timer, setTimer] = useState<NodeJS.Timeout | null>(null)
    const [lastHighLightedNotification, setLastHighLightedNotification] = useState<UUID | null>(null)
    const {
        data: dataSubscriptions,
        isError: isDataSubscriptionError
    } = useGetSubscriptionByCallbackAddress(GATEWAY_CALLBACK_ADDRESS) // get all subscriptions


    // Check if server is up when websocket is not connected
    const reconnectOnClose = () => {
        const _timer = setInterval(() => {
            if (!websocket || websocket.readyState === WebSocket.CLOSED) {
                console.log("Attempting to reconnect WebSocket")
                webSocketConfig()
            }
        }, 1000)
        setTimer(_timer)
    }

    // connect to the WebSocket server
    const webSocketConfig = () => {
        const ws = new WebSocket(GATEWAY_API + "/ws")
        setWebsocket(ws)
        ws.onopen = () => {
            console.log("WebSocket connected")
            setWsConnected(true)
        }
        ws.onclose = () => {
            console.log("WebSocket disconnected")
            setWsConnected(false)
        }
        ws.onerror = (error) => {
            console.error("WebSocket error", error)
            setWsConnectionError(true)
        }
        ws.onmessage = (event) => {
            const notification: NotificationSub = JSON.parse(event.data)
            const category = notification.category
            const subcategory = notification.subcategory
            const notificationOperation = notification.messageOperation

            switch (category) {
                case "ContractNegotiation":
                    switch (subcategory) {
                        case "Participant":
                            switch (notificationOperation) {
                                case "Creation":
                                    queryClient.setQueryData(["PARTICIPANTS"], (oldData: Participant[]) => {
                                        const data = [...oldData]
                                        data.push(notification.messageContent as Participant)
                                        return data
                                    })
                                    setLastHighLightedNotification(notification.messageContent.participant_id)
                                    break;
                                default:
                                    console.warn("Unknown ContractNegotiation subcategory:", subcategory);
                            }
                            console.log("Participant Notification:", notification);
                            break;
                        case "ContractRequestMessage":
                            console.log("ContractRequestMessage Notification:", notification);
                            break;
                        case "ContractOfferMessage":
                            console.log("ContractOfferMessage Notification:", notification);
                            break;
                        case "ContractNegotiationEventMessage:accepted":
                            console.log("ContractNegotiationEventMessage Notification:", notification);
                            break;
                        case "ContractAgreementMessage":
                            console.log("ContractAgreementMessage Notification:", notification);
                            break;
                        case "ContractAgreementVerificationMessage":
                            console.log("ContractAgreementVerificationMessage Notification:", notification);
                            break;
                        case "ContractNegotiationEventMessage:finalized":
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
                            console.log("TransferRequestMessage Notification:", notification);
                            break;
                        case "TransferStartMessage":
                            console.log("TransferStartMessage Notification:", notification);
                            break;
                        case "TransferSuspensionMessage":
                            console.log("TransferSuspensionMessage Notification:", notification);
                            break;
                        case "TransferCompletionMessage":
                            console.log("TransferCompletionMessage Notification:", notification);
                            break;
                        case "TransferTerminationMessage":
                            console.log("TransferTerminationMessage Notification:", notification);
                            break;
                        default:
                            console.warn("Unknown TransferProcess subcategory:", subcategory);
                    }
                    break;
                default:
                    console.warn("Unknown notification category:", category);
            }
        }
        return () => {
            ws.close()
        }
    }

    useEffect(() => {
        webSocketConfig()
    }, []);

    useEffect(() => {
        if (websocket) {
            if (websocket.readyState === WebSocket.CLOSED) {
                reconnectOnClose()
            } else {
                if (timer) {
                    clearInterval(timer)
                    setTimer(null);
                }
            }
            return () => {
                if (timer) {
                    clearInterval(timer)
                    setTimer(null);
                }
            }
        }
    }, [wsConnected]);

    const value = {
        websocket,
        connected: wsConnected,
        connectionError: wsConnectionError,
        subscriptionId: isDataSubscriptionError ? null : dataSubscriptions.subscriptionId,
        lastHighLightedNotification
    }

    return <PubSubContext.Provider value={value}>
        {children}
    </PubSubContext.Provider>
}