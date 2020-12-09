import { makeStyles } from '@material-ui/core/styles';
import Table from '@material-ui/core/Table';
import TableBody from '@material-ui/core/TableBody';
import TableCell from '@material-ui/core/TableCell';
import TableContainer from '@material-ui/core/TableContainer';
import TableHead from '@material-ui/core/TableHead';
import TableRow from '@material-ui/core/TableRow';
import Paper from '@material-ui/core/Paper';

const useStyles = makeStyles({
  table: {
    minWidth: 650,
  },
});

function TableList(props) {
    const classes = useStyles();

    const tables = props.tables;
    let listItems = null;
    if (tables !== null) {
        listItems = tables.map((table) => 
            <TableRow key={table}>
                <TableCell component="th" scope="row">
                    {table}
                </TableCell>
            </TableRow>
        );
    }
    else {
        listItems = (
            <TableRow>
                <TableCell>No Tables Found.</TableCell>
            </TableRow>
        );
    }

    return (
        <TableContainer component={Paper}>
            <Table className={classes.table}>
                <TableHead>
                    <TableRow>
                        <TableCell><strong>Tables by Account</strong></TableCell>
                    </TableRow>
                </TableHead>
                <TableBody>
                    {listItems}
                </TableBody>
            </Table>
        </TableContainer>
    );
}

export default TableList;
