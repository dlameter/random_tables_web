import { Button } from "@material-ui/core";
import React from 'react';
import { Link } from "react-router-dom/cjs/react-router-dom.min";

export default function EditAccount(props) {
    return (
        <>
            <Button variant="contained" component={Link} to="/account/password">Change Password</Button>
            <Button variant="contained" component={Link} to="/account/name">Change Account Name</Button>
        </>
    )
}