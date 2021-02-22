import { Button, TextField } from "@material-ui/core";
import { useAuth } from "./auth";

export default function EditAccount(props) {
    const [user] = useAuth(); //prefill with current username
    const password = ''; // leave empty
    const username = user.username;

    // onSubmit
    // Check if username changed, send to update if it did
    // Check if password is not empty, send to update if it is not.

    return (
        <>
            <form id="update-account">
                <TextField variant="outlined" label="Account Name" value={username} />
                <TextField variant="outlined" label="Password" />
                <Button variant="contained" form="update-account">Update</Button>
            </form>
        </>
    )
}