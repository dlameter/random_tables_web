import { Button, TextField } from "@material-ui/core"
import { useState } from "react"
import { useAuth } from "./auth"

export default function ChangePasswordForm(props) {
    let [pass, changePass] = useState('')
    let [confirmPass, changeConfirmPass] = useState('')
    let [error, changeError] = useState()
    let { changePassword } = useAuth()

    let onChange = (e) => {
        changePass(e.target.value)
    }

    let onConfirmChange = (e) => {
        changeConfirmPass(e.target.value)
    }

    let checkPasswordMatching = (pass, confirm) => {
        let result = pass === confirm

        if (!result) {
            changeError('Passwords do not match')
        }

        return true
    }

    let onSubmit = (e) => {
        changeError('')
        if (checkPasswordMatching(pass, confirmPass)) {
            changePassword(pass)
        }
    }

    return (
        <>
            {error && <p>{error}</p>}
            <form>
                <TextField variant="outlined" label="Password" value={pass} onChange={onChange} />
                <TextField variant="outlined" label="Confirm password" value={confirmPass} onChange={onConfirmChange} />
                <Button variant="contained" onClick={onSubmit}>Change password</Button>
            </form>
        </>
    )
}