import axios from 'axios';
import React, { createContext, useContext, useState } from 'react';
import BackendURLBuilder from './BackendURLBuilder';

const authContext = createContext();

export function ProvideAuth({ children }) {
    const auth = useProvideAuth();
    return <authContext.Provider value={auth}>{children}</authContext.Provider>
}

export const useAuth = () => {
    return useContext(authContext);
}

function useProvideAuth() {
    const [user, setUser] = useState(null);

    const login = (username, password) => {
        const url = BackendURLBuilder.login();
        const data = { username: username, password: password };
        return axios.post(url, data, { withCredentials: true })
            .then((response) => { setUser(response.data); return response; });
    }

    const logout = () => {
        const url = BackendURLBuilder.logout();
        return axios.post(url, null, { withCredentials: true })
            .then((response) => { setUser(null); return response; });
    }

    return { user, login, logout };
}