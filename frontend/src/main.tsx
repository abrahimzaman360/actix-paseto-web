import {StrictMode} from 'react'
import ReactDOM from 'react-dom/client'
import {RouterProvider, createRouter} from '@tanstack/react-router'

// Import the generated route tree
import {routeTree} from './routeTree.gen'

import './styles.css'
import reportWebVitals from './reportWebVitals.ts'
import {Loader2} from "lucide-react";
import {useAuth} from "@/hooks/useAuth.ts";

// Create a new router instance
const router = createRouter({
    routeTree,
    context: {
        isAuthenticated: false,
        isLoading: false
    },
    defaultPreload: 'intent',
    scrollRestoration: true,
    defaultStructuralSharing: true,
    defaultPreloadStaleTime: 0,
})

// Register the router instance for type safety
declare module '@tanstack/react-router' {
    interface Register {
        router: typeof router
    }
}

// Create an App component to manage the router's context dynamically
function App() {
    const {isAuthenticated, isLoading} = useAuth();

    if (isLoading) {
        return (
            <div className={'flex flex-col items-center justify-center h-screen'}>
                <Loader2 className={'animate-spin size-10'}/>
            </div>
        );
    }

    return <RouterProvider router={router} context={{isAuthenticated, isLoading}}/>;
}

// Render the app
const rootElement = document.getElementById('app')
if (rootElement && !rootElement.innerHTML) {
    const root = ReactDOM.createRoot(rootElement)
    root.render(
        <StrictMode>
            <App/>
        </StrictMode>,
    )
}

// Web Vitals
reportWebVitals()
