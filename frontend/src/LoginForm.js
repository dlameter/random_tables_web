import { Button, TextField } from '@material-ui/core';
import React, { useState } from 'react';
import { Redirect } from 'react-router-dom';
import { useAuth } from './auth.js';

export default function LoginForm(props) {
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [error, setError] = useState(null);
    const [redirect, setRedirect] = useState(false);

    const auth = useAuth();
    if (auth.user) {
        return <Redirect to="/" />;
    }

    function onSubmit(e) {
        auth.login(username, password).then((result) => { setRedirect(true) }, (error) => { setError(error) });
        e.preventDefault();
    }

    function handleChange(func) {
        return (e) => { func(e.target.value) };
    }

    return (
        <>
            {error && <p>{error.message}</p>}
            <form onSubmit={onSubmit}>
                <TextField id="username" label="Username" variant="outlined" value={username} onChange={handleChange(setUsername)} />
                <TextField id="password" label="Password" variant="outlined" type="password" value={password} onChange={handleChange(setPassword)} />
                <Button variant="contained" type="submit">Login</Button>
            </form>
            {redirect && <Redirect to="/" />}
        </>
    );
}