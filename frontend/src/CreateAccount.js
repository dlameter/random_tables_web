import React from 'react';
import TextField from '@material-ui/core/TextField';
import Button from '@material-ui/core/Button';
import { createStyles, withStyles } from '@material-ui/core/styles';

const styles = (theme) => createStyles({
    root: {'& .MuiTextField-root': {
            margin: theme.spacing(1),
            width: '25ch',
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
            password: ""
        };
    }
    
    onSubmit() {
    }

    render() {
        const handleNameChange = (event) => {
            this.setState({ name: event.target.value });
        }
        const handlePasswordChange = (event) => {
            this.setState({ password: event.target.value });
        }

        return (
            <form id="create-account" class={this.classes.root}>
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
                        type="submit" 
                        form="create-account"
                        onClick={this.onSubmit()}
                    >
                        Submit
                    </Button>
                </div>
            </form>
        );
    }
}

export default withStyles(styles, { withTheme: true })(CreateAccount)