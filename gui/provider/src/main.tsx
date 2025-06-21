import ReactDOM from "react-dom/client";
import "shared/index.css";
import {createRouter, RouterProvider} from "@tanstack/react-router";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {routeTree} from "./routeTree.gen";
import {PubSubContextProvider} from "shared/src/context/PubSubContext.tsx";
import {AuthContextProvider} from "shared/src/context/AuthContext.tsx";
import {GlobalInfoContextProvider} from "shared/src/context/GlobalInfoContext.tsx";

export const queryClient = new QueryClient();

// Create a new router instance
const router = createRouter({routeTree, context: {queryClient}});

ReactDOM.createRoot(document.getElementById("root")!).render(
    <GlobalInfoContextProvider api_gateway_base="http://127.0.0.1:1205" role="provider" catalog_type="datahub">
        <QueryClientProvider client={queryClient}>
            <AuthContextProvider>
                <PubSubContextProvider>
                    <RouterProvider router={router}/>
                </PubSubContextProvider>
            </AuthContextProvider>
        </QueryClientProvider>
    </GlobalInfoContextProvider>
);
