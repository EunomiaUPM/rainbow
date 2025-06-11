import "shared/index.css";
import ReactDOM from "react-dom/client";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {createRouter, RouterProvider} from "@tanstack/react-router";
import {routeTree} from "./routeTree.gen.ts";
import {GlobalInfoContextProvider} from "shared/src/context/GlobalInfoContext.tsx";

const queryClient = new QueryClient();

const router = createRouter({routeTree, context: {queryClient}});

ReactDOM.createRoot(document.getElementById("root")!).render(
    <GlobalInfoContextProvider api_gateway_base="http://127.0.0.1:1105">
        <QueryClientProvider client={queryClient}>
            <RouterProvider router={router}/>
        </QueryClientProvider>
    </GlobalInfoContextProvider>
);