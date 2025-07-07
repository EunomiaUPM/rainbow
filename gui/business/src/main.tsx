import ReactDOM from "react-dom/client";
import "shared/index.css";
import { createRouter, RouterProvider } from "@tanstack/react-router";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { routeTree } from "./routeTree.gen";
import {
  AuthContext,
  AuthContextProvider,
  AuthContextType,
} from "shared/src/context/AuthContext.tsx";
import { GlobalInfoContextProvider } from "shared/src/context/GlobalInfoContext.tsx";
import { useContext } from "react";

export const queryClient = new QueryClient();

// Inner app for setting context in react router
const InnerApp = () => {
  const auth = useContext<AuthContextType | null>(AuthContext)!;
  const router = createRouter({
    routeTree,
    context: {
      queryClient,
      // @ts-ignore
      auth,
    },
    defaultPreload: "intent",
  });
  return <RouterProvider router={router} />;
};

ReactDOM.createRoot(document.getElementById("root")!).render(
  <GlobalInfoContextProvider
    api_gateway_base="http://127.0.0.1:1206"
    role="business"
    catalog_type="datahub"
  >
    <QueryClientProvider client={queryClient}>
      <AuthContextProvider>
        <InnerApp />
      </AuthContextProvider>
    </QueryClientProvider>
  </GlobalInfoContextProvider>,
);
