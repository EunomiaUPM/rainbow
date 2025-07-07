import { createFileRoute, redirect } from "@tanstack/react-router";

const Index = () => {
  return <div className="space-y-2 p-4"></div>;
};

export const Route = createFileRoute("/")({
  component: Index,
  beforeLoad: ({ context }) => {
    // @ts-ignore
    if (!context.auth.isAuthenticated) {
      throw redirect({
        to: "/login", // Redirige a la página de login si no está autenticado
        search: {
          redirect: location.pathname, // Opcional: para redirigir de vuelta después del login
        },
      });
    }
  },
});

export default Index;
