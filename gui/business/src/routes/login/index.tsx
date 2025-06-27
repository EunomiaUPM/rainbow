import {createFileRoute} from '@tanstack/react-router'
import Heading from "shared/src/components/ui/heading.tsx";
import {postLoginPoll, usePostLogin} from "shared/src/data/business-queries.ts";
import {useContext, useEffect, useState} from "react";
import {GlobalInfoContext, GlobalInfoContextType} from "shared/src/context/GlobalInfoContext.tsx";
import QRCode from "react-qr-code";
import {Button} from "shared/src/components/ui/button.tsx";
import {generateRandomString} from "shared/src/lib/utils.ts";
import {useQuery} from "@tanstack/react-query";
import {AuthContext, AuthContextType} from "shared/src/context/AuthContext.tsx";

export const Route = createFileRoute('/login/')({
    component: RouteComponent,
})


function RouteComponent() {
    const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
    const {setAuthentication} = useContext<AuthContextType | null>(AuthContext)!;
    const {mutateAsync: loginAsync} = usePostLogin();
    const [oidcString, setOidcString] = useState("");
    const [copyStatus, setCopyStatus] = useState('');
    const [authRequestId, setAuthRequestId] = useState("")


    // Step 1: Get the initial OIDC string and authRequestId
    useEffect(() => {
        const getOidcLogin = async () => {
            const requestId = generateRandomString(12);
            setAuthRequestId(requestId);
            const oidc = await loginAsync({
                api_gateway,
                content: {
                    authRequestId: requestId
                }
            });
            setOidcString(oidc);
        };
        getOidcLogin().then(() => {
        });
    }, [api_gateway, loginAsync]);

    // Step 2: Use useQuery for polling
    const {data: pollData, isSuccess: pollSuccess} = useQuery({
        queryKey: ['LOGIN_POLL', authRequestId],
        queryFn: async () => {
            return await postLoginPoll({
                api_gateway,
                content: {authRequestId}
            });
        },
        // Enable the query only when authRequestId is truly available and oidcString is set
        enabled: !!authRequestId && authRequestId !== "" && oidcString !== "",
        refetchInterval: (data) => {
            // Stop polling if the login is successful
            if (data && data.state.data != undefined) { // Adjust "LOGIN_SUCCESS" to your actual success condition
                const data_json = JSON.parse(data.state.data)
                console.log(data_json)
                if (data_json.error == undefined) {
                    setAuthentication(data_json.mate as Participant, data_json.token)
                    return false; // Return `false` to stop polling
                }
            }
            return 1000; // Poll every 3 seconds
        },
        refetchIntervalInBackground: true,
        staleTime: Infinity,
        retry: false
    });

    useEffect(() => {
        if (pollSuccess) { // Adjust "LOGIN_SUCCESS" to your actual condition
            console.log("Login successful! Redirecting...");
            // Handle successful login (e.g., redirect to dashboard)
            // For example: navigate('/dashboard');
        }
    }, [pollSuccess, pollData]); // Add dependencies to useEffect


    const handleCopy = async () => {
        try {
            await navigator.clipboard.writeText(oidcString);
            setCopyStatus('Copied!');
            setTimeout(() => setCopyStatus(''), 2000); // Clear message after 2 seconds
        } catch (err) {
            console.error('Failed to copy: ', err);
            setCopyStatus('Failed to copy.');
            setTimeout(() => setCopyStatus(''), 2000); // Clear message after 2 seconds
        }
    };


    return <div className="space-y-4">
        <div>
            <Heading level="h5">Login</Heading>
            <div className="w-full space-y-4">
                <div className="max-w-[400px]">
                    {oidcString != "" &&
                        <QRCode size={32} value={oidcString} style={{height: "auto", maxWidth: "100%", width: "100%"}}

                                fgColor="#fff" bgColor="#0a0a1b"/>}
                </div>
                <div>
                    <div className="break-words">{oidcString}</div>
                    <Button onClick={handleCopy}>{copyStatus || 'Copy Text'}</Button>
                </div>
            </div>

        </div>
    </div>
}
