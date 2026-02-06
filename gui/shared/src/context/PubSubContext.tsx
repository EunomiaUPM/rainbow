/**
 * PubSubContext.tsx
 *
 * This module provides a React Context for managing WebSocket-based PubSub
 * (Publish/Subscribe) notifications. It handles real-time updates for:
 * - Contract Negotiations (requests, offers, agreements, verifications, terminations)
 * - Transfer Processes (requests, starts, suspensions, completions, terminations)
 * - Catalog events (datasets, data services, distributions)
 *
 * Architecture:
 * - WebSocket connection management with automatic reconnection
 * - Notification routing via category/subcategory dispatching
 * - React Query cache invalidation for real-time UI updates
 */

import {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useState,
  useCallback,
  useRef,
} from "react";
import { useGetSubscriptionByCallbackAddress } from "../data/pubsub-queries";
import { QueryClient, useQueryClient } from "@tanstack/react-query";
import { GlobalInfoContext, GlobalInfoContextType } from "./../context/GlobalInfoContext";

// =============================================================================
// TYPES & INTERFACES
// =============================================================================

interface PubSubContextType {
  websocket: WebSocket | null;
  connected: boolean;
  connectionError: boolean;
  subscriptionId: UUID | null;
  lastHighLightedNotification: UUID | null;
}

/**
 * Notification categories supported by the PubSub system.
 */
type NotificationCategory = "ContractNegotiation" | "Catalog" | "TransferProcess";

/**
 * Query keys used for React Query cache management.
 */
const QUERY_KEYS = {
  // Contract Negotiation
  PARTICIPANTS: "PARTICIPANTS",
  CN_PROCESSES: "CONTRACT_NEGOTIATION_PROCESSES",
  CN_PROCESS_BY_ID: "CONTRACT_NEGOTIATION_PROCESSES_BY_ID",
  CN_MESSAGES_BY_ID: "CONTRACT_NEGOTIATION_MESSAGES_BY_CNID",
  // Transfer Process
  TRANSFER_PROCESSES: "TRANSFER_PROCESSES",
  TRANSFER_PROCESS_BY_ID: "TRANSFER_PROCESS_BY_ID",
  TRANSFER_MESSAGES_BY_ID: "TRANSFER_MESSAGES_BY_PROVIDER_ID",
} as const;

// =============================================================================
// WEBSOCKET CONFIGURATION
// =============================================================================

const WS_RECONNECT_INTERVAL_MS = 1000;

// =============================================================================
// NOTIFICATION HANDLERS
// =============================================================================

/**
 * Updates the list cache by finding and replacing an existing item,
 * or adding a new one if not found.
 *
 * @param oldData - Current cached data array
 * @param newItem - New item to upsert
 * @param matchFn - Function to find the matching item
 * @returns Updated data array
 */
function upsertInList<T>(
  oldData: T[] | undefined,
  newItem: T,
  matchFn: (item: T) => boolean,
): T[] | undefined {
  if (!oldData) return undefined;

  const data = [...oldData];
  const index = data.findIndex(matchFn);

  if (index !== -1) {
    data[index] = newItem;
  } else {
    data.push(newItem);
  }

  return data;
}

/**
 * Updates the list cache by finding and replacing an existing item only.
 * Does not add new items if not found.
 *
 * @param oldData - Current cached data array
 * @param newItem - New item to update
 * @param matchFn - Function to find the matching item
 * @returns Updated data array
 */
function updateInList<T>(
  oldData: T[] | undefined,
  newItem: T,
  matchFn: (item: T) => boolean,
): T[] | undefined {
  if (!oldData) return undefined;

  const data = [...oldData];
  const index = data.findIndex(matchFn);

  if (index !== -1) {
    data[index] = newItem;
  }

  return data;
}

/**
 * Handles Contract Negotiation process updates.
 * Updates both list and single-item caches, and refetches related messages.
 */
