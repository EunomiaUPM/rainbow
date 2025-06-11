import * as React from "react";
import {
  useRouterState,
} from "@tanstack/react-router";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from "./ui/breadcrumb";
import NotificationsIcon from '@mui/icons-material/Notifications';
import PersonIcon from '@mui/icons-material/Person';

const Header = () => {
  const routerState = useRouterState();
  // console.log("Estado del router:", routerState);
  // console.log("Pathname actual:", routerState.location.pathname);
  // console.log("Ruta activa (última):", routerState.currentLocation.route.id);

  // sacar ruta activa. Separar los parametros por "/"
  // por cada parametro un breadcrumb
  let paths = routerState.location.pathname.split("/");
  let formatPath = (path) => {
    let formattedPath = path.split("-").join(" ");
    return console.log("Formatted path:", formattedPath), formattedPath;
  }
  paths.forEach((path, index) => {
    console.log(`Path ${index}:`, path);
  });
  paths.splice(0, 1); // Eliminar el primer elemento vacío
  console.log(formatPath(paths[0]), " formatted path");
  return (
    <div className="w-full border-b pb-3 border-black px-4 flex justify-between items-center">
      <Breadcrumb>
        <BreadcrumbList>
          {paths.map((path, index) => (
            <>
              <BreadcrumbItem key={index}>
                <BreadcrumbLink
                  href={`/${paths.slice(0, index + 1).join("/")}`}
                >
                 {formatPath(path)}
                </BreadcrumbLink>
              </BreadcrumbItem>
              {index < paths.length - 1 ? <BreadcrumbSeparator /> : ""}
            </>
          ))}
        </BreadcrumbList>
      </Breadcrumb>
      <div className="flex flex-row gap-4">
      <NotificationsIcon className="cursor-pointer" />
        <PersonIcon className="cursor-pointer" />
        {/* <Link to="/user">User</Link> */}
      
      </div>
    </div>
  );
};

export { Header };
