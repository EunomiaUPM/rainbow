import "shared/index.css";
import ReactDOM from "react-dom/client";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {createRouter, RouterProvider} from "@tanstack/react-router";
import {routeTree} from "./routeTree.gen.ts";
import {GlobalInfoContextProvider} from "shared/src/context/GlobalInfoContext.tsx";
import {AuthContextProvider} from "shared/src/context/AuthContext.tsx";
import {PubSubContextProvider} from "shared/src/context/PubSubContext.tsx";

const queryClient = new QueryClient();

const router = createRouter({routeTree, context: {queryClient}});
const apiGatewayBase = `${import.meta.env.GATEWAY_PROTOCOL}://${import.meta.env.GATEWAY_HOST}:${import.meta.env.GATEWAY_PORT}`;
const configRole = (import.meta.env.CONFIG_ROLE as string).toLowerCase();
const catalogType = import.meta.env.CATALOG_AS_DATAHUB === "true" ? "datahub" : "rainbow";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <GlobalInfoContextProvider
    api_gateway_base={apiGatewayBase}
    dsrole={configRole}
    catalog_type={catalogType}
  >
    <QueryClientProvider client={queryClient}>
      <AuthContextProvider>
        <PubSubContextProvider>
          <RouterProvider router={router}/>
        </PubSubContextProvider>
      </AuthContextProvider>
    </QueryClientProvider>
  </GlobalInfoContextProvider>,
);