function handleContractNegotiationProcess(
  queryClient: QueryClient,
  process: CNProcess,
  setHighlight: (id: UUID) => void,
): void {
  // Update the list view cache
  queryClient.setQueryData([QUERY_KEYS.CN_PROCESSES], (oldData: CNProcess[] | undefined) =>
    updateInList(oldData, process, (d) => d.provider_id === process.provider_id),
  );

  // Update the single item cache
  queryClient.setQueryData([QUERY_KEYS.CN_PROCESS_BY_ID, process.cn_process_id], process);

  // Refetch related messages to show new activity
  queryClient.refetchQueries({
    queryKey: [QUERY_KEYS.CN_MESSAGES_BY_ID, process.cn_process_id],
  });

  // Highlight the updated item in the UI
  setHighlight(process.cn_process_id);
}

/**
 * Handles Contract Request Message specifically, which may create new processes.
 */
function handleContractRequestMessage(
  queryClient: QueryClient,
  process: CNProcess,
  setHighlight: (id: UUID) => void,
): void {
  // Contract requests may create new processes, so we use upsert
  queryClient.setQueryData([QUERY_KEYS.CN_PROCESSES], (oldData: CNProcess[] | undefined) =>
    upsertInList(oldData, process, (d) => d.provider_id === process.provider_id),
  );

  queryClient.setQueryData([QUERY_KEYS.CN_PROCESS_BY_ID, process.cn_process_id], process);

  queryClient.refetchQueries({
    queryKey: [QUERY_KEYS.CN_MESSAGES_BY_ID, process.cn_process_id],
  });

  setHighlight(process.cn_process_id);
}

/**
 * Handles Transfer Process updates.
 * Updates both list and single-item caches, and refetches related messages.
 */
function handleTransferProcess(
  queryClient: QueryClient,
  process: TransferProcess,
  setHighlight: (id: UUID) => void,
): void {
  // Update the list view cache
  queryClient.setQueryData(
    [QUERY_KEYS.TRANSFER_PROCESSES],
    (oldData: TransferProcess[] | undefined) =>
      updateInList(oldData, process, (d) => d.provider_pid === process.provider_pid),
  );

  // Update the single item cache
  queryClient.setQueryData([QUERY_KEYS.TRANSFER_PROCESS_BY_ID, process.provider_pid], process);

  // Refetch related messages
  queryClient.refetchQueries({
    queryKey: [QUERY_KEYS.TRANSFER_MESSAGES_BY_ID, process.provider_pid],
  });

  // Highlight the updated item
  setHighlight(process.provider_pid);
}

/**
 * Handles Participant creation notifications.
 */
function handleParticipantCreation(
  queryClient: QueryClient,
  participant: Participant,
  setHighlight: (id: UUID) => void,
): void {
  queryClient.setQueryData([QUERY_KEYS.PARTICIPANTS], (oldData: Participant[] | undefined) => {
    if (!oldData) return undefined;
    return [...oldData, participant];
  });

  setHighlight(participant.participant_id);
}

// =============================================================================
// NOTIFICATION DISPATCHER
// =============================================================================

/**
 * Subcategories that trigger Contract Negotiation process updates.
 * These all share the same handling logic for process state changes.
 */
const CN_PROCESS_SUBCATEGORIES = new Set([
  "ContractOfferMessage",
  "ContractNegotiationEventMessage:accepted",
  "ContractAgreementMessage",
  "ContractVerificationMessage",
  "ContractAgreementVerificationMessage",
  "ContractEventMessage:finalized",
  "ContractNegotiationEventMessage:finalized",
  "ContractAcceptanceMessage",
  "ContractNegotiationTerminationMessage",
  "ContractTerminationMessage",
]);

/**
 * Subcategories that trigger Transfer Process updates.
 */
const TRANSFER_PROCESS_SUBCATEGORIES = new Set([
  "TransferRequestMessage",
  "TransferStartMessage",
  "TransferSuspensionMessage",
  "TransferCompletionMessage",
  "TransferTerminationMessage",
]);

