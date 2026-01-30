import ReactDOM from "react-dom/client";
import "shared/index.css";
import {createRouter, RouterProvider} from "@tanstack/react-router";
import {QueryClient, QueryClientProvider} from "@tanstack/react-query";
import {routeTree} from "./routeTree.gen";
import {AuthContextProvider} from "shared/src/context/AuthContext.tsx";
import {GlobalInfoContextProvider} from "shared/src/context/GlobalInfoContext.tsx";

export const queryClient = new QueryClient();

// Create a new router instance
const router = createRouter({routeTree, context: {queryClient}, basepath: "/admin"});

ReactDOM.createRoot(document.getElementById("root")!).render(
  <GlobalInfoContextProvider>
    <QueryClientProvider client={queryClient}>
      <AuthContextProvider>
          {/*<PubSubContextProvider>*/}
          {/*  <RouterProvider router={router}/>*/}
          {/*</PubSubContextProvider>*/}
          <RouterProvider router={router}/>
      </AuthContextProvider>
    </QueryClientProvider>
  </GlobalInfoContextProvider>,
);
