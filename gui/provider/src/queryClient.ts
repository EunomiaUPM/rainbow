// // src/queryClient.js
// import { QueryClient, QueryCache, MutationCache } from '@tanstack/react-query';
// import { history } from './history'; // O tu propia instancia de navegación si no usas React Router DOM directamente
//
// const queryClient = new QueryClient({
//     queryCache: new QueryCache({
//         onError: (error) => {
//             // Asumiendo que usas Axios y el error es un AxiosError
//             if (error.response && (error.response.status === 401 || error.response.status === 403)) {
//                 localStorage.removeItem('authToken');
//                 queryClient.clear();
//
//                 // Redirigir al login
//                 if (history) { // Si usas una instancia de historial como en React Router v5 o un helper
//                     history.push('/login');
//                 } else { // Para React Router v6+ en un contexto de componente
//                     window.location.href = '/login';
//                 }
//             }
//         },
//     }),
//     mutationCache: new MutationCache({
//         onError: (error) => {
//             // También maneja errores de autenticación para mutaciones
//             if (error.response && (error.response.status === 401 || error.response.status === 403)) {
//                 localStorage.removeItem('authToken');
//                 queryClient.clear();
//                 if (history) {
//                     history.push('/login');
//                 } else {
//                     window.location.href = '/login';
//                 }
//             }
//         },
//     }),
//     defaultOptions: {
//         queries: {
//             retry: false, // No reintentar en errores de autenticación
//         },
//         mutations: {
//             retry: false, // No reintentar en errores de autenticación
//         },
//     },
// });
//
// export default queryClient;