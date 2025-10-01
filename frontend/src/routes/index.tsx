import {createFileRoute, Link} from '@tanstack/react-router'
import {Card} from "@/components/ui/card.tsx";
import {Button} from "@/components/ui/button.tsx";

export const Route = createFileRoute('/')({
    component: App,
})

function App() {
    const {isAuthenticated, isLoading} = Route.useRouteContext();

    return (
        <div className={"flex flex-col items-center justify-center h-screen gap-8"}>
            <Card className={"border p-4 max-w-md w-full"}>
                <h1 className={"text-2xl font-bold mb-4 text-center"}>Welcome to PASETO Test</h1>

                {!isAuthenticated && !isLoading ? (
                    <div className={"flex flex-col items-center justify-center gap-2 "}>
                        <Link className={"border w-full py-2 text-center bg-amber-100 text-black"} to={"/auth/sign-in"}>Go
                            to Sign In</Link>
                        <Link className={"border w-full py-2 text-center bg-green-100 text-black"} to={"/auth/sign-up"}>Go
                            to Sign Up</Link>
                    </div>
                ) : (<div className={"flex flex-col items-center justify-center"}>
                    <Button className={"cursor-pointer"} asChild>
                        <Link to={"/dashboard"}>Go to Dashboard!</Link>
                    </Button>
                </div>)}
            </Card>
        </div>
    )
}
