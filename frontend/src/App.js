import {
    BrowserRouter as Router,
    Switch,
    Route,
    Link,
} from 'react-router-dom';
import { makeStyles } from '@material-ui/core/styles';
import AppBar from '@material-ui/core/AppBar';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';
import Button from '@material-ui/core/Button';
import IconButton from '@material-ui/core/IconButton';
import MenuIcon from '@material-ui/icons/Menu';
import Container from '@material-ui/core/Container';

import Home from './Home.js';
import { AccountPage } from './Account.js';
import CreateAccount from './CreateAccount.js';

const useStyles = makeStyles((theme) => ({
  root: {
    flexGrow: 1,
  },
  menuButton: {
    marginRight: theme.spacing(2),
  },
  title: {
    flexGrow: 1,
  },
  spaced: {
    paddingTop: theme.spacing(4),
  },
}));

function App() {
    const classes = useStyles();
    
    return (
        <Router>
            <AppBar>
                <Toolbar>
                    <IconButton edge="start" className={classes.menuButton} color="inherit" aria-label="menu">
                        <MenuIcon />
                    </IconButton>
                    <Typography variant="h6" className={classes.title}>
                        Random Tables Web
                    </Typography>
                    <Button color="inherit" component={Link} to="/">Home</Button>
                    <Button color="inherit" component={Link} to="/createAccount">Sign up</Button>
                </Toolbar>
            </AppBar>
            <Toolbar></Toolbar>
            <Container maxWidth="md" className={classes.spaced}>
                <Switch>
                    <Route exact path="/">
                        <Home />
                    </Route>
                    <Route path="/account/:accountId">
                        <AccountPage />
                    </Route>
                    <Route path="/createAccount">
                        <CreateAccount />
                    </Route>
                </Switch>
            </Container>
        </Router>
    );
}

export default App;
