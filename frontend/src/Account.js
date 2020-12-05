import React from "react";
import axios from "axios";
import { useParams} from 'react-router-dom';

import TableList from "./TableList";
import BackendURLBuilder from "./BackendURLBuilder";

export const Account = class Account extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
            error: null,
            accountId: props.accountId,
            account: null,
            accountLoaded: false,
            tables: [],
            tablesLoaded: false,
        };
    }
    
    componentDidMount() {
        const url = BackendURLBuilder.getAccountById(this.state.accountId);
        axios.get(url)
            .then(
                (res) => {
                    let account = res.data;
                    this.setState({ account, accountLoaded: true})
                }, 
                (error) => this.setState({error, accountLoaded: true})
            );
    }

    render() {
        if (this.state.error) {
            return (
                <div>
                    <h1>Error: {this.state.error.message}</h1>
                </div>
            );
        }
        else if (!this.state.accountLoaded) {
            return (
                <div>
                    <h1>Loading account...</h1>
                </div>
            );
        }
        else {
            return (
                <div>
                    <h1>{this.state.account.name}</h1>
                    <TableList table={['list item', 'list item', 'list item']}/>
                </div>
            );
        }
    }
}

export const AccountPage = function AccountPage() {
    let { accountId } = useParams();
    return (<Account accountId={accountId} />);
};
