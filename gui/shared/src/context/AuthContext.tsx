import {createContext, ReactNode} from "react";

interface AuthContextType {
    isAuthenticated: boolean
}

export const AuthContext = createContext<AuthContextType | null>(null);

export const AuthContextProvider = ({children}: { children: ReactNode }) => {
    const value = {
        isAuthenticated: false
    }
    // @ts-ignore
    return <AuthContext.Provider value={value}>
        {children}
    </AuthContext.Provider>
}