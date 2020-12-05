import {
    BrowserRouter as Router,
    Switch,
    Route,
    Link,
    useParams,
} from 'react-router-dom';

import logo from './logo.svg';
import './App.css';
import Home from './Home.js';
import { AccountPage } from './Account.js';

function App() {
    return (
        <Router>
            <div className="App">
                <header className="App-header">
                    <img src={logo} className="App-logo" alt="logo" />
                    <p>
                        Edit <code>src/App.js</code> and save to reload.
                    </p>
                    <a
                    className="App-link"
                    href="https://reactjs.org"
                    target="_blank"
                    rel="noopener noreferrer"
                    >
                    React
                    </a>
                </header>
                <Switch>
                    <Route exact path="/">
                        <Home />
                    </Route>
                    <Route path="/account/:accountId">
                        <AccountPage />
                    </Route>
                </Switch>
            </div>
        </Router>
    );
}

export default App;
