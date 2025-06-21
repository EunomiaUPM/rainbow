import {createContext, ReactNode} from "react";

export interface GlobalInfoContextType {
    catalog_type: "rainbow" | "datahub" | "both";
    role: string
    api_gateway_base: string,
    api_gateway: string,
    api_gateway_callback_address: string
}

export const GlobalInfoContext = createContext<GlobalInfoContextType | null>(null)
export const GlobalInfoContextProvider = ({children, api_gateway_base, role, catalog_type}: {
    children: ReactNode,
    api_gateway_base: string,
    role: string,
    catalog_type: "rainbow" | "datahub" | "both"
}) => {
    const value: GlobalInfoContextType = {
        catalog_type,
        role: role,
        api_gateway_base: api_gateway_base,
        api_gateway: api_gateway_base + "/gateway/api",
        api_gateway_callback_address: api_gateway_base + "/incoming-notification"
    }
    // @ts-ignore
    return <GlobalInfoContext.Provider value={value}>
        {children}
    </GlobalInfoContext.Provider>
}
