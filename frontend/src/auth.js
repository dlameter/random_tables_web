import axios from 'axios';
import React, { createContext, useContext, useEffect, useState } from 'react';
import BackendURLBuilder from './BackendURLBuilder';
import { useCookies } from 'react-cookie';

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
    const [cookies] = useCookies(["EXAUTH"]);

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

    const whois = () => {
        const url = BackendURLBuilder.whois();
        return axios.get(url, { withCredentials: true }).then((response) => { setUser(response.data); return response; })
    }

    // Might want to consider requiring re-entering the old password for extra protection
    const changePassword = (password) => {
        const url = BackendURLBuilder.changePassword()
        const data = { password: password }
        return axios.put(url, data, { withCredentials: true });
    }

    useEffect(() => {
        if (cookies.EXAUTH) {
            whois();
        }
    }, [cookies.EXAUTH]);

    return { user, login, logout, changePassword };
}