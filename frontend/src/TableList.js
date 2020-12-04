function TableList(props) {
    const tables = props.tables;
    let listItems = null;
    if (tables != null) {
        listItems = tables.map((table) => 
            <li>{table}</li>
        );
    }
    else {
        listItems = <li>No Tables Found.</li>
    }
    return (
        <div>
            <h2>List of random tables</h2>
            <ul>{listItems}</ul>
        </div>
    );
}

export default TableList;
