import ReactDOM from "react-dom/client";
import "shared/index.css";
import {createRouter, RouterProvider} from "@tanstack/react-router";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {routeTree} from "./routeTree.gen";
import {PubSubContextProvider} from "@/context/PubSubContext.tsx";
import {AuthContextProvider} from "@/context/AuthContext.tsx";

export const queryClient = new QueryClient();

// Create a new router instance
const router = createRouter({routeTree, context: {queryClient}});

ReactDOM.createRoot(document.getElementById("root")!).render(
    <QueryClientProvider client={queryClient}>
        <AuthContextProvider>
            <PubSubContextProvider>
                <RouterProvider router={router}/>
            </PubSubContextProvider>
        </AuthContextProvider>
    </QueryClientProvider>
);
