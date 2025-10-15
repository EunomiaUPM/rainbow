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

ReactDOM.createRoot(document.getElementById("root")!).render(
  <GlobalInfoContextProvider
    api_gateway_base="http://127.0.0.1:1106"
    dsrole="consumer"
    catalog_type="rainbow"
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