/**
 * Main notification dispatcher. Routes incoming WebSocket messages
 * to the appropriate handler based on category and subcategory.
 */
function dispatchNotification(
  notification: NotificationSub,
  queryClient: QueryClient,
  setHighlight: (id: UUID) => void,
): void {
  const { category, subcategory, messageOperation, messageContent } = notification;

  switch (category) {
    case "ContractNegotiation":
      handleContractNegotiationCategory(
        subcategory,
        messageOperation,
        messageContent,
        queryClient,
        setHighlight,
      );
      break;

    case "Catalog":
      handleCatalogCategory(subcategory, notification);
      break;

    case "TransferProcess":
      handleTransferProcessCategory(subcategory, messageContent, queryClient, setHighlight);
      break;

    default:
      console.warn("Unknown notification category:", category);
  }
}

/**
 * Handles all Contract Negotiation category notifications.
 */
function handleContractNegotiationCategory(
  subcategory: string,
  messageOperation: string,
  messageContent: any,
  queryClient: QueryClient,
  setHighlight: (id: UUID) => void,
): void {
  // Handle Participant subcategory separately
  if (subcategory === "Participant") {
    if (messageOperation === "Creation") {
      handleParticipantCreation(queryClient, messageContent as Participant, setHighlight);
      console.log("Participant Creation Notification:", messageContent);
    } else {
      console.warn("Unknown Participant operation:", messageOperation);
    }
    return;
  }

  // Handle Contract Request (may create new process)
  if (subcategory === "ContractRequestMessage") {
    handleContractRequestMessage(queryClient, messageContent.process as CNProcess, setHighlight);
    console.log("ContractRequestMessage Notification:", messageContent);
    return;
  }

  // Handle all other CN process subcategories with unified logic
  if (CN_PROCESS_SUBCATEGORIES.has(subcategory)) {
    handleContractNegotiationProcess(
      queryClient,
      messageContent.process as CNProcess,
      setHighlight,
    );
    console.log(`${subcategory} Notification:`, messageContent);
    return;
  }

  console.warn("Unknown ContractNegotiation subcategory:", subcategory);
}

/**
 * Handles all Catalog category notifications.
 * Currently just logs them - extend as needed.
 */
function handleCatalogCategory(subcategory: string, notification: NotificationSub): void {
  const catalogSubcategories = [
    "Catalog",
    "Dataset",
    "DataService",
    "Distribution",
    "DatasetPolicies",
  ];

  if (catalogSubcategories.includes(subcategory)) {
    console.log(`${subcategory} Notification:`, notification);
  } else {
    console.warn("Unknown Catalog subcategory:", subcategory);
  }
}

/**
 * Handles all Transfer Process category notifications.
 */
function handleTransferProcessCategory(
  subcategory: string,
  messageContent: any,
  queryClient: QueryClient,
  setHighlight: (id: UUID) => void,
): void {
  if (TRANSFER_PROCESS_SUBCATEGORIES.has(subcategory)) {
    handleTransferProcess(queryClient, messageContent.process as TransferProcess, setHighlight);
    console.log(`${subcategory} Notification:`, messageContent);
    return;
  }

  console.warn("Unknown TransferProcess subcategory:", subcategory);
}

// =============================================================================
// CONTEXT PROVIDER
// =============================================================================

export const PubSubContext = createContext<PubSubContextType | null>(null);

