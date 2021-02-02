import axios from 'axios';
import React, { useEffect, useState } from 'react';
import { useCookies } from 'react-cookie';
import { Button, TextField } from '@material-ui/core';
import BackendURLBuilder from './BackendURLBuilder';

const cookieName = 'EXAUTH';

function CookieTest() {
    const [cookies, setCookie, removeCookie] = useCookies([cookieName]);

    useEffect(() => {
        axios.get(BackendURLBuilder.withPath("/cookie"), { withCredentials: true })
            .then();
        }
    )

    // function onSubmit(newMessage) {
    //     setCookie(cookieName, newMessage, { path: '/' });
    // }

    // function onRemove() {
    //     removeCookie(cookieName);
    // }

    // return (
    //     <div>
    //         <CookieForm message={cookies.EXAUTH} onChange={onSubmit} onRemove={onRemove}/>
    //         {cookies.message && <p>{cookies.EXAUTH}</p>}
    //     </div>
    // );

    return null;
}

function CookieForm(props) {
    const [message, setMessage] = useState(props.message);
    
    const onChange = (event) => {
        setMessage(event.target.value);
    };

    const callProp = () => {
        props.onChange(message);
    }

    const callRemove = () => {
        props.onRemove();
    }

    return (
        <div>
            <TextField variant="outlined" value={message} onChange={onChange}/>
            <Button variant="contained" onClick={callProp} >Change Cookie</Button>
            <Button variant="contained" onClick={callRemove} >Remove Cookie</Button>
        </div>
    );
}

export default CookieTest;