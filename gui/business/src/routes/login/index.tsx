import {createFileRoute} from '@tanstack/react-router'
import Heading from "shared/src/components/ui/heading.tsx";
import {usePostLogin} from "shared/src/data/business-queries.ts";
import {useContext, useEffect, useState} from "react";
import {GlobalInfoContext, GlobalInfoContextType} from "shared/src/context/GlobalInfoContext.tsx";
import QRCode from "react-qr-code";
import {Button} from "shared/src/components/ui/button.tsx";
import {generateRandomString} from "shared/src/lib/utils.ts";

export const Route = createFileRoute('/login/')({
    component: RouteComponent,
})

function RouteComponent() {
    const {api_gateway} = useContext<GlobalInfoContextType | null>(GlobalInfoContext)!;
    const {mutateAsync: loginAsync} = usePostLogin();
    const [oidcString, getOidcString] = useState("");
    const [copyStatus, setCopyStatus] = useState(''); // 'Copied!', 'Failed to copy.', ''


    useEffect(() => {
        getOidcLogin()
    }, []);

    const getOidcLogin = async () => {
        const authRequestId = generateRandomString(12)
        console.log(authRequestId)
        const oidc = await loginAsync({
            api_gateway,
            content: {
                authRequestId
            }
        });
        getOidcString(oidc);
    }
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