export const PubSubContextProvider = ({ children }: { children: ReactNode }) => {
  const queryClient = useQueryClient();

  // WebSocket state
  const [websocket, setWebsocket] = useState<WebSocket | null>(null);
  const [wsConnected, setWsConnected] = useState(false);
  const [wsConnectionError, setWsConnectionError] = useState(false);
  const [lastHighLightedNotification, setLastHighLightedNotification] = useState<UUID | null>(null);

  // Reconnection timer ref (using ref to avoid stale closure issues)
  const reconnectTimerRef = useRef<NodeJS.Timeout | null>(null);

  // Global configuration
  const globalInfo = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
  const api_gateway = globalInfo?.api_gateway;
  const api_gateway_callback_address = globalInfo?.api_gateway_callback_address;

  // Subscription data
  const { data: dataSubscriptions, isError: isDataSubscriptionError } =
    useGetSubscriptionByCallbackAddress(api_gateway_callback_address);

  /**
   * Clears the reconnection timer if active.
   */
  const clearReconnectTimer = useCallback(() => {
    if (reconnectTimerRef.current) {
      clearInterval(reconnectTimerRef.current);
      reconnectTimerRef.current = null;
    }
  }, []);

  /**
   * Creates and configures a new WebSocket connection.
   */
  const createWebSocket = useCallback((): WebSocket | null => {
    if (!api_gateway) return null;

    const wsUrl = `${api_gateway}/ws`;
    console.log("Connecting WebSocket to:", wsUrl);

    const ws = new WebSocket(wsUrl);

    ws.onopen = () => {
      console.log("WebSocket connected");
      setWsConnected(true);
      setWsConnectionError(false);
      clearReconnectTimer();
    };

    ws.onclose = () => {
      console.log("WebSocket disconnected");
      setWsConnected(false);
    };

    ws.onerror = (error) => {
      console.error("WebSocket error:", error);
      setWsConnectionError(true);
    };

    ws.onmessage = (event) => {
      try {
        const notification: NotificationSub = JSON.parse(event.data);
        console.log("Received notification:", notification);
        dispatchNotification(notification, queryClient, setLastHighLightedNotification);
      } catch (error) {
        console.error("Failed to parse WebSocket message:", error);
      }
    };

    return ws;
  }, [api_gateway, queryClient, clearReconnectTimer]);

  /**
   * Starts the reconnection loop when WebSocket is closed.
   */
  const startReconnectLoop = useCallback(() => {
    if (reconnectTimerRef.current) return; // Already reconnecting

    reconnectTimerRef.current = setInterval(() => {
      if (!websocket || websocket.readyState === WebSocket.CLOSED) {
        console.log("Attempting to reconnect WebSocket...");
        const newWs = createWebSocket();
        if (newWs) {
          setWebsocket(newWs);
        }
      }
    }, WS_RECONNECT_INTERVAL_MS);
  }, [websocket, createWebSocket]);

  // Initialize WebSocket on mount
  useEffect(() => {
    if (!api_gateway) return;

    const ws = createWebSocket();
    if (ws) {
      setWebsocket(ws);
    }

    return () => {
      clearReconnectTimer();
      ws?.close();
    };
  }, [api_gateway]); // eslint-disable-line react-hooks/exhaustive-deps

  // Handle reconnection logic when connection state changes
  useEffect(() => {
    if (!websocket) return;

    if (websocket.readyState === WebSocket.CLOSED) {
      startReconnectLoop();
    } else {
      clearReconnectTimer();
    }

    return clearReconnectTimer;
  }, [wsConnected, websocket, startReconnectLoop, clearReconnectTimer]);

  // Context value
  const value: PubSubContextType = {
    websocket,
    connected: wsConnected,
    connectionError: wsConnectionError,
    subscriptionId: isDataSubscriptionError ? null : (dataSubscriptions?.subscriptionId ?? null),
    lastHighLightedNotification,
  };

  return <PubSubContext.Provider value={value}>{children}</PubSubContext.Provider>;
};

/**
 * Custom hook to access the PubSub context.
 * Throws an error if used outside of PubSubContextProvider.
 */
export const usePubSub = (): PubSubContextType => {
  const context = useContext(PubSubContext);
  if (!context) {
    throw new Error("usePubSub must be used within a PubSubContextProvider");
  }
  return context;
};
