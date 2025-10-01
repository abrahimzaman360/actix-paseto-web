import {createFileRoute, Link, useRouter} from '@tanstack/react-router'
import {useState} from "react";
import * as React from "react";
import {Button} from "@/components/ui/button.tsx";

export const Route = createFileRoute('/auth/sign-up/')({
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
        full_name: "",
        username: "",
        email_address: "",
        password: "",
    });

    const [loading, setLoading] = useState(false);
    const [success, setSuccess] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const router = useRouter();

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setFormData({...formData, [e.target.name]: e.target.value});
    };

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setLoading(true);
        setError(null);
        setSuccess(false);

        try {
            const res = await fetch("http://127.0.0.1:8080/auth/sign-up", {
                method: "POST",
                headers: {"Content-Type": "application/json"},
                body: JSON.stringify(formData),
            });

            if (!res.ok) {
                const data = await res.json();
                throw new Error(data.message || "Something went wrong");
            }

            setSuccess(true);

            // Navigate to the home page
            router.navigate({to: "/auth/sign-in"});

            // Force a full page reload after a short delay
            setTimeout(() => {
                window.location.reload();
            }, 300);
        } catch (err: any) {
            setError(err.message);
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="flex flex-col items-center justify-center h-screen gap-8">
            <div className="border p-4  max-w-md w-full">
                <h1 className="text-2xl font-bold mb-4 text-center">Sign Up</h1>
                <form
                    className="flex flex-col items-start justify-around gap-4 w-full"
                    onSubmit={handleSubmit}
                >
                    <div className="flex flex-col w-full">
                        <label className="font-bold text-md">Full Name:</label>
                        <input
                            className="border p-2"
                            type="text"
                            name="full_name"
                            autoComplete="name"
                            value={formData.full_name}
                            onChange={handleChange}
                        />
                    </div>
                    <div className="flex flex-col w-full">
                        <label className="font-bold text-md">Username:</label>
                        <input
                            className="border p-2"
                            type="text"
                            name="username"
                            autoComplete="username"
                            value={formData.username}
                            onChange={handleChange}
                        />
                    </div>
                    <div className="flex flex-col w-full">
                        <label className="font-bold text-md">Email address:</label>
                        <input
                            className="border p-2"
                            type="email"
                            name="email_address"
                            autoComplete="email"
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
                            autoComplete="current-password"
                            value={formData.password}
                            onChange={handleChange}
                        />
                    </div>

                    <button
                        className={`w-full py-2 ${
                            loading ? "bg-gray-400 cursor-not-allowed" : "bg-amber-100 text-black hover:cursor-pointer"
                        }`}
                        type="submit"
                        disabled={loading}
                    >
                        {loading ? "Signing Up..." : "Sign Up"}
                    </button>
                </form>

                {success && <p className="text-green-500 mt-2">Sign Up Successful!</p>}
                {error && <p className="text-red-500 mt-2">{error}</p>}

                <Button className={"mt-3 text-black"} variant={'outline'} asChild>
                    <Link className={"w-full py-2"} to={"/auth/sign-in"}>
                        Sign In
                    </Link>
                </Button>
            </div>
        </div>
    );
}

