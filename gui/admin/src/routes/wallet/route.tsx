import { createFileRoute, Outlet } from "@tanstack/react-router";
import { useContext } from "react";
import { SSIAuthContext } from "shared/src/context/SSIAuthContext";
import { RouteTabs } from "shared/src/components/layout/RouteTabs";
import { PageLayout } from "shared/src/components/layout/PageLayout";
import { PageHeader } from "shared/src/components/layout/PageHeader";
import { Button } from "shared/src/components/ui/button";
import { RefreshCw, Wallet } from "lucide-react";

/**
 * Wallet route layout.
 * Provides the main navigation tabs for the wallet sub-sections.
 */
const WalletLayout = () => {
  const { ownWalletOnboarded, onboardInWallet, isLoading } = useContext(SSIAuthContext);

  const tabs = [
    { label: "Info", to: "/wallet/info" },
    { label: "DID", to: "/wallet/did" },
    { label: "Keys", to: "/wallet/keys" },
    { label: "Credentials", to: "/wallet/credentials" },
    { label: "OID4VP", to: "/wallet/oidc4vp" },
    { label: "OID4VCI", to: "/wallet/oidc4vci" },
  ];

  return (
    <PageLayout>
      <PageHeader
        title="Wallet"
        description="Decentralized identity: DIDs, keys and verifiable credentials."
        actions={
          ownWalletOnboarded && (
            <Button
              onClick={onboardInWallet}
              isLoading={isLoading.onboard}
              variant="outline"
              size="sm"
            >
              {!isLoading.onboard && <RefreshCw />}
              {isLoading.onboard ? "Relinking…" : "Relink wallet"}
            </Button>
          )
        }
      />

      {!ownWalletOnboarded ? (
        <div className="flex flex-col items-center justify-center min-h-[400px] border border-dashed border-ink/10 rounded-xl bg-ink/5 p-12">
          <Wallet className="h-12 w-12 text-muted-foreground/40 mb-4" />
          <p className="text-muted-foreground font-medium text-lg">Wallet not linked</p>
          <p className="text-muted-foreground/60 text-sm mt-1 mb-6">
            Link your wallet to start managing your identity.
          </p>
          <Button onClick={onboardInWallet} isLoading={isLoading.onboard}>
            Link wallet
          </Button>
        </div>
      ) : (
        <>
          <RouteTabs tabs={tabs} />
          <Outlet />
        </>
      )}
    </PageLayout>
  );
};

export const Route = createFileRoute("/wallet")({
  component: WalletLayout,
});
