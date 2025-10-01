import {createFileRoute, Link, useRouter} from '@tanstack/react-router'
import {useState} from "react";
import {Button} from "@/components/ui/button.tsx";

export const Route = createFileRoute('/dashboard/')({
    component: RouteComponent,
})

function RouteComponent() {
    const {isAuthenticated} = Route.useRouteContext();

    if (!isAuthenticated) {
        return (
            <div className={"flex flex-col items-center justify-center h-screen"}>
                <Button className={"text-2xl font-bold"} asChild>
                    <Link to={"/auth/sign-in"}>
                        Go to Authentication Page!
                    </Link>
                </Button>
            </div>
        )
    }

    // Main Stuff
    const [loading, setLoading] = useState<boolean>(false);
    const router = useRouter();

    const handleLogout = () => {
        setLoading(true);

        setTimeout(() => {
            try {
                // Remove cookies
                localStorage.removeItem("tea-token");

                // Navigate to the home page
                router.navigate({to: "/"});

                // Force a full page reload after a short delay
                setTimeout(() => {
                    window.location.reload();
                }, 100);
            } catch (e: any) {
                console.log("> Error while logging out!", e.message);
            } finally {
                setLoading(false)
            }
        }, 2000);
    }

    return <div className={"flex flex-col items-center justify-center h-screen"}>
        <div className={"flex flex-col items-center justify-center gap-y-8"}>
            <h1 className={"text-5xl font-bold"}>Welcome to Dashboard </h1>

            <div>
                <Button variant={'outline'} className={"bg-blue-950 hover:cursor-pointer px-4 py-2"}
                        onClick={handleLogout}>{loading ? "Logging out!" : "Logout"}</Button>
            </div>
        </div>
    </div>
}
