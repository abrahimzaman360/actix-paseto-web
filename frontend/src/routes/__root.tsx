import {Outlet, createRootRouteWithContext} from '@tanstack/react-router'
import {TanStackRouterDevtoolsPanel} from '@tanstack/react-router-devtools'
import {TanstackDevtools} from '@tanstack/react-devtools'
import {Toaster} from "sonner";


// Define the type for your authentication context
interface AuthContext {
    isAuthenticated: boolean;
    isLoading: boolean
}


export const Route = createRootRouteWithContext<AuthContext>()({
    component: () => (
        <>
            <Outlet/>
            <TanstackDevtools
                config={{
                    position: 'bottom-left',
                }}
                plugins={[
                    {
                        name: 'Tanstack Router',
                        render: <TanStackRouterDevtoolsPanel/>,
                    },
                ]}
            />
            <Toaster richColors closeButton/>
        </>
    ),
})
