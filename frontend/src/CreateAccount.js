import React from 'react';
import axios from 'axios';
import {
    Button,
    Container,
    TextField
} from '@material-ui/core';
import { createStyles, withStyles } from '@material-ui/core/styles';
import BackendURLBuilder from './BackendURLBuilder';
import { Redirect } from 'react-router-dom';

const styles = (theme) => createStyles({
    root: {
        '& .MuiTextField-root': {
            margin: theme.spacing(1),
            width: '25ch',
        },
        '& .MuiButton-root': {
            margin: theme.spacing(1),
        },
    }
});

class CreateAccount extends React.Component {
    constructor(props) {
        super(props);
        const { classes } = props;
        this.classes = classes;
        this.state = {
            name: "",
            password: "",
            error: null,
            redirect: false,
        };

        this.onSubmit = this.onSubmit.bind(this);
    }

    onSubmit(event) {
        const url = BackendURLBuilder.createAccount();
        const data = { username: this.state.name, password: this.state.password };
        axios.post(url, data, { withCredentials: true }).then(
            (result) => {
                this.setState({ redirect: true });
            },
            (error) => {
                this.setState({ error: error });
            }
        );
        event.preventDefault();
    }

    render() {
        const handleNameChange = (event) => {
            this.setState({ name: event.target.value });
        }
        const handlePasswordChange = (event) => {
            this.setState({ password: event.target.value });
        }

        let error;
        if (this.state.error) {
            error = <h2>{this.state.error.message}</h2>
        }

        return (
            <Container maxWidth="sm">
                <form id="create-account" className={this.classes.root} noValidate autoComplete="off">
                    <div>
                        <TextField
                            required
                            label="Account Name"
                            value={this.state.name}
                            onChange={handleNameChange}
                            variant="outlined"
                        />
                        <TextField
                            required
                            label="Password"
                            value={this.state.password}
                            onChange={handlePasswordChange}
                            type="password"
                            variant="outlined"
                        />
                    </div>
                    <div>
                        <Button
                            variant="contained"
                            form="create-account"
                            onClick={this.onSubmit}
                        >
                            Create
                        </Button>
                    </div>
                </form>
                {error}
                { this.state.redirect && <Redirect to="/" />}
            </Container>
        );
    }
}

export default withStyles(styles, { withTheme: true })(CreateAccount)