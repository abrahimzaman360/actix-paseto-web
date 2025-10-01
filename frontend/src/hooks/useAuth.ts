import { useState, useEffect, useCallback } from 'react';

const getAuthStatus = () => {
    const token = localStorage.getItem('tea-token');
    return !!token;
};

export const useAuth = () => {
    const [isAuthenticated, setIsAuthenticated] = useState(getAuthStatus());
    const [isLoading, setIsLoading] = useState(true);

    const handleStorageChange = useCallback(() => {
        setIsAuthenticated(getAuthStatus());
    }, []);

    useEffect(() => {
        // Initial check
        setIsAuthenticated(getAuthStatus());
        setIsLoading(false);

        // Listen for changes in localStorage across other tabs
        window.addEventListener('storage', handleStorageChange);

        // Clean up the event listener
        return () => {
            window.removeEventListener('storage', handleStorageChange);
        };
    }, [handleStorageChange]);

    return { isAuthenticated, isLoading };
};