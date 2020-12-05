import {
    BrowserRouter as Router,
    Switch,
    Route,
    Link,
    useParams,
} from 'react-router-dom';

import Home from './Home.js';
import { AccountPage } from './Account.js';

function App() {
    return (
        <Router>
            <div>
                <nav>
                    <ul>
                        <li>
                            <Link to="/">Home</Link>
                        </li>
                        <li>
                            <Link to="/account/1">First Account</Link>
                        </li>
                    </ul>
                </nav>
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
