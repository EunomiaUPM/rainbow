import React, {
  createContext,
  ReactNode,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import { QueryClient, useQueryClient } from "@tanstack/react-query";
import { EventEnvelope } from "../data/orval/model";
import { getApiGatewayBase } from "../data/orval-mutator";

type Listener = (event: EventEnvelope) => void;

export interface LiveContextType {
  connected: boolean;
  /** Latest events, newest first. */
  events: EventEnvelope[];
  /** Calls `listener` for every event until the returned function runs. */
  subscribe: (listener: Listener) => () => void;
}

export const LiveContext = createContext<LiveContextType | null>(null);

/** Query key prefixes (orval keys are request paths) refreshed by each topic prefix. */
const INVALIDATIONS: [topic: string, keys: string[]][] = [
  ["transfers:", ["/transfers/"]],
  ["negotiations:", ["/negotiations/"]],
  [
    "catalog:",
    [
      "/catalogs",
      "/datasets",
      "/distributions",
      "/data-services",
      "/odrl-policies",
      "/policy-templates",
    ],
  ],
  ["connector:", ["/connector"]],
  ["keystore:", ["/keystore"]],
];
const BUFFER = 100;
const FLUSH_MS = 250;

/** One WebSocket to the gateway's `/live` for the whole app; events refresh the queries they touch. */
export const LiveContextProvider = ({ children }: { children: ReactNode }) => {
  const queryClient = useQueryClient();
  const [connected, setConnected] = useState(false);
  const [events, setEvents] = useState<EventEnvelope[]>([]);
  const listeners = useRef(new Set<Listener>());

  useEffect(() => {
    let socket: WebSocket | null = null;
    let retry: ReturnType<typeof setTimeout> | undefined;
    let attempts = 0;
    let stopped = false;
    const pending = new Set<string>();
    let flush: ReturnType<typeof setTimeout> | undefined;

    // Bursts of events (a whole DSP flow) collapse into one refresh per key prefix.
    const schedule = (topic: string) => {
      LiveContextProvider.prefixesFor(topic).forEach((p) => pending.add(p));
      flush ??= setTimeout(() => {
        flush = undefined;
        LiveContextProvider.invalidate(queryClient, [...pending]);
        pending.clear();
      }, FLUSH_MS);
    };

    const connect = () => {
      socket = new WebSocket(LiveContextProvider.url());
      socket.onopen = () => {
        attempts = 0;
        setConnected(true);
      };
      socket.onmessage = (msg) => {
        let envelope: EventEnvelope;
        try {
          envelope = JSON.parse(msg.data);
        } catch {
          return;
        }
        setEvents((prev) => [envelope, ...prev].slice(0, BUFFER));
        listeners.current.forEach((l) => l(envelope));
        schedule(String(envelope.topic));
      };
      socket.onclose = () => {
        setConnected(false);
        if (stopped) return;
        const delay = Math.min(30_000, 1_000 * 2 ** attempts++);
        retry = setTimeout(connect, delay);
      };
    };
    connect();

    return () => {
      stopped = true;
      clearTimeout(retry);
      clearTimeout(flush);
      socket?.close();
    };
  }, [queryClient]);

  const value = useMemo<LiveContextType>(
    () => ({
      connected,
      events,
      subscribe: (listener) => {
        listeners.current.add(listener);
        return () => listeners.current.delete(listener);
      },
    }),
    [connected, events],
  );

  return <LiveContext.Provider value={value}>{children}</LiveContext.Provider>;
};

/** `ws(s)://…/admin/api/live` from the configured gateway, or this page's origin. */
LiveContextProvider.url = (): string => {
  const base = (getApiGatewayBase() || `${window.location.origin}/admin/api`).replace(/\/$/, "");
  const api = base.endsWith("/admin/api") ? base : `${base}/admin/api`;
  return `${api.replace(/^http/, "ws")}/live`;
};

/** Every event refreshes the event feed and unread count, plus the keys of its own area. */
LiveContextProvider.prefixesFor = (topic: string): string[] => [
  "/events",
  ...INVALIDATIONS.filter(([t]) => topic.startsWith(t)).flatMap(([, keys]) => keys),
];

LiveContextProvider.invalidate = (queryClient: QueryClient, prefixes: string[]) => {
  if (prefixes.length === 0) return;
  queryClient.invalidateQueries({
    predicate: (q) => {
      const head = q.queryKey[0];
      return typeof head === "string" && prefixes.some((p) => head.startsWith(p));
    },
  });
};

/** Live connection state and events; `subscribe` for a callback per event. */
export const useLive = (): LiveContextType => {
  const ctx = useContext(LiveContext);
  if (!ctx) throw new Error("useLive must be used inside LiveContextProvider");
  return ctx;
};
