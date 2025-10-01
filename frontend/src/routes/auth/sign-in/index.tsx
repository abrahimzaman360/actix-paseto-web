import {createFileRoute, Link, useRouter} from '@tanstack/react-router'
import {useState} from "react";
import * as React from "react";
import {toast} from "sonner";
import {Card} from "@/components/ui/card.tsx";
import {Button} from "@/components/ui/button.tsx";

export const Route = createFileRoute('/auth/sign-in/')({
    head: () => ({
        title: 'Sign in - Actix Web',
        meta: [
            {name: 'description', content: 'Sign in page for Actix Web'},
        ],
    }),
    component: RouteComponent,
})


function RouteComponent() {
    const {isAuthenticated} = Route.useRouteContext();

    if (isAuthenticated) {
        return (
            <div className={"flex flex-col items-center justify-center h-screen mx-auto"}>
                <h1 className={"text-2xl font-bold"}>You are already authenticated!</h1>
                <div className={"mt-4"}>
                    <Button asChild>
                        <Link to={"/dashboard"}>Go to Dashboard</Link>
                    </Button>
                </div>
            </div>
        )
    }


    const [formData, setFormData] = useState({
        email_address: "",
        password: "",
    });

    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const router = useRouter();

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setFormData({...formData, [e.target.name]: e.target.value});
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setLoading(true);
        setError(null);

        try {
            const res = await fetch("http://localhost:8080/auth/sign-in", {
                method: "POST",
                headers: {"Content-Type": "application/json"},
                body: JSON.stringify(formData),
                credentials: "include"
            });

            if (!res.ok) {
                const data = await res.text();
                throw new Error(data.toString() || "Sign in failed");
            }

            const data = await res.json();

            // Save in Cookie Store and Local Storage
            localStorage.setItem("tea-token", data.token);

            // Toaster
            toast.success("Successfully Signed In!");

            // Navigate to the home page
            router.navigate({ to: "/dashboard" });

            // Force a full page reload after a short delay
            setTimeout(() => {
                window.location.reload();
            }, 100);
        } catch (err: any) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="flex flex-col items-center justify-center h-screen gap-8">
            <Card className="border p-4  max-w-md w-full">
                <h1 className="text-2xl font-bold mb-4 text-center">Sign In</h1>
                <form
                    className="flex flex-col items-start justify-around gap-4 w-full"
                    onSubmit={handleSubmit}
                >
                    <div className="flex flex-col w-full">
                        <label className="font-bold text-md">Email address:</label>
                        <input
                            className="border p-2"
                            type="text"
                            name="email_address"
                            value={formData.email_address}
                            onChange={handleChange}
                        />
                    </div>
                    <div className="flex flex-col w-full">
                        <label className="font-bold text-md">Password:</label>
                        <input
                            className="border p-2"
                            type="password"
                            name="password"
                            value={formData.password}
                            onChange={handleChange}
                        />
                    </div>

                    <Button variant={'default'}
                            className={`w-full py-2 ${
                                loading ? " cursor-not-allowed" : " hover:cursor-pointer"
                            }`}
                            type="submit"
                            disabled={loading}
                    >
                        {loading ? "Signing In..." : "Sign In"}
                    </Button>
                </form>
                <div className="flex flex-col items-center justify-center gap-y-4">
                    <h2>or</h2>
                    <Button variant={'outline'}
                            className={`w-full border-2 ${
                                loading ? " cursor-not-allowed" : " hover:cursor-pointer"
                            }`}
                            asChild
                            disabled={loading}
                    >
                        <Link to={"/auth/sign-up"}>Sign up</Link>
                    </Button>
                </div>
                {error && <p className="text-red-500 mt-2">{error}</p>}
            </Card>
        </div>
    );
}
